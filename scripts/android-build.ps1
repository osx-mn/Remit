# Compila la aplicacion Android en release y genera APK/AAB.
# No usa AWS_LC_SYS_NO_ASM: ese flag solo es valido en debug.
# Uso:
#   ./scripts/android-build.ps1
#   ./scripts/android-build.ps1 -- --apk true --aab false
param(
  [Parameter(ValueFromRemainingArguments = $true)]
  [string[]]$TauriArgs
)

$ErrorActionPreference = "Stop"

Remove-Item Env:AWS_LC_SYS_NO_ASM -ErrorAction SilentlyContinue
$env:CMAKE_GENERATOR = "Ninja"
$env:AWS_LC_SYS_CMAKE_GENERATOR = "Ninja"

if (-not $TauriArgs) {
  $TauriArgs = @()
} elseif ($TauriArgs[0] -eq "--") {
  $TauriArgs = $TauriArgs | Select-Object -Skip 1
}

& node ./scripts/run-with-log.js --platform android -- cargo tauri android build @TauriArgs
if ($LASTEXITCODE -ne 0) {
  exit $LASTEXITCODE
}
