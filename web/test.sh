#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

"$ROOT_DIR/build.sh" >/dev/null

test -f "$ROOT_DIR/dist/index.html"
test -f "$ROOT_DIR/dist/game/index.html"
find "$ROOT_DIR/dist/assets" -type f | grep -q '\.js$'
find "$ROOT_DIR/dist/assets" -type f | grep -q '\.css$'
grep -q "x10 admin" "$ROOT_DIR/dist/index.html"

printf 'web smoke tests passed\n'
