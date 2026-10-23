# Wave FM-1 Preview 0.2.0 · Windows y macOS

VST3 experimental basado en fm1-emu de Joakim Eriksson. Añade un menú sin conexión
con **21 firmwares originales**, carga manual `.fwsc`/`.fm1save` y **seis skins**
basadas en los acabados de la imagen de referencia. Cada instancia conserva
firmware, flash y skin independientes. El instrumento sigue apareciendo como
**FM-1**, fabricante **fm1-emu**, con la identidad VST3 de la preview anterior.

## Descargas

- **Windows x64**: `Wave-FM-1-0.2.0-Windows-x64-Setup.exe`, con desinstalador
  en Agregar o quitar programas. Ruta predeterminada:
  `C:\Program Files\Common Files\VST3\Wave FM-1.vst3`.
- **Windows portátil**: `Wave-FM-1-0.2.0-Windows-x64-Portable.zip`. Mantén unido
  todo el bundle `VST3/FM-1.vst3`; evita dos copias simultáneas.
- **macOS Universal 2**: ZIP y PKG para Intel/Apple Silicon, compilados con
  mínimo macOS 11. El ZIP permite instalar por usuario; el PKG instala el bundle
  en `/Library/Audio/Plug-Ins/VST3/`. No incluye AU.
- `SHA256SUMS`, `SHA256SUMS-macos` y manifiestos `build-info-*` identifican
  los archivos publicados. Los assets se adjuntan solo tras completar las
  comprobaciones de cada plataforma; ambas compilaciones pueden acabar a distinta hora.

Desinstala primero la preview Windows 0.1.0 para evitar duplicados, conservando
proyectos y exportaciones. Cierra Live durante instalación/desinstalación.
El instalador solo administra su producto: no modifica archivos de Live,
preferencias, proyectos, otros plugins ni instala servicios o controladores.
Live puede descubrirlo automáticamente al explorar la carpeta VST3 estándar.

## Firmwares, autores y fuentes

Felucca, X0X, AMB-1, FoMni-1, Polly, Skiff, DUB-1, fm1-chord, CTL-1 (USB),
FiMba-1, MIN-1, Niagara Drift, TRIBE, Chipbox, Frodhes, FMini-1, Felucca 8
Patterns, Pocket GammaWave, NOIZ DEMONS, Unholy Union y WaveLoop FM-1.
Las versiones, autores y licencias están en `firmware/README.md`. Los paquetes
conservan sus fuentes completas, submódulos necesarios y avisos dentro del bundle.
No se incluye firmware propietario de fábrica. `firmware/RESEARCH.md` registra
los otros candidatos y por qué quedan pendientes o fuera de esta distribución.

## Pruebas y límites

Los paquetes reales pasan pruebas del motor de arranque, pantalla y audio;
la carga portable y los errores de integridad también se comprueban. Las
compilaciones nativas y sus registros están en GitHub Actions. Consulta
`downloads/windows/VALIDATION.md` y `docs/SECURITY_REVIEW.md` para distinguir
las comprobaciones ejecutadas y sus límites. No se han probado todas las
funciones de cada firmware ni todas las configuraciones de Live.

**Prueba primero en un Set vacío, con proyectos importantes cerrados y copia
de seguridad.** Un plugin experimental dentro del DAW no permite garantizar
riesgo cero. Windows no tiene firma Authenticode propia. macOS lleva firma
ad-hoc, sin Developer ID ni notarización; el PKG no está firmado por Apple y
Gatekeeper puede bloquearlo. No desactives protecciones del sistema.

No está afiliado a M-VAVE ni a Ableton. El emulador no es un sandbox: abre solo
firmwares y estados de confianza. Persisten avisos de mantenimiento de dependencias,
registrados en la revisión de seguridad.
