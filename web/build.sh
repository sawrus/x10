#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GAME_DIR="$ROOT_DIR/game"

install_dependencies() {
  local app_dir="$1"

  cd "$app_dir"
  if [ -f package-lock.json ]; then
    npm ci --no-audit --no-fund
  else
    npm install --no-audit --no-fund
  fi
}

install_dependencies "$GAME_DIR"
npm run --prefix "$GAME_DIR" build

install_dependencies "$ROOT_DIR"
npm run build
mkdir -p "$ROOT_DIR/dist/game"
cp -R "$GAME_DIR/dist/." "$ROOT_DIR/dist/game/"

printf 'web build ready in %s/dist\n' "$ROOT_DIR"
