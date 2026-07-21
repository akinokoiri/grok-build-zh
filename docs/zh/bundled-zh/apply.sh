#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
GROK_HOME="${GROK_HOME:-$HOME/.grok}"
DEST="$GROK_HOME/bundled"
mkdir -p "$DEST/personas" "$DEST/roles" "$DEST/agents"
cp -f "$ROOT/personas/"*.toml "$DEST/personas/" 2>/dev/null || true
cp -f "$ROOT/roles/"*.toml "$DEST/roles/" 2>/dev/null || true
cp -f "$ROOT/agents/"*.md "$DEST/agents/" 2>/dev/null || true
echo "已应用人设/角色/代理中文 description → $DEST"

# 技能：只改 SKILL.md 的 description / short-description（ID 与正文指令不动）
if command -v python3 >/dev/null 2>&1; then
  echo "正在应用技能中文 description…"
  python3 "$ROOT/apply_skill_descriptions.py" "$GROK_HOME"
else
  echo "警告: 未找到 python3，跳过技能 description 汉化" >&2
fi

echo "完成。重启 grok-zh 后在 /skills 列表中可见中文描述。"
