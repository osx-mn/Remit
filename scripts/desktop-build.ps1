# Compila y empaqueta la aplicacion de escritorio en release (.exe, .msi, .nsis).
# NO debe tener AWS_LC_SYS_NO_ASM=1 porque aws-lc-sys lo prohibe en release:
#   "AWS_LC_SYS_NO_ASM only allowed for debug builds!"
# Uso:
#   ./scripts/desktop-build.ps1
#   ./scripts/desktop-build.ps1 -- --bundles msi
#   bun run desktop:build
param(
  [Parameter(ValueFromRemainingArguments = $true)]
  [string[]]$TauriArgs
)

$ErrorActionPreference = "Stop"

Remove-Item Env:AWS_LC_SYS_NO_ASM -ErrorAction SilentlyContinue
$env:CMAKE_GENERATOR = "Ninja"
$env:AWS_LC_SYS_CMAKE_GENERATOR = "Ninja"

if (-not $TauriArgs -or $TauriArgs.Count -eq 0) {
  $TauriArgs = @("build")
} elseif ($TauriArgs[0] -eq "--") {
  $TauriArgs = $TauriArgs | Select-Object -Skip 1
}

& node ./scripts/run-with-log.js --platform desktop -- cargo tauri @TauriArgs
if ($LASTEXITCODE -ne 0) {
  exit $LASTEXITCODE
}
