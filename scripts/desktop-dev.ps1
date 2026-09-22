# Ejecuta la app de escritorio en modo desarrollo (tauri dev).
# Uso:
#   ./scripts/desktop-dev.ps1
#   ./scripts/desktop-dev.ps1 -- --release
# Variable opcional:
#   TAURI_CMD   comando de Tauri (defecto: "cargo tauri"), ej: "bun run tauri"
param(
  [Parameter(ValueFromRemainingArguments = $true)]
  [string[]]$TauriArgs
)

$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)   # raiz del proyecto

# Limpia variables de build de Android para no contaminar desktop
Remove-Item Env:AWS_LC_SYS_NO_ASM -ErrorAction SilentlyContinue
Remove-Item Env:CMAKE_GENERATOR -ErrorAction SilentlyContinue
Remove-Item Env:AWS_LC_SYS_CMAKE_GENERATOR -ErrorAction SilentlyContinue

if (-not $TauriArgs) {
  $TauriArgs = @()
} elseif ($TauriArgs[0] -eq "--") {
  $TauriArgs = @($TauriArgs | Select-Object -Skip 1)
}

$tauri = if ($env:TAURI_CMD) { $env:TAURI_CMD } else { "cargo tauri" }
$tauriCmd = @($tauri.Trim() -split "\s+")

& node (Join-Path $PSScriptRoot "run-with-log.js") --platform desktop -- @tauriCmd dev @TauriArgs
if ($LASTEXITCODE -ne 0) {
  exit $LASTEXITCODE
}