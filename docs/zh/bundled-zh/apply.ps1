<#
.SYNOPSIS
    单独应用 Grok Build 内置人设、角色与技能的中文 description（Windows PowerShell 版）
#>

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$GrokHome = if ($env:GROK_HOME) { $env:GROK_HOME } else { "$HOME\.grok" }
$Dest = Join-Path $GrokHome "bundled"

Write-Host "正在应用人设/角色/代理中文描述 -> $Dest" -ForegroundColor Cyan

$Dirs = @("personas", "roles", "agents")
foreach ($d in $Dirs) {
    $Src = Join-Path $ScriptDir $d
    $Target = Join-Path $Dest $d
    if (Test-Path $Src) {
        if (-not (Test-Path $Target)) { New-Item -ItemType Directory -Path $Target -Force | Out-Null }
        Copy-Item -Path "$Src\*" -Destination $Target -Recurse -Force
    }
}

$PyScript = Join-Path $ScriptDir "apply_skill_descriptions.py"
$PythonCmd = Get-Command python, python3 -ErrorAction SilentlyContinue | Select-Object -First 1
if ($PythonCmd -and (Test-Path $PyScript)) {
    Write-Host "正在应用技能中文 description..." -ForegroundColor Cyan
    & $PythonCmd.Source $PyScript $GrokHome
} else {
    Write-Host "提示: 未找到 python，跳过技能 description 汉化" -ForegroundColor Yellow
}

Write-Host "完成。重启 grok-zh 后在 /skills 列表中可见中文描述。" -ForegroundColor Green
