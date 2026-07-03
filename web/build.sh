#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_DIR="$ROOT_DIR/src"
DIST_DIR="$ROOT_DIR/dist"

mkdir -p "$DIST_DIR"
cp "$SRC_DIR/index.html" "$DIST_DIR/index.html"
cp "$SRC_DIR/app.css" "$DIST_DIR/app.css"
cp "$SRC_DIR/app.js" "$DIST_DIR/app.js"

printf 'web build ready in %s\n' "$DIST_DIR"
