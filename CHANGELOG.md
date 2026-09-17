# Changelog — Remit

## [1.1.2] — 2026-09-15

### Corregido

- Se solucionó la selección de archivos en Android: ahora se abren correctamente las URI `content://` que devuelve el selector nativo del sistema (a través de `tauri-plugin-fs`), sin necesidad de pedir permisos amplios de almacenamiento.
- Se corrigió la ubicación donde se inicializaba `remit_data.db` en Android, para que use el directorio de datos privado de la app en lugar de una ubicación menos segura.
- Se unificó el cliente FTP en una sola implementación multiplataforma, usada tanto en escritorio como en Android, reduciendo duplicación de código y posibles inconsistencias.
- Se corrigió el nombre de los archivos enviados desde Android: ahora se obtiene `DISPLAY_NAME` del proveedor de documentos y se conserva la extensión original.