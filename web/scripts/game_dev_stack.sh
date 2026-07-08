#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GAME_DIR="$ROOT_DIR/web/game"

backend_pid=""
frontend_pid=""

"$ROOT_DIR/web/scripts/kill_game_processes.sh"

cleanup() {
  if [[ -n "$frontend_pid" ]] && kill -0 "$frontend_pid" 2>/dev/null; then
    kill "$frontend_pid" 2>/dev/null || true
  fi

  if [[ -n "$backend_pid" ]] && kill -0 "$backend_pid" 2>/dev/null; then
    kill "$backend_pid" 2>/dev/null || true
  fi
}

trap cleanup EXIT INT TERM

(
  cd "$ROOT_DIR"
  cargo run --bin x10-backend
) &
backend_pid=$!

(
  cd "$GAME_DIR"
  npm run dev
) &
frontend_pid=$!

wait -n "$backend_pid" "$frontend_pid"
