[CmdletBinding()]
param(
    [string]$Catalog = "crates/codegen/xai-grok-shared/i18n/zh-CN.json"
)

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path $PSScriptRoot -Parent
$CatalogPath = Join-Path $RepoRoot $Catalog

if (-not (Test-Path -LiteralPath $CatalogPath)) {
    throw "Translation catalog not found: $CatalogPath"
}

$Document = Get-Content -LiteralPath $CatalogPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($Document.version -ne 1 -or $Document.locale -ne "zh-CN") {
    throw "Translation catalog must use version=1 and locale=zh-CN"
}

$Translations = @{}
foreach ($Property in $Document.translations.PSObject.Properties) {
    if ([string]::IsNullOrWhiteSpace($Property.Name) -or
        [string]::IsNullOrWhiteSpace([string]$Property.Value)) {
        throw "Translation keys and values must be non-empty"
    }
    $Translations[$Property.Name] = [string]$Property.Value
}

$CommandNames = [System.Collections.Generic.HashSet[string]]::new(
    [System.StringComparer]::OrdinalIgnoreCase
)

$PagerCommands = Join-Path $RepoRoot "crates/codegen/xai-grok-pager/src/slash/commands"
Get-ChildItem -LiteralPath $PagerCommands -Filter "*.rs" -File |
    Where-Object {
        $_.Name -notlike "*test*" -and
        $_.Name -ne "screen_mode_switch.rs"
    } |
    ForEach-Object {
        $Source = Get-Content -LiteralPath $_.FullName -Raw -Encoding UTF8
        foreach ($Match in [regex]::Matches(
            $Source,
            'fn\s+name\s*\(&self\).*?\{\s*"([^"]+)"',
            [System.Text.RegularExpressions.RegexOptions]::Singleline
        )) {
            [void]$CommandNames.Add($Match.Groups[1].Value)
        }
    }

$ShellCommandsPath = Join-Path $RepoRoot "crates/codegen/xai-grok-shell/src/session/slash_commands.rs"
$ShellSource = Get-Content -LiteralPath $ShellCommandsPath -Raw -Encoding UTF8
foreach ($Match in [regex]::Matches($ShellSource, 'name:\s*"([a-z0-9-]+)"')) {
    [void]$CommandNames.Add($Match.Groups[1].Value)
}

# Commands whose name is selected dynamically or declared through a macro are
# not discoverable by the conservative source regex above.
foreach ($Name in @("exit", "fullscreen", "minimal", "hooks", "marketplace", "skills")) {
    [void]$CommandNames.Add($Name)
}

$ActionDefaultsPath = Join-Path $RepoRoot "crates/codegen/xai-grok-pager/src/actions/defaults.rs"
$ActionSource = Get-Content -LiteralPath $ActionDefaultsPath -Raw -Encoding UTF8
$ActionIds = [System.Collections.Generic.HashSet[string]]::new(
    [System.StringComparer]::Ordinal
)
foreach ($Match in [regex]::Matches($ActionSource, 'id:\s*ActionId::([A-Za-z0-9_]+)')) {
    [void]$ActionIds.Add($Match.Groups[1].Value)
}

$Missing = [System.Collections.Generic.List[string]]::new()
foreach ($Name in ($CommandNames | Sort-Object)) {
    $Key = "command.$Name.description"
    if (-not $Translations.ContainsKey($Key)) {
        $Missing.Add($Key)
    }
}
foreach ($Id in ($ActionIds | Sort-Object)) {
    foreach ($Field in @("label", "description")) {
        $Key = "action.$Id.$Field"
        if (-not $Translations.ContainsKey($Key)) {
            $Missing.Add($Key)
        }
    }
}

$PalettePath = Join-Path $RepoRoot "crates/codegen/xai-grok-pager/src/views/modal.rs"
$PaletteSource = Get-Content -LiteralPath $PalettePath -Raw -Encoding UTF8
$PaletteBlock = [regex]::Match(
    $PaletteSource,
    '(?s)fn\s+default_palette_entries.*?let\s+mut\s+entries\s*=\s*vec!\[(.*?)\n\s*\];'
).Groups[1].Value
foreach ($Match in [regex]::Matches($PaletteBlock, 'label:\s*"([^"]+)"')) {
    $Slug = ($Match.Groups[1].Value.ToLowerInvariant() -replace '[^a-z0-9]+', '_').Trim('_')
    $Key = "palette.$Slug"
    if (-not $Translations.ContainsKey($Key)) {
        $Missing.Add($Key)
    }
}

$RequiredIds = @(
    "welcome.menu.import_claude_settings",
    "welcome.menu.new_worktree",
    "welcome.menu.resume_session",
    "welcome.menu.changelog",
    "welcome.menu.logout",
    "welcome.menu.quit",
    "welcome.changelog.title",
    "welcome.tip.prefix",
    "welcome.mode.always_approve",
    "welcome.channel.stable",
    "welcome.channel.alpha",
    "reasoning_effort.none",
    "reasoning_effort.minimal",
    "reasoning_effort.low",
    "reasoning_effort.medium",
    "reasoning_effort.high",
    "reasoning_effort.xhigh",
    "reasoning_effort.max",
    "reasoning_effort.ultra",
    "source.Grok 4.6 is here!",
    "source.Select 'Grok 4.6' under /model.",
    "source.Use Shift+Tab to cycle between modes like Plan mode.",
    "source.Run /compact [context] when chat gets long.",
    "source.Press Ctrl+B to background a running terminal command.",
    'source.Start Grok in a fresh worktree with `-w`; add `-r <session-id>` to resume an existing session there.',
    "source.Use Ctrl+Enter to interject messages. Or just Enter to queue",
    "source.Run /dashboard (or Ctrl+\) to see and manage all your agents in one place.",
    "source.Try out workflows using /workflows.",
    "source.Use @ to attach files like @src/main.rs.",
    "source.Use @! for hidden or ignored files: @!.github/workflows.",
    "source.Press Ctrl+O to toggle auto-approve mode.",
    "skill.deep-research.description",
    "workflow.create-workflow.description",
    "workflow.deep-research.description",
    "persona.design-doc-reviewer.description",
    "persona.design-doc-writer.description",
    "persona.implementer.description",
    "persona.researcher.description",
    "persona.reviewer.description",
    "persona.security-auditor.description",
    "persona.test-writer.description"
)
foreach ($Key in $RequiredIds) {
    if (-not $Translations.ContainsKey($Key)) {
        $Missing.Add($Key)
    }
}

if ($Missing.Count -gt 0) {
    foreach ($Key in $Missing) {
        Write-Host "::error title=Missing Chinese translation::$Key"
    }
    throw "$($Missing.Count) required Chinese translations are missing"
}

# English literals in these high-churn UI catalogs are expected as safe
# fallbacks. Surface their count for the LLM maintenance task rather than
# guessing whether protocol/test strings are user-visible.
$ReviewFiles = @(
    "crates/codegen/xai-grok-pager/src/settings/defs.rs",
    "crates/codegen/xai-grok-pager/src/actions/defaults.rs",
    "crates/codegen/xai-grok-shell/src/session/slash_commands.rs"
)
$ReviewCount = 0
foreach ($RelativePath in $ReviewFiles) {
    $Path = Join-Path $RepoRoot $RelativePath
    if (-not (Test-Path -LiteralPath $Path)) { continue }
    $Source = Get-Content -LiteralPath $Path -Raw -Encoding UTF8
    $ReviewCount += [regex]::Matches(
        $Source,
        '(?:label|description):\s*"[A-Za-z]'
    ).Count
}

Write-Host "Translation audit passed: $($Translations.Count) entries; $($CommandNames.Count) command descriptions covered."
Write-Host "Shortcut audit: $($ActionIds.Count) built-in actions covered."
Write-Host "LLM review queue: $ReviewCount high-confidence English fallback literals across curated UI catalogs."
