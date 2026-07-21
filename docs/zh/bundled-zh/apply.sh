#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
DEST="${GROK_HOME:-$HOME/.grok}/bundled"
mkdir -p "$DEST/personas" "$DEST/roles" "$DEST/agents"
cp -f "$ROOT/personas/"*.toml "$DEST/personas/" 2>/dev/null || true
cp -f "$ROOT/roles/"*.toml "$DEST/roles/" 2>/dev/null || true
cp -f "$ROOT/agents/"*.md "$DEST/agents/" 2>/dev/null || true
echo "已应用中文 description 到 $DEST"
