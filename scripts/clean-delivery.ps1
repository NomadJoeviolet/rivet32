# Manual cleanup entry point. Preview with -WhatIf before executing locally.
[CmdletBinding(SupportsShouldProcess = $true, ConfirmImpact = 'Medium')]
param()
$ErrorActionPreference = 'Stop'
$workspaceRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..')).TrimEnd([IO.Path]::DirectorySeparatorChar)
if (-not (Test-Path -LiteralPath (Join-Path $workspaceRoot 'Cargo.toml'))) { throw 'Workspace manifest missing' }
if (Get-Process -Name cargo,rustc -ErrorAction SilentlyContinue) { throw 'Stop Cargo/rustc processes before cleanup.' }
# BEGIN REMOVAL PATHS
$obsoletePaths = @(
    '.cache',
    '.github/fixtures',
    'App/src/bin/f723_r_pins.rs',
    'App/src/bin/h5_li_pins.rs',
    'App/src/bin/peripherals_h723.rs',
    'App/tests',
    'crates/embodied-algorithms/tests',
    'crates/embodied-core/tests',
    'crates/embodied-devices/src/vt03.rs',
    'crates/embodied-devices/tests',
    'crates/embodied-runtime/tests',
    'crates/embodied-stm32/tests',
    'data/validation',
    'dist',
    'docs/STATUS.md',
    'docs/algorithms-report.md',
    'docs/can-runtime.md',
    'docs/ci-cd.md',
    'docs/core-runtime-report.md',
    'docs/delivery-audit.md',
    'docs/devices-report.md',
    'docs/fdcan-flush.md',
    'docs/fdcan-timestamp.md',
    'docs/h7rs-fdcan.md',
    'docs/hal-and-boards.md',
    'docs/hardware-compare-timer.md',
    'docs/implementation-plan.md',
    'docs/lightweight-check-20261003.md',
    'docs/peripheral-clock-audit.md',
    'docs/peripheral-smoke.md',
    'docs/sheriffos-migration.md',
    'docs/stm32-g0-ucpd.md',
    'docs/stm32-gaps.md',
    'docs/stm32-h5-47a.md',
    'docs/stm32-h5-47c.md',
    'docs/stm32-h5-li.md',
    'docs/toolchain.md',
    'docs/xtask-report.md',
    'scripts/test_package_release.py',
    'scripts/test_verify_generated_project.py',
    'xtask/scripts/check_fdcan_flush.py',
    'xtask/scripts/fixtures',
    'xtask/scripts/test_cache_retention.py',
    'xtask/scripts/test_catalog.py',
    'xtask/scripts/test_dma_priority_contract.py',
    'xtask/scripts/test_dual_rcc_contract.py',
    'xtask/scripts/test_f723_patch.py',
    'xtask/scripts/test_f7_otp_overlay.py',
    'xtask/scripts/test_fdcan_flush.py',
    'xtask/scripts/test_fdcan_timestamp.py',
    'xtask/scripts/test_fetch_source_documents.py',
    'xtask/scripts/test_generated_dual_pair.py',
    'xtask/scripts/test_h5_47a_composition.py',
    'xtask/scripts/test_h5_47a_hal_composition.py',
    'xtask/scripts/test_h5_47a_hal_replay.py',
    'xtask/scripts/test_h5_47a_integration.py',
    'xtask/scripts/test_h5_47a_pac_replay.py',
    'xtask/scripts/test_h5_47a_regeneration.py',
    'xtask/scripts/test_h5_47c_baseline.py',
    'xtask/scripts/test_h5_47c_composition.py',
    'xtask/scripts/test_h5_47c_hal.py',
    'xtask/scripts/test_h5_li_patch.py',
    'xtask/scripts/test_h5_rtc_overlay.py',
    'xtask/scripts/test_h7rs_usb_clock.py',
    'xtask/scripts/test_hal_platform_behavior.py',
    'xtask/scripts/test_hal_platform_fixes.py',
    'xtask/scripts/test_hal_post_patches.py',
    'xtask/scripts/test_patch_manifest.py',
    'xtask/scripts/test_peripheral_clocks.py',
    'xtask/scripts/test_peripheral_dual.py',
    'xtask/scripts/test_peripheral_legacy_clocks.py',
    'xtask/scripts/test_peripheral_matrix.py',
    'xtask/scripts/test_peripheral_recipes.py',
    'xtask/scripts/test_source_checkout.py',
    'xtask/scripts/test_source_package_overlay.py',
    'xtask/scripts/test_ucpd_init.py',
    'xtask/scripts/test_vendor_embassy.py',
    'xtask/scripts/verify_peripheral_clock_fixtures.py',
    'xtask/tests'
)
# END REMOVAL PATHS
$verifiedTargets = @()
foreach ($relative in $obsoletePaths) {
    $absolute = [IO.Path]::GetFullPath((Join-Path $workspaceRoot $relative))
    if (-not $absolute.StartsWith($workspaceRoot + [IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) { throw "Outside workspace: $absolute" }
    if (-not (Test-Path -LiteralPath $absolute)) { continue }
    $resolved = (Resolve-Path -LiteralPath $absolute).Path
    if ($resolved -ne $absolute) { throw "Unexpected resolution: $absolute" }
    $node = Get-Item -LiteralPath $absolute -Force
    while ($node.FullName -ne $workspaceRoot) {
        if ($node.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw "Reparse point: $($node.FullName)" }
        $parentPath = [IO.Path]::GetDirectoryName($node.FullName)
        $node = Get-Item -LiteralPath $parentPath -Force
    }
    $item = Get-Item -LiteralPath $absolute -Force
    if ($item.PSIsContainer -and (Get-ChildItem -LiteralPath $absolute -Force -Recurse -Attributes ReparsePoint)) { throw "Nested reparse point: $absolute" }
    $verifiedTargets += $absolute
}
foreach ($absolute in $verifiedTargets) {
    if ($PSCmdlet.ShouldProcess($absolute, 'Remove obsolete workspace files')) {
        Remove-Item -LiteralPath $absolute -Recurse -Force
    }
}
Write-Output "Reviewed $($verifiedTargets.Count) existing obsolete paths."
