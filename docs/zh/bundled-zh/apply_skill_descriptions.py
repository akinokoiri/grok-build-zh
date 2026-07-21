#!/usr/bin/env python3
"""Patch description / short-description fields in SKILL.md frontmatter.

Keeps skill IDs and body (model instructions) unchanged. Only rewrites the
user-visible description fields used by the skills list UI.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path


def load_map(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def split_frontmatter(text: str) -> tuple[str | None, str, str]:
    if not text.startswith("---"):
        return None, "", text
    end = text.find("\n---", 3)
    if end < 0:
        return None, "", text
    fm = text[3:end + 1]
    body = text[end + 4 :]
    return fm, text[: 3 + len(fm) + 4], body  # unused middle; keep simple


def replace_scalar_or_block(fm: str, key: str, new_value: str) -> str:
    """Replace `key:` scalar or folded block with a single-line quoted scalar."""
    lines = fm.splitlines(keepends=True)
    out: list[str] = []
    i = 0
    key_prefix = f"{key}:"
    while i < len(lines):
        line = lines[i]
        stripped = line.lstrip()
        if stripped.startswith(key_prefix) and (
            line.startswith(key_prefix) or re.match(rf"^[ \t]+{re.escape(key)}:", line)
        ):
            # Only top-level keys (no indent) or metadata nested short-description handled separately
            if line.startswith(key_prefix):
                rest = line[len(key_prefix) :].strip()
                # consume folded/literal block if present
                if rest in (">", ">-", "|", "|+", "|-", "") or rest == "":
                    i += 1
                    while i < len(lines):
                        nxt = lines[i]
                        if re.match(r"^[ \t]", nxt) or (
                            nxt.strip() == ""
                            and i + 1 < len(lines)
                            and re.match(r"^[ \t]", lines[i + 1])
                        ):
                            i += 1
                            continue
                        break
                else:
                    i += 1
                # emit as folded block for long text, scalar for short
                if len(new_value) > 80 or "\n" in new_value:
                    out.append(f"{key}: >\n")
                    # wrap ~88 cols
                    words = new_value.split()
                    row = "  "
                    for w in words:
                        if len(row) + 1 + len(w) > 90 and row.strip():
                            out.append(row.rstrip() + "\n")
                            row = "  " + w
                        else:
                            row = (row + " " + w).rstrip() if row.strip() else "  " + w
                    if row.strip():
                        out.append(row.rstrip() + "\n")
                else:
                    esc = new_value.replace("\\", "\\\\").replace('"', '\\"')
                    out.append(f'{key}: "{esc}"\n')
                continue
        out.append(line)
        i += 1
    return "".join(out)


def replace_metadata_short(fm: str, new_short: str) -> str:
    """Replace metadata.short-description if present."""
    pattern = re.compile(
        r'(^[ \t]*short-description:\s*)(?:"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|[^\n]+)',
        re.M,
    )
    esc = new_short.replace("\\", "\\\\").replace('"', '\\"')

    def repl(m: re.Match[str]) -> str:
        return f'{m.group(1)}"{esc}"'

    new_fm, n = pattern.subn(repl, fm, count=1)
    return new_fm if n else fm


def patch_skill_md(path: Path, entry: dict) -> bool:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---"):
        return False
    end = text.find("\n---", 3)
    if end < 0:
        return False
    fm = text[3:end + 1]
    body = text[end + 4 :]
    desc = entry.get("description")
    short = entry.get("short")
    if desc:
        fm = replace_scalar_or_block(fm, "description", desc)
    if short:
        # top-level short-description (rare) + metadata.short-description
        if re.search(r"^short-description:", fm, re.M):
            fm = replace_scalar_or_block(fm, "short-description", short)
        fm = replace_metadata_short(fm, short)
    path.write_text("---" + fm + "---" + body, encoding="utf-8")
    return True


def main() -> int:
    here = Path(__file__).resolve().parent
    desc_map = load_map(here / "skills" / "descriptions.json")
    grok_home = Path(sys.argv[1] if len(sys.argv) > 1 else Path.home() / ".grok")
    targets = [
        grok_home / "bundled" / "skills",
        grok_home / "skills",
    ]
    patched = 0
    skipped = 0
    for root in targets:
        if not root.is_dir():
            continue
        for skill_dir in sorted(root.iterdir()):
            skill_md = skill_dir / "SKILL.md"
            if not skill_md.is_file():
                continue
            sid = skill_dir.name
            entry = desc_map.get(sid)
            if not entry:
                skipped += 1
                continue
            if patch_skill_md(skill_md, entry):
                print(f"  ok  {skill_md}")
                patched += 1
            else:
                print(f"  skip {skill_md} (no frontmatter)")
                skipped += 1
    print(f"技能 description 已更新: {patched} 个；未映射跳过: {skipped}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
