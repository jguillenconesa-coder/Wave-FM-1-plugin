# Wave FM-1 Preview 0.2.4 · Windows y macOS

Añade **Musical clock → Internal / firmware o Ableton / DAW** por instancia.
El modo DAW genera MIDI Clock de 24 PPQN y Start/Stop desde el tempo y transporte
del host. Cada inicio o salto de posición reinicia la secuencia; el firmware
debe admitir reloj externo y usar su entrada TRS/MIDI. Consulta `docs/DAW_SYNC.md`.

Corrige la recuperación de audio cuando el motor se retrasa: mantiene la línea
de tiempo y descarta muestras tardías en lugar de estirar la reproducción.
Los eventos MIDI se programan por ciclos emulados, evitando agrupar los pulsos
en los límites de bloques DMA. Se limita la cola pendiente y se muestra un
aviso de underruns. Estas correcciones son comunes a todos los firmwares.
Una CPU que no alcanza tiempo real todavía puede producir cortes.

Corrige la posición visual de los knobs al mover un controlador MIDI asignado
en Ableton o una automatización del host. **Knob 1–4, Select, Presets y Algorithm**
reflejan su referencia 0–127 con un recorrido visual de 270°, también al reabrir
la ventana o recuperar un Set. La actualización del dibujo no envía giros
adicionales al motor. El ratón mantiene sus giros continuos; el siguiente cambio
del host vuelve a situar el indicador en su referencia MIDI. Esta referencia
no equivale al valor musical del parámetro que muestra cada firmware.

Se mantiene la corrección de descargas de **Community firmwares…** de 0.2.2:
el botón principal prioriza
la publicación original del autor en GitHub, en lugar del enlace del Hub que
puede responder HTTP 403. Melodee se comprueba con una descarga real desde el
mismo flujo que usa el botón principal. Se muestran la versión, el proveedor,
el progreso y la ruta guardada, o el error en rojo. Un archivo existente idéntico
se reutiliza tras verificarlo; uno diferente se conserva. Se añade descarga en
el navegador como alternativa para redes que bloqueen la conexión del plugin.

La consulta del Hub conserva las referencias del autor aunque el repositorio
venga como URL completa o cambie de mayúsculas. El fallback automático mantiene
versión y hash; no permite eludir una comprobación de integridad fallida.
Se conservan los controles MIDI, skins, firmwares incluidos e identidad VST3.

Nueva vista **MIDI / Live** con etiquetas legibles, siete encoders y 14 botones
expuestos como parámetros de host, además de Master. La guía integrada explica
Configure y MIDI Map. Los botones asignados se liberan tras un pulso de 100 ms;
la función de cada control depende del firmware. El panel conserva sus controles
manuales. Consulta `docs/MIDI_LIVE.md` para las referencias relativas de los encoders.

Nuevo explorador **Community firmwares…**: 83 referencias conocidas y consulta
manual de novedades en el Hub, con descarga opcional de archivos `.fwsc`.
64 publicaciones de los autores tienen enlaces alternativos de GitHub y hashes
de referencia. Descargar no ejecuta el paquete ni sustituye un Set. Estos
archivos externos no se redistribuyen dentro del instalador y no todos pasan
la prueba genérica de audio; consulta `docs/COMMUNITY_BROWSER.md` y el informe
de compatibilidad. El menú offline mantiene los 23 paquetes originales revisados.

VST3 experimental basado en fm1-emu de Joakim Eriksson. Añade un menú sin conexión
con **23 firmwares originales**, carga manual `.fwsc`/`.fm1save` y **seis skins**
basadas en los acabados de la imagen de referencia. Cada instancia conserva
firmware, flash y skin independientes. El instrumento sigue apareciendo como
**FM-1**, fabricante **fm1-emu**, con la identidad VST3 de la preview anterior.

## Descargas

- **Windows x64**: `Wave-FM-1-0.2.4-Windows-x64-Setup.exe`, con desinstalador
  en Agregar o quitar programas. Ruta predeterminada:
  `C:\Program Files\Common Files\VST3\Wave FM-1.vst3`.
- **Windows portátil**: `Wave-FM-1-0.2.4-Windows-x64-Portable.zip`. Mantén unido
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
Patterns, Pocket GammaWave, NOIZ DEMONS, Unholy Union, WaveLoop FM-1, Ketch y GR!MM.
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
