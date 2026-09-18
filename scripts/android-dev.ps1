# Ejecuta la aplicacion Android en modo desarrollo con hot reload.
# Uso:
#   ./scripts/android-dev.ps1
#   ./scripts/android-dev.ps1 Pixel_7
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

& node ./scripts/run-with-log.js --platform android --env AWS_LC_SYS_NO_ASM=1 -- cargo tauri android dev @TauriArgs
if ($LASTEXITCODE -ne 0) {
  exit $LASTEXITCODE
}
