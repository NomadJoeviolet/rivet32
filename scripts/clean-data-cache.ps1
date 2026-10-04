# Cleanup inventory reviewed on 2026-10-04. Run in PowerShell; preview with -WhatIf.
# Refuses changed inputs. Does not touch .tools, vendor, patches or generated metadata.
[CmdletBinding(SupportsShouldProcess = $true, ConfirmImpact = 'Medium')]
param()

$ErrorActionPreference = 'Stop'
$workspaceRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).Path
if (-not (Test-Path -LiteralPath (Join-Path $workspaceRoot 'Cargo.toml'))) {
    throw 'Workspace manifest missing'
}
$sourceRoot = (Resolve-Path -LiteralPath (Join-Path $workspaceRoot 'data/sources')).Path
if ($sourceRoot -ne (Join-Path $workspaceRoot 'data\sources')) {
    throw 'Unexpected source-cache path'
}
if (Get-Process -Name cargo,rustc -ErrorAction SilentlyContinue) {
    throw 'Stop Cargo/rustc before cleanup'
}
$node = Get-Item -LiteralPath $sourceRoot -Force
while ($node.FullName -ne $workspaceRoot) {
    if ($node.Attributes -band [IO.FileAttributes]::ReparsePoint) {
        throw "Reparse point: $($node.FullName)"
    }
    $node = Get-Item -LiteralPath ([IO.Path]::GetDirectoryName($node.FullName)) -Force
}
$sourceItems = @(Get-ChildItem -LiteralPath $sourceRoot -Recurse -Force)
if ($sourceItems | Where-Object { $_.Attributes -band [IO.FileAttributes]::ReparsePoint }) {
    throw 'Source cache contains a reparse point'
}

# Keep inputs still read by capture_sources.py and the offline PAC provenance check.
$keepNames = @(
    'stm32-gaps/audit-results.json',
    'stm32-gaps/stm32h543xx.h',
    'stm32-gaps/stm32h553xx.h',
    'stm32-gaps/stm32h563xx.h',
    'stm32-gaps/stm32f723-rev9.pdf'
)
$generatorBase = 'stm32-gaps/stm32-data-e6a417fa643efaf031abc79590ea3e76bc4edbf2/'
$keepNames += @(
    'data/header_map.yaml',
    'stm32-data-gen/src/perimap.rs',
    'stm32-data-gen/src/memory.rs',
    'stm32-data-gen/src/chips.rs',
    'stm32-data-gen/src/dma.rs',
    'data/registers/rcc_h5.yaml',
    'data/registers/pwr_h5.yaml',
    'data/registers/flash_h5.yaml',
    'data/registers/gpdma_v1.yaml',
    'data/registers/i3c_v1.yaml',
    'data/registers/rng_v4.yaml'
) | ForEach-Object { $generatorBase + $_ }
foreach ($relativeName in $keepNames) {
    if (-not (Test-Path -LiteralPath (Join-Path $sourceRoot $relativeName) -PathType Leaf)) {
        throw "Required input missing: $relativeName"
    }
}
$candidates = @($sourceItems | Where-Object {
    -not $_.PSIsContainer -and
    $_.FullName.Substring($sourceRoot.Length + 1).Replace('\', '/') -notin $keepNames
} | Sort-Object FullName)
if ($candidates.Count -eq 0) {
    Write-Output 'Reviewed source-cache cleanup is already complete.'
    return
}
foreach ($candidate in $candidates) {
    $resolvedFile = (Resolve-Path -LiteralPath $candidate.FullName).Path
    if (-not $resolvedFile.StartsWith($sourceRoot + '\', [StringComparison]::OrdinalIgnoreCase) -or
        $resolvedFile -ne $candidate.FullName) {
        throw "Unsafe deletion target: $resolvedFile"
    }
}
$records = @($candidates | ForEach-Object {
    $_.FullName.Substring($sourceRoot.Length + 1).Replace('\', '/') + "`t" +
    $_.Length + "`t" + (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash
})
$digest = [Security.Cryptography.SHA256]::Create()
try {
    $inventoryHash = ([BitConverter]::ToString($digest.ComputeHash(
        [Text.Encoding]::UTF8.GetBytes(($records -join "`n"))
    ))).Replace('-', '').ToLowerInvariant()
} finally {
    $digest.Dispose()
}
if ($inventoryHash -ne '131300425c1a71449df9a541c91bca7c78ec94fc4d04e42b8dcab368bd617ba2') {
    throw 'Cache inventory changed since review; no files deleted. Review the new contents first.'
}
$removedBytes = ($candidates | Measure-Object Length -Sum).Sum
$summary = "$($candidates.Count) files, $([math]::Round($removedBytes / 1MB, 2)) MiB"
Write-Output "Reviewed source cache: $summary. Retaining $($keepNames.Count) referenced source inputs."
if (-not $PSCmdlet.ShouldProcess($sourceRoot, "Delete reviewed cache: $summary")) {
    return
}

$dataRoot = Join-Path $workspaceRoot 'data'
$preservedHashes = @{}
foreach ($file in (Get-ChildItem -LiteralPath $dataRoot -Recurse -File -Force)) {
    if ($file.FullName -notin $candidates.FullName) {
        $preservedHashes[$file.FullName] = (Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash
    }
}
foreach ($candidate in $candidates) {
    Remove-Item -LiteralPath $candidate.FullName -Force
}
foreach ($directory in ($sourceItems | Where-Object PSIsContainer |
        Sort-Object { $_.FullName.Length } -Descending)) {
    if (-not $directory.FullName.StartsWith($sourceRoot + '\', [StringComparison]::OrdinalIgnoreCase)) {
        throw 'Directory outside source cache'
    }
    if (@(Get-ChildItem -LiteralPath $directory.FullName -Force).Count -eq 0) {
        Remove-Item -LiteralPath $directory.FullName
    }
}
foreach ($path in $preservedHashes.Keys) {
    if ((Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash -ne $preservedHashes[$path]) {
        throw "Retained file changed: $path"
    }
}
if (@(Get-ChildItem -LiteralPath $sourceRoot -Recurse -File -Force).Count -ne $keepNames.Count) {
    throw 'Unexpected remaining source inventory'
}
Write-Output "Removed $summary. Retained data files passed SHA-256 comparison."
