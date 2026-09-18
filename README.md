# Remit
#proyectos #remit #readme

---
**Remit** - Aplicación de escritorio multiplataforma para transferencia de archivos *peer-to-peer* en red local, sin servidor central. Descubre dispositivos por mDNS y transfiere vía FTP.

#### Descargar
Los instaladores para las diferentes plataformas se encuentran en la sección  [Releases](https://github.com/osx-mn/Remit/releases)

#### Uso
Al iniciar Remit, detecta automáticamente otros dispositivos en la misma red local que estén ejecutando la aplicación, permitiendo enviar archivos directamente.

![funcionamiento.gif](Docs/funcionamiento.gif)


#### Instalación
###### Compilar desde el código fuente

bash

```bash
git clone https://github.com/osx-mn/Remit.git
cd Remit
bun install
```

> Reemplaza `bun install` por `npm install` si prefieres npm.

Remit usa [Tauri](https://tauri.app/) + Rust, y la compilación de Android requiere **Ninja** como generador de CMake (el generador por defecto falla al compilar `aws-lc-sys`).

### Windows

Los scripts de PowerShell configuran las variables de entorno necesarias automáticamente:

bash

```bash
bun run desktop:dev      # Modo desarrollo (escritorio)
bun run desktop:build    # Build de producción (escritorio)
bun run android:dev      # Modo desarrollo (Android)
bun run android:build    # Build de producción (Android)
```

### macOS / Linux

Instala Ninja (`brew install ninja` en macOS, `apt install ninja-build` en Linux) y define las mismas variables de entorno manualmente:

bash

```bash
# Escritorio - desarrollo
export CMAKE_GENERATOR=Ninja
export AWS_LC_SYS_CMAKE_GENERATOR=Ninja
export AWS_LC_SYS_NO_ASM=1   # solo en debug, evita requerir Perl/NASM
cargo tauri dev

# Escritorio - build de producción
export CMAKE_GENERATOR=Ninja
export AWS_LC_SYS_CMAKE_GENERATOR=Ninja
cargo tauri build

# Android
export CMAKE_GENERATOR=Ninja
export AWS_LC_SYS_CMAKE_GENERATOR=Ninja
cargo tauri android build
```

> Soporte de Android/iOS en macOS y Linux aún no está probado oficialmente — si lo pruebas, feedback y PRs son bienvenidos.

## Licencia

MIT