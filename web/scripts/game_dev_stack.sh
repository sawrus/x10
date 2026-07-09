#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GAME_DIR="$ROOT_DIR/web/game"
DEV_ADMIN_USERNAME="admin"
DEV_ADMIN_PASSWORD_HASH='$argon2id$v=19$m=19456,t=2,p=1$ZfmqeR+rQgaHE96FVRGIvQ$fGXssSG70hSSBbuc01AbAPZU/RFok1+JCAWovmZ38Yo'
DEV_ADMIN_SESSION_SECRET="x10-game-stack-dev-secret"

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
  export X10_ADMIN_USERNAME="${X10_ADMIN_USERNAME:-$DEV_ADMIN_USERNAME}"
  export X10_ADMIN_PASSWORD_HASH="${X10_ADMIN_PASSWORD_HASH:-$DEV_ADMIN_PASSWORD_HASH}"
  export X10_ADMIN_SESSION_SECRET="${X10_ADMIN_SESSION_SECRET:-$DEV_ADMIN_SESSION_SECRET}"
  export X10_ADMIN_SESSION_SECURE="${X10_ADMIN_SESSION_SECURE:-false}"
  cargo run --bin x10-backend
) &
backend_pid=$!

(
  cd "$GAME_DIR"
  npm run dev
) &
frontend_pid=$!

wait -n "$backend_pid" "$frontend_pid"
