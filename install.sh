#!/usr/bin/env bash
#
# Grok Build 汉化版（grok-build-zh）一键安装脚本
# 社区维护 · 非官方 · 与 SpaceXAI / xAI 无隶属关系
#
# 用法（推荐）:
#   curl -fsSL https://raw.githubusercontent.com/ivan6232/grok-build-zh/zh-CN/install.sh | bash
#
# 指定版本标签:
#   curl -fsSL https://raw.githubusercontent.com/ivan6232/grok-build-zh/zh-CN/install.sh | bash -s -- v0.1.0-zh.4
#
# 环境变量:
#   GROK_ZH_BIN_DIR   安装目录（默认: ~/.local/bin）
#   GROK_ZH_REPO      GitHub 仓库（默认: ivan6232/grok-build-zh）
#   GROK_ZH_VERSION   版本标签，例如 v0.1.0-zh.4（也可用脚本第一个参数）
#   GROK_ZH_FROM_SOURCE=1  强制从源码编译
#   GROK_ZH_NO_PATH=1      不修改 shell 配置
#
set -euo pipefail

REPO="${GROK_ZH_REPO:-ivan6232/grok-build-zh}"
BIN_NAME="grok-zh"
INSTALL_DIR="${GROK_ZH_BIN_DIR:-$HOME/.local/bin}"
VERSION="${1:-${GROK_ZH_VERSION:-}}"
FROM_SOURCE="${GROK_ZH_FROM_SOURCE:-0}"
NO_PATH="${GROK_ZH_NO_PATH:-0}"
GITHUB_API="${GITHUB_API:-https://api.github.com}"
GITHUB_RAW="${GITHUB_RAW:-https://raw.githubusercontent.com}"

# colors if tty
if [[ -t 1 ]]; then
  C_GREEN=$'\033[32m'; C_YELLOW=$'\033[33m'; C_RED=$'\033[31m'; C_BOLD=$'\033[1m'; C_RESET=$'\033[0m'
else
  C_GREEN=""; C_YELLOW=""; C_RED=""; C_BOLD=""; C_RESET=""
fi

info()  { printf '%s==>%s %s\n' "$C_BOLD" "$C_RESET" "$*"; }
ok()    { printf '%s✓%s %s\n' "$C_GREEN" "$C_RESET" "$*"; }
warn()  { printf '%s!%s %s\n' "$C_YELLOW" "$C_RESET" "$*" >&2; }
die()   { printf '%serror:%s %s\n' "$C_RED" "$C_RESET" "$*" >&2; exit 1; }

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "需要命令: $1"
}

download() {
  local url="$1" out="${2:-}"
  if command -v curl >/dev/null 2>&1; then
    if [[ -n "$out" ]]; then
      curl -fsSL --retry 3 --retry-delay 1 -o "$out" "$url"
    else
      curl -fsSL --retry 3 --retry-delay 1 "$url"
    fi
  elif command -v wget >/dev/null 2>&1; then
    if [[ -n "$out" ]]; then
      wget -q -O "$out" "$url"
    else
      wget -q -O - "$url"
    fi
  else
    die "需要 curl 或 wget"
  fi
}

detect_platform() {
  local os arch
  os="$(uname -s | tr '[:upper:]' '[:lower:]')"
  arch="$(uname -m)"

  case "$os" in
    darwin) os="apple-darwin" ;;
    linux)  os="unknown-linux-gnu" ;;
    msys*|mingw*|cygwin*)
      die "Windows 暂不提供预编译包。请在 WSL2 中安装，或从源码编译。"
      ;;
    *) die "不支持的操作系统: $(uname -s)" ;;
  esac

  case "$arch" in
    x86_64|amd64) arch="x86_64" ;;
    arm64|aarch64) arch="aarch64" ;;
    *) die "不支持的 CPU 架构: $(uname -m)" ;;
  esac

  # Linux aarch64: 若无预编译包会回退源码
  PLATFORM="${arch}-${os}"
  ASSET_STEM="grok-zh-${PLATFORM}"
  ASSET_NAME="${ASSET_STEM}.tar.gz"
}

resolve_version() {
  if [[ -n "$VERSION" ]]; then
    # accept with or without leading v
    [[ "$VERSION" == v* ]] || VERSION="v${VERSION}"
    return
  fi
  info "查询最新 Release…"
  local json
  # prefer latest release (incl. prerelease via releases list)
  if ! json="$(download "${GITHUB_API}/repos/${REPO}/releases?per_page=20" 2>/dev/null)"; then
    warn "无法访问 GitHub API，将尝试从源码安装"
    VERSION=""
    return
  fi
  # pick first non-draft release that has our asset pattern, else first non-draft
  VERSION="$(
    printf '%s' "$json" | python3 -c '
import json,sys
try:
    data=json.load(sys.stdin)
except Exception:
    sys.exit(0)
for r in data:
    if r.get("draft"): continue
    assets=[a.get("name","") for a in r.get("assets") or []]
    if any(n.startswith("grok-zh-") and n.endswith(".tar.gz") for n in assets):
        print(r.get("tag_name",""))
        break
else:
    for r in data:
        if not r.get("draft"):
            print(r.get("tag_name",""))
            break
' 2>/dev/null || true
  )"
  if [[ -z "$VERSION" ]]; then
    warn "仓库尚无可用 Release，将尝试从源码编译"
  else
    ok "版本: $VERSION"
  fi
}

asset_url_for() {
  local tag="$1" name="$2"
  local json url
  json="$(download "${GITHUB_API}/repos/${REPO}/releases/tags/${tag}")" || return 1
  url="$(
    printf '%s' "$json" | python3 -c '
import json,sys
name=sys.argv[1]
data=json.load(sys.stdin)
for a in data.get("assets") or []:
    if a.get("name")==name:
        print(a.get("browser_download_url",""))
        break
' "$name" 2>/dev/null || true
  )"
  [[ -n "$url" ]] || return 1
  printf '%s' "$url"
}

install_path_line() {
  local line='export PATH="$HOME/.local/bin:$PATH"'
  if [[ "$INSTALL_DIR" != "$HOME/.local/bin" ]]; then
    line="export PATH=\"${INSTALL_DIR}:\$PATH\""
  fi
  printf '%s' "$line"
}

ensure_path() {
  [[ "$NO_PATH" == "1" ]] && return 0
  local path_line rc marker
  path_line="$(install_path_line)"
  marker="# >>> grok-zh 汉化版 >>>"

  # Always export for current process
  export PATH="${INSTALL_DIR}:$PATH"

  # Already on PATH for future shells?
  case ":$PATH:" in
    *":${INSTALL_DIR}:"*) ;;
  esac

  if [[ -n "${ZSH_VERSION:-}" ]] || [[ "$(basename "${SHELL:-}")" == "zsh" ]]; then
    rc="$HOME/.zshrc"
  elif [[ -n "${BASH_VERSION:-}" ]] || [[ "$(basename "${SHELL:-}")" == "bash" ]]; then
    if [[ -f "$HOME/.bashrc" ]]; then
      rc="$HOME/.bashrc"
    else
      rc="$HOME/.bash_profile"
    fi
  else
    rc="$HOME/.profile"
  fi

  if [[ -f "$rc" ]] && grep -qF "$marker" "$rc" 2>/dev/null; then
    ok "PATH 配置已存在: $rc"
    return 0
  fi
  if [[ -f "$rc" ]] && grep -qF "$INSTALL_DIR" "$rc" 2>/dev/null; then
    ok "PATH 中已包含 ${INSTALL_DIR}（$rc）"
    return 0
  fi

  {
    echo ""
    echo "$marker"
    echo "# 社区汉化 Grok Build：命令 grok-zh（官方原版仍是 grok）"
    echo "$path_line"
    echo "# <<< grok-zh 汉化版 <<<"
  } >> "$rc"
  ok "已写入 PATH 到 $rc"
  warn "新开终端后生效；当前会话可执行: export PATH=\"${INSTALL_DIR}:\$PATH\""
}

verify_binary() {
  local bin="$1"
  [[ -x "$bin" ]] || die "安装后的二进制不可执行: $bin"
  if ! "$bin" --version >/dev/null 2>&1; then
    # some builds may need dyld — still check file type
    warn "无法运行 --version，请检查依赖库"
  else
    ok "$("$bin" --version 2>/dev/null | head -1)"
  fi
}

install_from_tarball_url() {
  local url="$1"
  local tmp found
  tmp="$(mktemp -d "${TMPDIR:-/tmp}/grok-zh-install.XXXXXX")"
  # Expand path now so RETURN cleanup works under `set -u`
  # shellcheck disable=SC2064
  trap "rm -rf '$tmp'" RETURN

  info "下载: $url"
  download "$url" "$tmp/pkg.tar.gz"

  info "解压…"
  mkdir -p "$tmp/extract"
  tar -xzf "$tmp/pkg.tar.gz" -C "$tmp/extract"

  # find grok-zh binary (tarball may nest one directory)
  found="$(find "$tmp/extract" -type f -name "$BIN_NAME" | head -1 || true)"
  [[ -n "$found" ]] || die "压缩包内未找到 $BIN_NAME"

  mkdir -p "$INSTALL_DIR"
  # atomic-ish replace
  cp "$found" "$INSTALL_DIR/${BIN_NAME}.new"
  chmod +x "$INSTALL_DIR/${BIN_NAME}.new"
  mv -f "$INSTALL_DIR/${BIN_NAME}.new" "$INSTALL_DIR/${BIN_NAME}"
  ok "已安装: $INSTALL_DIR/$BIN_NAME"
}

install_from_release() {
  [[ -n "$VERSION" ]] || return 1
  local url
  info "寻找资产: ${ASSET_NAME} @ ${VERSION}"
  if ! url="$(asset_url_for "$VERSION" "$ASSET_NAME")"; then
    warn "Release ${VERSION} 没有 ${ASSET_NAME}"
    return 1
  fi
  install_from_tarball_url "$url"
}

install_from_source() {
  need_cmd git
  need_cmd cargo || true

  if ! command -v cargo >/dev/null 2>&1; then
    die "未找到 cargo。请先安装 Rust: https://rustup.rs （或设置镜像后安装 rustup）"
  fi
  if ! command -v rustc >/dev/null 2>&1; then
    die "未找到 rustc。请先安装 Rust toolchain。"
  fi

  info "从源码编译（可能需要 10–40 分钟）…"
  local src
  src="$(mktemp -d "${TMPDIR:-/tmp}/grok-zh-src.XXXXXX")"
  # shellcheck disable=SC2064
  trap "rm -rf '$src'" EXIT

  git clone --depth 1 --branch zh-CN "https://github.com/${REPO}.git" "$src"
  cd "$src"

  if ! command -v dotslash >/dev/null 2>&1; then
    info "安装 DotSlash…"
    cargo install dotslash --locked || cargo install dotslash || true
  fi

  if ! command -v protoc >/dev/null 2>&1; then
    if [[ -x "$src/bin/protoc" ]]; then
      export PATH="$src/bin:$PATH"
    elif command -v brew >/dev/null 2>&1; then
      info "通过 Homebrew 安装 protobuf…"
      brew install protobuf || true
    else
      warn "未检测到 protoc，构建可能会失败。请安装 protobuf-compiler。"
    fi
  fi
  if command -v protoc >/dev/null 2>&1; then
    export PROTOC="$(command -v protoc)"
  fi

  cargo build -p xai-grok-pager-bin --release

  local bin="target/release/xai-grok-pager"
  [[ -f "$bin" ]] || die "编译完成但未找到 $bin"

  mkdir -p "$INSTALL_DIR"
  cp "$bin" "$INSTALL_DIR/$BIN_NAME"
  chmod +x "$INSTALL_DIR/$BIN_NAME"
  ok "源码编译并安装: $INSTALL_DIR/$BIN_NAME"
}

print_banner() {
  cat <<EOF
${C_BOLD}Grok Build 汉化版安装程序${C_RESET}
  仓库: https://github.com/${REPO}
  命令: ${BIN_NAME}
  说明: 社区非官方；官方英文版请用: curl -fsSL https://x.ai/cli/install.sh | bash

EOF
}

print_done() {
  cat <<EOF

${C_GREEN}${C_BOLD}安装完成${C_RESET}

  启动汉化界面:
    ${BIN_NAME}

  若提示 command not found:
    export PATH="${INSTALL_DIR}:\$PATH"
    # 或重新打开终端

  与官方并存:
    grok      → 官方英文版（若已安装）
    grok-zh   → 本汉化版

  文档: https://github.com/${REPO}#安装与使用

EOF
}

main() {
  print_banner
  detect_platform
  info "平台: ${PLATFORM}"
  info "安装到: ${INSTALL_DIR}/${BIN_NAME}"

  if [[ "$FROM_SOURCE" == "1" ]]; then
    install_from_source
  else
    resolve_version
    if ! install_from_release; then
      warn "预编译包不可用，回退到源码编译…"
      install_from_source
    fi
  fi

  ensure_path
  export PATH="${INSTALL_DIR}:$PATH"
  verify_binary "${INSTALL_DIR}/${BIN_NAME}"
  print_done
}

main "$@"
