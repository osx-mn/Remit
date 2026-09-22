# Ejecuta la app Android en modo desarrollo con hot reload (tauri android dev).
# Uso:
#   ./scripts/android-dev.ps1
#   ./scripts/android-dev.ps1 -Asm        (no define AWS_LC_SYS_NO_ASM)
# Variable opcional:
#   TAURI_CMD   comando de Tauri (defecto: "cargo tauri"), ej: "bun run tauri"
param(
  [switch]$Asm,
  [Parameter(ValueFromRemainingArguments = $true)]
  [string[]]$TauriArgs
)

$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)   # raiz del proyecto

if (-not $TauriArgs) {
  $TauriArgs = @()
} elseif ($TauriArgs[0] -eq "--") {
  $TauriArgs = @($TauriArgs | Select-Object -Skip 1)
}
if ($TauriArgs -contains "-Asm") {
  $Asm = $true
  $TauriArgs = @($TauriArgs | Where-Object { $_ -ne "-Asm" })
}

# aws-lc-sys sin ensamblador evita requerir Perl/NASM en debug (inocuo si el proyecto no lo usa)
if ($Asm) {
  Remove-Item Env:AWS_LC_SYS_NO_ASM -ErrorAction SilentlyContinue
} else {
  $env:AWS_LC_SYS_NO_ASM = "1"
}

# Ninja solo si esta instalado
if (Get-Command ninja -ErrorAction SilentlyContinue) {
  $env:CMAKE_GENERATOR = "Ninja"
  $env:AWS_LC_SYS_CMAKE_GENERATOR = "Ninja"
}

$tauri = if ($env:TAURI_CMD) { $env:TAURI_CMD } else { "cargo tauri" }
$tauriCmd = @($tauri.Trim() -split "\s+")

& node (Join-Path $PSScriptRoot "run-with-log.js") --platform android -- @tauriCmd android dev @TauriArgs
if ($LASTEXITCODE -ne 0) {
  exit $LASTEXITCODE
}