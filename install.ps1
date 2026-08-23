<#
.SYNOPSIS
    Grok Build 社区汉化版 Windows 11 一键安装 / 更新脚本
.DESCRIPTION
    自动从 GitHub Releases 下载最新的 Windows 预编译版 grok-zh.exe，
    部署到用户目录并自动配置环境变量与内置汉化技能。
.EXAMPLE
    irm https://raw.githubusercontent.com/akinokoiri/grok-build-zh/zh-CN/install.ps1 | iex
#>

[CmdletBinding()]
param(
    [string]$Tag = "",
    [string]$Repo = "akinokoiri/grok-build-zh",
    [string]$InstallDir = "$HOME\.grok\bin",
    [switch]$SkipBundledZh
)

$ErrorActionPreference = "Stop"

function Write-Info([string]$msg) {
    Write-Host "[信息] " -ForegroundColor Cyan -NoNewline
    Write-Host $msg
}

function Write-Success([string]$msg) {
    Write-Host "[成功] " -ForegroundColor Green -NoNewline
    Write-Host $msg
}

function Write-Warn([string]$msg) {
    Write-Host "[提示] " -ForegroundColor Yellow -NoNewline
    Write-Host $msg
}

function Write-Err([string]$msg) {
    Write-Host "[错误] " -ForegroundColor Red -NoNewline
    Write-Host $msg
}

Write-Host ""
Write-Host "=================================================" -ForegroundColor Cyan
Write-Host "      Grok Build 社区汉化版 (Windows 11 安装程序)     " -ForegroundColor White -BackgroundColor DarkBlue
Write-Host "=================================================" -ForegroundColor Cyan
Write-Host ""

# 1. 确定目标 Release 版本
$Headers = @{
    "Accept" = "application/vnd.github+json"
    "User-Agent" = "Grok-ZH-Installer-PowerShell"
}

Write-Info "正在获取 GitHub Release 版本信息 ($Repo)..."
try {
    if ([string]::IsNullOrWhiteSpace($Tag)) {
        $ReleaseUrl = "https://api.github.com/repos/$Repo/releases/latest"
        $Release = Invoke-RestMethod -Uri $ReleaseUrl -Headers $Headers -Method Get
    } else {
        $ReleaseUrl = "https://api.github.com/repos/$Repo/releases/tags/$Tag"
        $Release = Invoke-RestMethod -Uri $ReleaseUrl -Headers $Headers -Method Get
    }
} catch {
    # 如果 latest 404 (比如只有 prerelease)，获取所有 releases 中的第一个
    try {
        $AllReleasesUrl = "https://api.github.com/repos/$Repo/releases"
        $AllReleases = Invoke-RestMethod -Uri $AllReleasesUrl -Headers $Headers -Method Get
        if ($AllReleases -and $AllReleases.Count -gt 0) {
            $Release = $AllReleases[0]
        } else {
            throw "未在仓库 $Repo 找到任何发布版本"
        }
    } catch {
        Write-Err "获取 Release 失败: $_"
        Write-Warn "请确认仓库已完成 Release 工作流构建发布。"
        exit 1
    }
}

$TagName = $Release.tag_name
Write-Info "目标版本: $TagName"

# 2. 匹配 Windows 资产包
$ZipAsset = $Release.assets | Where-Object { $_.name -like "*windows*.zip" -or $_.name -like "*msvc*.zip" } | Select-Object -First 1
$ShaAsset = $Release.assets | Where-Object { $_.name -eq "$($ZipAsset.name).sha256" } | Select-Object -First 1

if (-not $ZipAsset) {
    Write-Err "在版本 $TagName 中未找到适用于 Windows 的 .zip 预编译包！"
    Write-Warn "可能原因：GitHub Actions 编译尚未完成，或 Release 中缺少 Windows 构建产物。"
    exit 1
}

$TempDir = Join-Path ([System.IO.Path]::GetTempPath()) ("grok-zh-install-" + [System.Guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Path $TempDir -Force | Out-Null
$ZipPath = Join-Path $TempDir $ZipAsset.name

try {
    # 3. 下载资产包
    Write-Info "正在下载 $($ZipAsset.name) ($([math]::Round($ZipAsset.size / 1MB, 2)) MB)..."
    Invoke-WebRequest -Uri $ZipAsset.browser_download_url -OutFile $ZipPath -UseBasicParsing

    # 校验和验证（如果存在 sha256 文件）
    if ($ShaAsset) {
        Write-Info "正在校验 SHA256 完整性..."
        $ExpectedSha = (Invoke-RestMethod -Uri $ShaAsset.browser_download_url -UseBasicParsing).ToString().Trim().Split(" ")[0]
        $ActualSha = (Get-FileHash -Path $ZipPath -Algorithm SHA256).Hash.ToLower()
        if ($ExpectedSha.ToLower() -ne $ActualSha) {
            Write-Err "SHA256 校验失败！期望值: $ExpectedSha, 实际值: $ActualSha"
            exit 1
        }
        Write-Success "SHA256 校验通过。"
    }

    # 4. 解压资产包
    Write-Info "正在解压资产包..."
    $ExtractDir = Join-Path $TempDir "extracted"
    Expand-Archive -Path $ZipPath -DestinationPath $ExtractDir -Force

    # 寻找解压出的 grok-zh.exe
    $BinFile = Get-ChildItem -Path $ExtractDir -Filter "grok-zh.exe" -Recurse | Select-Object -First 1
    if (-not $BinFile) {
        Write-Err "解压目录中未找到 grok-zh.exe"
        exit 1
    }

    # 5. 部署到目标目录
    if (-not (Test-Path -Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    $TargetExe = Join-Path $InstallDir "grok-zh.exe"
    Copy-Item -Path $BinFile.FullName -Destination $TargetExe -Force
    Write-Success "二进制已部署至: $TargetExe"

    # 6. 配置环境变量 PATH
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $PathParts = $UserPath -split ";" | Where-Object { $_ -ne "" }
    if ($PathParts -notcontains $InstallDir -and $PathParts -notcontains "$InstallDir\") {
        Write-Info "正在将 $InstallDir 添加到用户环境变量 PATH..."
        $NewUserPath = "$UserPath;$InstallDir"
        [Environment]::SetEnvironmentVariable("Path", $NewUserPath, "User")
        $env:Path = "$env:Path;$InstallDir"
        Write-Success "PATH 环境变量已成功添加！"
    } else {
        Write-Info "PATH 环境变量已包含 $InstallDir，无需重复添加。"
    }

    # 7. 应用内置汉化技能与人设（bundled-zh）
    if (-not $SkipBundledZh) {
        $BundledZhDir = Get-ChildItem -Path $ExtractDir -Filter "bundled-zh" -Directory -Recurse | Select-Object -First 1
        if ($BundledZhDir) {
            Write-Info "正在部署中文人设、角色与技能描述..."
            $GrokHome = Split-Path $InstallDir -Parent
            $DestBundled = Join-Path $GrokHome "bundled"
            
            # 复制 personas / roles / agents
            $DirsToCopy = @("personas", "roles", "agents")
            foreach ($d in $DirsToCopy) {
                $SrcSub = Join-Path $BundledZhDir.FullName $d
                $DestSub = Join-Path $DestBundled $d
                if (Test-Path $SrcSub) {
                    if (-not (Test-Path $DestSub)) { New-Item -ItemType Directory -Path $DestSub -Force | Out-Null }
                    Copy-Item -Path "$SrcSub\*" -Destination $DestSub -Recurse -Force
                }
            }

            # 如果存在 Python，运行技能描述补丁
            $PyScript = Join-Path $BundledZhDir.FullName "apply_skill_descriptions.py"
            if (Test-Path $PyScript) {
                $PythonCmd = Get-Command python, python3 -ErrorAction SilentlyContinue | Select-Object -First 1
                if ($PythonCmd) {
                    try {
                        & $PythonCmd.Source $PyScript $GrokHome | Out-Null
                        Write-Success "已成功应用内置技能中文 description。"
                    } catch {
                        Write-Warn "运行技能描述汉化脚本时出现小问题，跳过: $_"
                    }
                }
            }
            Write-Success "中文内置资源已部署至: $DestBundled"
        }
    }

    Write-Host ""
    Write-Host "=================================================" -ForegroundColor Green
    Write-Host "           Grok Build 社区汉化版 安装成功！          " -ForegroundColor White -BackgroundColor DarkGreen
    Write-Host "=================================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "如何启动汉化版：" -ForegroundColor Yellow
    Write-Host "  在任意终端直接输入: " -NoNewline
    Write-Host "grok-zh" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "与官方英文版并存：" -ForegroundColor Yellow
    Write-Host "  中文汉化版: " -NoNewline
    Write-Host "grok-zh" -ForegroundColor Cyan
    Write-Host "  官方英文版: " -NoNewline
    Write-Host "grok" -ForegroundColor Cyan
    Write-Host ""

} finally {
    # 清理临时文件
    if (Test-Path -Path $TempDir) {
        Remove-Item -Path $TempDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}
