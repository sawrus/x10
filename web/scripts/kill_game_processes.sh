#!/usr/bin/env bash
set -euo pipefail

readonly ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

process_lines() {
  local pattern="$1"
  ps -eo pid=,ppid=,stat=,args= | grep -E "$pattern" | grep -v grep || true
}

kill_port_listeners() {
  local label="$1"
  local port="$2"
  local pids

  pids="$(lsof -tiTCP:"$port" -sTCP:LISTEN 2>/dev/null || true)"
  if [[ -z "$pids" ]]; then
    return 0
  fi

  printf 'Stopping %s listener(s) on port %s: %s\n' "$label" "$port" "$(echo "$pids" | tr '\n' ' ')"
  while IFS= read -r pid; do
    [[ -n "$pid" ]] || continue
    kill "$pid" 2>/dev/null || true
  done <<<"$pids"

  for _ in 1 2 3 4 5; do
    sleep 0.2
    pids="$(lsof -tiTCP:"$port" -sTCP:LISTEN 2>/dev/null || true)"
    if [[ -z "$pids" ]]; then
      return 0
    fi
  done

  printf 'Force stopping stubborn %s listener(s) on port %s: %s\n' "$label" "$port" "$(echo "$pids" | tr '\n' ' ')"
  while IFS= read -r pid; do
    [[ -n "$pid" ]] || continue
    kill -9 "$pid" 2>/dev/null || true
  done <<<"$pids"
}

kill_matching_processes() {
  local label="$1"
  local pattern="$2"
  local lines
  local live_pids=""
  local zombie_notes=""

  lines="$(process_lines "$pattern")"
  if [[ -z "$lines" ]]; then
    return 0
  fi

  while IFS= read -r line; do
    [[ -n "$line" ]] || continue
    local pid
    local ppid
    local stat
    pid="$(awk '{print $1}' <<<"$line")"
    ppid="$(awk '{print $2}' <<<"$line")"
    stat="$(awk '{print $3}' <<<"$line")"

    if [[ "$stat" == Z* ]]; then
      zombie_notes+="$pid(parent:$ppid) "
      continue
    fi

    live_pids+="$pid"$'\n'
  done <<<"$lines"

  if [[ -n "$zombie_notes" ]]; then
    printf 'Detected %s zombie process(es): %s\n' "$label" "$zombie_notes"
    printf 'These will disappear after their parent shell reaps them.\n'
  fi

  if [[ -z "$live_pids" ]]; then
    return 0
  fi

  printf 'Stopping stale %s process(es): %s\n' "$label" "$(echo "$live_pids" | tr '\n' ' ')"
  while IFS= read -r pid; do
    [[ -n "$pid" ]] || continue
    kill "$pid" 2>/dev/null || true
  done <<<"$live_pids"

  for _ in 1 2 3 4 5; do
    sleep 0.2
    lines="$(process_lines "$pattern")"
    if [[ -z "$lines" ]]; then
      return 0
    fi

    live_pids=""
    while IFS= read -r line; do
      [[ -n "$line" ]] || continue
      if [[ "$(awk '{print $3}' <<<"$line")" != Z* ]]; then
        live_pids+="$(
          awk '{print $1}' <<<"$line"
        )"$'\n'
      fi
    done <<<"$lines"

    if [[ -z "$live_pids" ]]; then
      return 0
    fi
  done

  if [[ -n "$live_pids" ]]; then
    printf 'Force stopping stubborn %s process(es): %s\n' "$label" "$(echo "$live_pids" | tr '\n' ' ')"
    while IFS= read -r pid; do
      [[ -n "$pid" ]] || continue
      kill -9 "$pid" 2>/dev/null || true
    done <<<"$live_pids"
  fi
}

kill_matching_processes "backend" "$ROOT_DIR/target/debug/x10-backend"
kill_matching_processes "backend" "(^|[[:space:]])target/debug/x10-backend($|[[:space:]])"
kill_matching_processes "backend runner" "cargo run --bin x10-backend"
kill_matching_processes "game vite" "$ROOT_DIR/web/game/node_modules/.bin/vite"
kill_matching_processes "game vite" "(^|[[:space:]])vite( |$)"
kill_matching_processes "game esbuild" "$ROOT_DIR/web/game/node_modules/@esbuild/.*/bin/esbuild --service="
kill_port_listeners "backend" 3000
kill_port_listeners "game vite" 5173
