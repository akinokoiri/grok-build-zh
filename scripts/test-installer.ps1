$ErrorActionPreference = 'Stop'
$RepoRoot = Split-Path $PSScriptRoot -Parent
$TaskInstallerBytes = [IO.File]::ReadAllBytes((Join-Path $RepoRoot 'install.ps1'))
# The application's updater invokes powershell.exe -File, which needs a BOM
# to decode this Chinese script as UTF-8 instead of the system ANSI code page.
if ($TaskInstallerBytes.Length -lt 3 -or
    $TaskInstallerBytes[0] -ne 0xEF -or $TaskInstallerBytes[1] -ne 0xBB -or $TaskInstallerBytes[2] -ne 0xBF) {
    throw 'Installer must retain its UTF-8 BOM for the Windows update launcher'
}
$TaskInstallerSource = (Get-Content (Join-Path $RepoRoot 'install.ps1') -Raw -Encoding UTF8).Replace("`r`n", "`n")
$TaskStart = $TaskInstallerSource.IndexOf('    try {' + "`n" + '        $InstalledVersionOutput')
if ($TaskStart -lt 0) { throw 'Installer validation block not found' }
$TaskEnd = $TaskInstallerSource.IndexOf('    $UserPath =', $TaskStart)
$TaskBlock = [scriptblock]::Create($TaskInstallerSource.Substring($TaskStart, $TaskEnd - $TaskStart))
$TaskTempParent = [IO.Path]::GetFullPath([IO.Path]::GetTempPath())
$TaskFixtureRoot = [IO.Path]::GetFullPath((Join-Path $TaskTempParent ('grok-zh-installer-test-' + [Guid]::NewGuid().ToString('N'))))
New-Item -ItemType Directory -Path $TaskFixtureRoot | Out-Null
try {
    foreach ($TaskCase in @('success', 'nonzero', 'wrong-version', 'version-prefix', 'fresh-install-failure')) {
        $TaskCaseRoot = Join-Path $TaskFixtureRoot ('installer-' + $TaskCase + '-' + [Guid]::NewGuid().ToString('N'))
        New-Item -ItemType Directory -Path $TaskCaseRoot | Out-Null
        $InstallDir = $TaskCaseRoot
        $I18nDir = Join-Path $TaskCaseRoot 'i18n'
        New-Item -ItemType Directory -Path $I18nDir | Out-Null
        $CurrentExe = Join-Path $TaskCaseRoot 'probe.cmd'
        $BackupExe = "$CurrentExe.previous"
        $LatestVersion = '1.0.24-zh.1'
        $TaskOutputVersion = switch ($TaskCase) {
            'wrong-version' { '1.0.16-zh.1' }
            'version-prefix' { '1.0.24-zh.10' }
            default { $LatestVersion }
        }
        $TaskExitCode = if ($TaskCase -in @('nonzero', 'fresh-install-failure')) { 23 } else { 0 }
        [IO.File]::WriteAllText($CurrentExe, "@echo off`r`necho grok $TaskOutputVersion (test) [alpha]`r`nexit /b $TaskExitCode`r`n")
        if ($TaskCase -ne 'fresh-install-failure') { Set-Content $BackupExe 'previous-binary' }
        $NewCatalog = Join-Path $TaskCaseRoot 'catalog.json'
        $NewSchema = Join-Path $TaskCaseRoot 'schema.json'
        $NewInstaller = Join-Path $TaskCaseRoot 'installer.ps1'
        Set-Content $NewCatalog 'new-catalog'
        Set-Content $NewSchema 'new-schema'
        Set-Content $NewInstaller 'new-installer'
        Set-Content (Join-Path $I18nDir 'zh-CN.json') 'old-catalog'
        $TaskFailed = $false
        try { & $TaskBlock } catch { $TaskFailed = $true }
        if ($TaskCase -eq 'success') {
            if ($TaskFailed -or (Test-Path $BackupExe)) { throw 'Successful install failed validation' }
            if ((Get-Content (Join-Path $I18nDir 'zh-CN.json') -Raw).Trim() -ne 'new-catalog') { throw 'Catalog not installed' }
        } else {
            if (-not $TaskFailed) { throw "$TaskCase unexpectedly accepted" }
            if ($TaskCase -eq 'fresh-install-failure') {
                if (Test-Path $CurrentExe) { throw 'Failed first install left a broken binary' }
            } elseif ((Get-Content $CurrentExe -Raw).Trim() -ne 'previous-binary') { throw 'Rollback did not restore binary' }
            if ((Get-Content (Join-Path $I18nDir 'zh-CN.json') -Raw).Trim() -ne 'old-catalog') { throw 'Failed install changed catalog' }
        }
        Write-Output "Installer $TaskCase`: passed"
    }
    $TaskTokens = $null
    $TaskErrors = $null
    [void][System.Management.Automation.Language.Parser]::ParseFile((Join-Path $RepoRoot 'install.ps1'), [ref]$TaskTokens, [ref]$TaskErrors)
    if ($TaskErrors) { throw ($TaskErrors | Out-String) }
    Write-Output 'Installer PowerShell syntax: passed'

} finally {
    $TaskCleanupRoot = [IO.Path]::GetFullPath($TaskFixtureRoot)
    $TaskCleanupParent = [IO.Path]::GetFullPath((Split-Path $TaskCleanupRoot -Parent))
    if ($TaskCleanupParent.TrimEnd('\', '/') -ine $TaskTempParent.TrimEnd('\', '/') -or
        (Split-Path $TaskCleanupRoot -Leaf) -notmatch '^grok-zh-installer-test-[0-9a-f]{32}$') {
        throw "Refusing to clean unexpected installer test directory: $TaskCleanupRoot"
    }
    Remove-Item -LiteralPath $TaskCleanupRoot -Recurse -Force
}

# Expected native failures are test inputs. Only after every assertion and
# cleanup has succeeded, clear their exit code for the GitHub PowerShell host.
$global:LASTEXITCODE = 0
