# Dot-source from any working directory: . .\scripts\env.ps1
$taskRoot = Split-Path -Parent $PSScriptRoot
$taskCargoBin = Join-Path $taskRoot '.tools\cargo\bin'
if (-not (Test-Path -LiteralPath (Join-Path $taskCargoBin 'cargo.exe'))) {
    throw 'Project-local Rust toolchain was not found. See docs/toolchain.md.'
}
$env:RUSTUP_HOME = Join-Path $taskRoot '.tools\rustup'
$env:CARGO_HOME = Join-Path $taskRoot '.tools\cargo'
$env:RUSTUP_AUTO_INSTALL = '0'
$env:PATH = "$taskCargoBin;$env:PATH"
