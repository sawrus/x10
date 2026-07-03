#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
"$ROOT_DIR/build.sh" >/dev/null

test -f "$ROOT_DIR/dist/index.html"
test -f "$ROOT_DIR/dist/app.css"
test -f "$ROOT_DIR/dist/app.js"
grep -q "Progression Dashboard" "$ROOT_DIR/dist/index.html"
grep -q "theme-select" "$ROOT_DIR/dist/app.js"

printf 'web smoke tests passed\n'
