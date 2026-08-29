[CmdletBinding()]
param(
    [string]$Version,
    [switch]$Check,
    [switch]$Json,
    [switch]$Force,
    [int]$WaitForProcessId = 0,
    [string]$InstallDir = (Join-Path ([Environment]::GetFolderPath("UserProfile")) ".grok\bin")
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$Repo = "akinokoiri/grok-build-zh"
$AssetName = "grok-zh-x86_64-pc-windows-msvc.zip"
$ApiHeaders = @{
    Accept = "application/vnd.github+json"
    "User-Agent" = "grok-build-zh-installer"
    "X-GitHub-Api-Version" = "2022-11-28"
}

if ($Version -and $Version -notmatch '^\d+\.\d+\.\d+-zh\.\d+$') {
    throw "版本格式无效：$Version（应类似 0.2.106-zh.1）"
}

$ReleaseUri = if ($Version) {
    "https://api.github.com/repos/$Repo/releases/tags/v$Version"
} else {
    "https://api.github.com/repos/$Repo/releases/latest"
}
try {
    $Release = Invoke-RestMethod -Uri $ReleaseUri -Headers $ApiHeaders
} catch {
    if ($Version) {
        throw
    }
    # GitHub's /latest endpoint excludes prereleases. During migration the
    # repository may only contain older prereleases, so use the newest
    # non-draft release until the first stable personal build is published.
    $Releases = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases?per_page=20" -Headers $ApiHeaders
    $Release = $Releases | Where-Object { -not $_.draft } | Select-Object -First 1
    if (-not $Release) {
        throw "仓库 $Repo 尚无可安装的 Release"
    }
}
$LatestVersion = ([string]$Release.tag_name).TrimStart("v")

$CurrentVersion = $null
$CurrentExe = Join-Path $InstallDir "grok-zh.exe"
if (Test-Path -LiteralPath $CurrentExe) {
    try {
        $VersionOutput = & $CurrentExe version 2>$null | Out-String
        if ($VersionOutput -match '(\d+\.\d+\.\d+-zh\.\d+)') {
            $CurrentVersion = $Matches[1]
        }
    } catch {
        # A damaged old installation should not prevent a repair install.
    }
}

$UpdateAvailable = $Force -or ($CurrentVersion -ne $LatestVersion)
if ($Check) {
    $Status = [ordered]@{
        currentVersion = $CurrentVersion
        latestVersion = $LatestVersion
        updateAvailable = [bool]$UpdateAvailable
        source = "github:$Repo"
    }
    if ($Json) {
        $Status | ConvertTo-Json -Compress
    } else {
        if ($UpdateAvailable) {
            $CurrentDisplay = if ($CurrentVersion) { $CurrentVersion } else { "未安装" }
            Write-Host "有可用更新：$CurrentDisplay -> $LatestVersion"
        } else {
            Write-Host "已是最新版本：$LatestVersion"
        }
    }
    exit 0
}

if (-not $UpdateAvailable) {
    Write-Host "已是最新版本：$LatestVersion"
    exit 0
}

$ZipAsset = $Release.assets | Where-Object { $_.name -eq $AssetName } | Select-Object -First 1
$HashAsset = $Release.assets | Where-Object { $_.name -eq "$AssetName.sha256" } | Select-Object -First 1
if (-not $ZipAsset -or -not $HashAsset) {
    throw "Release v$LatestVersion 缺少 Windows x64 安装包或校验文件"
}

$TempRoot = Join-Path ([IO.Path]::GetTempPath()) ("grok-zh-install-" + [Guid]::NewGuid().ToString("N"))
$ZipPath = Join-Path $TempRoot $AssetName
$HashPath = "$ZipPath.sha256"
$ExtractPath = Join-Path $TempRoot "extract"
New-Item -ItemType Directory -Path $ExtractPath -Force | Out-Null

try {
    Invoke-WebRequest -UseBasicParsing -Uri $ZipAsset.browser_download_url -OutFile $ZipPath
    Invoke-WebRequest -UseBasicParsing -Uri $HashAsset.browser_download_url -OutFile $HashPath

    $ExpectedHash = ((Get-Content -LiteralPath $HashPath -Raw).Trim() -split '\s+')[0]
    $ActualHash = (Get-FileHash -LiteralPath $ZipPath -Algorithm SHA256).Hash
    if ($ExpectedHash -ne $ActualHash) {
        throw "SHA-256 校验失败；安装包未被写入"
    }

    Expand-Archive -LiteralPath $ZipPath -DestinationPath $ExtractPath -Force
    $NewExe = Join-Path $ExtractPath "grok-zh.exe"
    $NewCatalog = Join-Path $ExtractPath "zh-CN.json"
    $NewSchema = Join-Path $ExtractPath "schema.json"
    $NewInstaller = Join-Path $ExtractPath "install.ps1"
    if (-not (Test-Path -LiteralPath $NewExe)) {
        throw "安装包中没有 grok-zh.exe"
    }

    if ($WaitForProcessId -gt 0) {
        $Deadline = [DateTime]::UtcNow.AddSeconds(60)
        while ([DateTime]::UtcNow -lt $Deadline) {
            if (-not (Get-Process -Id $WaitForProcessId -ErrorAction SilentlyContinue)) {
                break
            }
            Start-Sleep -Milliseconds 250
        }
    }

    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    $I18nDir = Join-Path (Split-Path $InstallDir -Parent) "i18n"
    New-Item -ItemType Directory -Path $I18nDir -Force | Out-Null

    $BackupExe = "$CurrentExe.previous"
    if (Test-Path -LiteralPath $CurrentExe) {
        Copy-Item -LiteralPath $CurrentExe -Destination $BackupExe -Force
    }

    $Installed = $false
    for ($Attempt = 0; $Attempt -lt 20 -and -not $Installed; $Attempt++) {
        try {
            Copy-Item -LiteralPath $NewExe -Destination $CurrentExe -Force
            $Installed = $true
        } catch {
            Start-Sleep -Milliseconds 500
        }
    }
    if (-not $Installed) {
        throw "无法替换 $CurrentExe；请关闭正在运行的 grok-zh 后重试"
    }

    try {
        if (Test-Path -LiteralPath $NewCatalog) {
            Copy-Item -LiteralPath $NewCatalog -Destination (Join-Path $I18nDir "zh-CN.json") -Force
        }
        if (Test-Path -LiteralPath $NewSchema) {
            Copy-Item -LiteralPath $NewSchema -Destination (Join-Path $I18nDir "schema.json") -Force
        }
        if (Test-Path -LiteralPath $NewInstaller) {
            Copy-Item -LiteralPath $NewInstaller -Destination (Join-Path $InstallDir "install-grok-zh.ps1") -Force
        }
        & $CurrentExe version | Out-Null
        Remove-Item -LiteralPath $BackupExe -Force -ErrorAction SilentlyContinue
    } catch {
        if (Test-Path -LiteralPath $BackupExe) {
            Copy-Item -LiteralPath $BackupExe -Destination $CurrentExe -Force
        }
        throw
    }

    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $PathParts = @($UserPath -split ';' | Where-Object { $_ })
    if ($PathParts -notcontains $InstallDir) {
        [Environment]::SetEnvironmentVariable(
            "Path",
            (($PathParts + $InstallDir) -join ';'),
            "User"
        )
        Write-Host "已把 $InstallDir 加入用户 PATH；新终端会自动生效。"
    }

    Write-Host "grok-zh $LatestVersion 安装完成。"
} finally {
    Remove-Item -LiteralPath $TempRoot -Recurse -Force -ErrorAction SilentlyContinue
}
