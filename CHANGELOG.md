# Changelog — Remit

## [1.1.1] — 2026-09-08

### Corregido

- Se corrigió la detección del propio dispositivo y de otros equipos mediante nombres mDNS únicos y hostnames dinámicos.
- Se evitó la creación duplicada de daemons mDNS y la acumulación de listeners al remontar la interfaz.
- Se controló la ausencia de red para evitar cierres inesperados.
- El servidor FTP ahora escucha en todas las interfaces y el cliente propaga los errores de inicio de sesión sin colapsar la aplicación.
- Se corrigió el resaltado de dispositivos para que solo se seleccione la tarjeta correspondiente.
- Se unificó el puerto anunciado por mDNS con el puerto del servidor FTP.
