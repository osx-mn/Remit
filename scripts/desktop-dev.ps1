# Ejecuta la aplicacion de escritorio en modo desarrollo.
# Usa aws-lc sin ensamblador para no requerir Perl/NASM en debug.
param(
  [Parameter(ValueFromRemainingArguments = $true)]
  [string[]]$TauriArgs
)

$ErrorActionPreference = "Stop"

$env:AWS_LC_SYS_NO_ASM = "1"
$env:CMAKE_GENERATOR = "Ninja"
$env:AWS_LC_SYS_CMAKE_GENERATOR = "Ninja"

if (-not $TauriArgs) {
  $TauriArgs = @()
}

& node ./scripts/run-with-log.js --platform desktop --env AWS_LC_SYS_NO_ASM=1 -- cargo tauri dev @TauriArgs
if ($LASTEXITCODE -ne 0) {
  exit $LASTEXITCODE
}