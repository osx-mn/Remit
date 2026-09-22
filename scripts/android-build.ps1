# Compila la app Android en release, firma el APK e instala en el dispositivo.
# Flujo: build -> zipalign -> apksigner -> verify -> adb install
# No usa AWS_LC_SYS_NO_ASM: ese flag solo es valido en debug.
#
# Uso:
#   ./scripts/android-build.ps1
#   ./scripts/android-build.ps1 -NoInstall
#   ./scripts/android-build.ps1 -- --target aarch64
#
# Variables opcionales:
#   TAURI_CMD        comando de Tauri     (defecto: "cargo tauri")
#   TAURI_KEYSTORE   ruta del .jks        (defecto: %USERPROFILE%\.keystores\<proyecto>.jks)
#   TAURI_KEY_ALIAS  alias de la clave    (defecto: <proyecto>)
#   TAURI_KS_PASS    password del keystore (si no existe, se pide al iniciar)
# <proyecto> se toma de productName (tauri.conf.json), luego name (package.json),
# y si no, del nombre de la carpeta.
param(
  [switch]$NoInstall,
  [Parameter(ValueFromRemainingArguments = $true)]
  [string[]]$TauriArgs
)

$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)   # raiz del proyecto

# --- Entorno de compilacion (release: sin AWS_LC_SYS_NO_ASM) ---
Remove-Item Env:AWS_LC_SYS_NO_ASM -ErrorAction SilentlyContinue
if (Get-Command ninja -ErrorAction SilentlyContinue) {
  $env:CMAKE_GENERATOR = "Ninja"
  $env:AWS_LC_SYS_CMAKE_GENERATOR = "Ninja"
}

# --- Argumentos ---
if (-not $TauriArgs) {
  $TauriArgs = @()
} elseif ($TauriArgs[0] -eq "--") {
  $TauriArgs = @($TauriArgs | Select-Object -Skip 1)
}
if ($TauriArgs -contains "-NoInstall") {
  $NoInstall = $true
  $TauriArgs = @($TauriArgs | Where-Object { $_ -ne "-NoInstall" })
}
if ($TauriArgs -contains "--debug") {
  throw "Este script es solo para release. Los APK debug ya vienen firmados."
}
if ($TauriArgs -notcontains "--apk") {
  $TauriArgs += "--apk"   # solo APK: el AAB no se instala ni se firma aqui
}

$tauri = if ($env:TAURI_CMD) { $env:TAURI_CMD } else { "cargo tauri" }
$tauriCmd = @($tauri.Trim() -split "\s+")

# --- Nombre del proyecto ---
function Get-ProjectName {
  foreach ($file in @("src-tauri/tauri.conf.json", "package.json")) {
    if (-not (Test-Path $file)) { continue }
    try {
      $json = Get-Content $file -Raw | ConvertFrom-Json
      $raw = if ($json.productName) { $json.productName } elseif ($json.name) { $json.name } else { "" }
      $slug = ($raw.ToLower() -replace "[^a-z0-9]+", "-").Trim("-")
      if ($slug) { return $slug }
    } catch { }
  }
  $slug = ((Get-Location).Path | Split-Path -Leaf).ToLower() -replace "[^a-z0-9]+", "-"
  return $slug.Trim("-")
}
$project = Get-ProjectName
if (-not $project) { $project = "app" }

# --- Herramientas del SDK ---
$sdk = if ($env:ANDROID_HOME) { $env:ANDROID_HOME } elseif ($env:ANDROID_SDK_ROOT) { $env:ANDROID_SDK_ROOT } else { $null }
if (-not $sdk) { throw "Define ANDROID_HOME apuntando al Android SDK." }

$buildTools = Get-ChildItem (Join-Path $sdk "build-tools") -Directory |
  Sort-Object { try { [version]$_.Name } catch { [version]"0.0" } } -Descending |
  Select-Object -First 1
if (-not $buildTools) { throw "No hay build-tools instalados en $sdk" }

$zipalign  = Join-Path $buildTools.FullName "zipalign.exe"
$apksigner = Join-Path $buildTools.FullName "apksigner.bat"
$adb       = Join-Path $sdk "platform-tools\adb.exe"
foreach ($tool in @($zipalign, $apksigner, $adb)) {
  if (-not (Test-Path $tool)) { throw "No se encontro: $tool" }
}

# --- Keystore (uno por proyecto, fuera del repo) ---
$keystore = if ($env:TAURI_KEYSTORE) { $env:TAURI_KEYSTORE } else { Join-Path $env:USERPROFILE ".keystores\$project.jks" }
$alias    = if ($env:TAURI_KEY_ALIAS) { $env:TAURI_KEY_ALIAS } else { $project }
$keytool  = if ($env:JAVA_HOME) { Join-Path $env:JAVA_HOME "bin\keytool.exe" } else { "keytool" }
$clearPass = $false

Write-Host "Proyecto: $project | Keystore: $keystore | Alias: $alias"

try {
  if (-not (Test-Path $keystore)) {
    Write-Host "No existe el keystore. Creando: $keystore"
    New-Item -ItemType Directory -Force (Split-Path $keystore) | Out-Null
    & $keytool -genkeypair -v -storetype PKCS12 -keystore $keystore -alias $alias -keyalg RSA -keysize 2048 -validity 10000
    if ($LASTEXITCODE -ne 0) { throw "No se pudo crear el keystore." }
    Write-Host "IMPORTANTE: guarda una copia del keystore y su password. Sin ellos no podras actualizar la app."
  }

  # Password: se pide ANTES del build para no esperar minutos y fallar al final.
  if (-not $env:TAURI_KS_PASS) {
    $secure = Read-Host "Password del keystore" -AsSecureString
    $env:TAURI_KS_PASS = [System.Net.NetworkCredential]::new("", $secure).Password
    $clearPass = $true
  }

  # Validar password y alias ANTES del build: falla en segundos, no tras minutos.
  $prevEap = $ErrorActionPreference
  $ErrorActionPreference = "Continue"   # PS 5.1: con "Stop", 2>&1 convierte stderr en excepcion
  $check = & $keytool -list -keystore $keystore -alias $alias "-storepass:env" TAURI_KS_PASS 2>&1
  $checkCode = $LASTEXITCODE
  $ErrorActionPreference = $prevEap
  if ($checkCode -ne 0) {
    throw "No se pudo abrir el keystore: password incorrecta o el alias '$alias' no existe.`n$(($check | Out-String).Trim())"
  }
  Write-Host "Keystore y password verificados."

  # --- Build ---
  $startTime = Get-Date
  & node (Join-Path $PSScriptRoot "run-with-log.js") --platform android -- @tauriCmd android build @TauriArgs
  if ($LASTEXITCODE -ne 0) { throw "El build fallo (codigo $LASTEXITCODE)." }

  # --- Localizar APK sin firmar ---
  $apkRoot = "src-tauri/gen/android/app/build/outputs/apk"
  $found = @(Get-ChildItem $apkRoot -Recurse -Filter "*-release-unsigned.apk")
  $unsigned = @($found | Where-Object { $_.LastWriteTime -ge $startTime })
  if ($unsigned.Count -eq 0 -and $found.Count -gt 0) {
    Write-Warning "Gradle no regenero el APK en este build; se firmara el existente."
    $unsigned = $found
  }
  if ($unsigned.Count -eq 0) { throw "No se encontro ningun *-release-unsigned.apk en $apkRoot" }

  # --- Alinear y firmar ---
  $outDir = "builds/android"
  New-Item -ItemType Directory -Force $outDir | Out-Null
  $signed = @()

  foreach ($apk in $unsigned) {
    $name    = $apk.BaseName -replace "-unsigned$", "-signed"
    $aligned = Join-Path $outDir "$name-aligned.apk"
    $final   = Join-Path $outDir "$name.apk"
    Remove-Item $aligned, $final -ErrorAction SilentlyContinue

    & $zipalign -f -p 4 $apk.FullName $aligned
    if ($LASTEXITCODE -ne 0) { throw "zipalign fallo con $($apk.Name)" }

    & $apksigner sign --ks $keystore --ks-key-alias $alias --ks-pass env:TAURI_KS_PASS --v4-signing-enabled false --out $final $aligned
    if ($LASTEXITCODE -ne 0) { throw "apksigner fallo (password o alias incorrectos?)" }

    & $apksigner verify $final
    if ($LASTEXITCODE -ne 0) { throw "La verificacion de la firma fallo: $final" }

    Remove-Item $aligned
    $signed += Get-Item $final
    Write-Host "Firmado: $final"
  }

  # --- Instalar ---
  if ($NoInstall) {
    Write-Host "Instalacion omitida (-NoInstall)."
  } else {
    $devices = @(& $adb devices | Select-String '\tdevice$')
    if ($devices.Count -eq 0) {
      Write-Warning "No hay dispositivo conectado por adb. APK firmados en: $outDir"
    } else {
      $serial = ($devices[0].Line -split "\s+")[0]
      $abi = (& $adb -s $serial shell getprop ro.product.cpu.abi).Trim()
      $abiName = @{ "arm64-v8a" = "arm64"; "armeabi-v7a" = "arm"; "x86_64" = "x86_64"; "x86" = "x86" }[$abi]

      $pick = $signed | Where-Object { $_.Name -like "*universal*" } | Select-Object -First 1
      if (-not $pick -and $abiName) {
        $pick = $signed | Where-Object { $_.Name -like "*-$abiName-*" } | Select-Object -First 1
      }

      if (-not $pick) {
        Write-Warning "Ningun APK coincide con el dispositivo ($abi). APK firmados en: $outDir"
      } else {
        Write-Host "Instalando $($pick.Name) en $serial ($abi)..."
        & $adb -s $serial install -r $pick.FullName
        if ($LASTEXITCODE -ne 0) {
          $id = try { (Get-Content src-tauri/tauri.conf.json -Raw | ConvertFrom-Json).identifier } catch { "<identifier>" }
          Write-Warning "Fallo la instalacion. Si dice INSTALL_FAILED_UPDATE_INCOMPATIBLE, la app instalada tiene otra firma (p. ej. debug). Desinstala con: adb uninstall $id"
          throw "adb install fallo."
        }
        Write-Host "Instalado correctamente."
      }
    }
  }
}
finally {
  if ($clearPass) { Remove-Item Env:TAURI_KS_PASS -ErrorAction SilentlyContinue }
}