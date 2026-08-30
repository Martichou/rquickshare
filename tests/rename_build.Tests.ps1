$ErrorActionPreference = 'Stop'
$root = Join-Path ([System.IO.Path]::GetTempPath()) ('rquickshare-rename-' + [guid]::NewGuid())
$scriptPath = Join-Path $PSScriptRoot '../rename_build.ps1'
New-Item -ItemType Directory -Path $root | Out-Null
try {
    foreach ($name in @('r-quick-share_0.11.5_x64.msi', 'RQuickShare-0.11.5-1-x64.exe', 'helper.exe', 'r-quick-share_0.11.5_x64[en].appx')) {
        [System.IO.File]::WriteAllText((Join-Path $root $name), $name)
    }
    & $scriptPath -BaseDirectory $root -TauriVersion main -DebugMode
    $expected = @('r-quick-share-main-debug_v0.11.5_x64.msi', 'r-quick-share-main-debug_v0.11.5_1-x64.exe', 'helper.exe', 'r-quick-share-main-debug_v0.11.5_x64[en].appx') | Sort-Object
    $actual = Get-ChildItem -LiteralPath $root -File | Select-Object -ExpandProperty Name | Sort-Object
    if (Compare-Object $expected $actual) { throw 'Unexpected renamed filenames' }
    & $scriptPath -BaseDirectory $root -TauriVersion main -DebugMode
    $actual = Get-ChildItem -LiteralPath $root -File | Select-Object -ExpandProperty Name | Sort-Object
    if (Compare-Object $expected $actual) { throw 'Renaming must be idempotent' }
    [System.IO.File]::WriteAllText((Join-Path $root 'r-quick-share_0.11.5_x64.msi'), 'collision')
    $rejected = $false
    try { & $scriptPath -BaseDirectory $root -TauriVersion main -DebugMode } catch { $rejected = $true }
    if (-not $rejected) { throw 'Existing destination was not protected' }
    if ([System.IO.File]::ReadAllText((Join-Path $root 'r-quick-share-main-debug_v0.11.5_x64.msi')) -ne 'r-quick-share_0.11.5_x64.msi') { throw 'Existing destination was overwritten' }
    Write-Host 'Artifact rename tests passed'
} finally {
    Remove-Item -LiteralPath $root -Recurse -Force
}
