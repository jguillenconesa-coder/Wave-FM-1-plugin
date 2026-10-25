# Wave FM-1 Preview

Plugin **VST3 de 64 bits** que emula el M-VAVE FM-1 y permite cargar firmwares
`.fwsc`. Se basa en [fm1-emu de Joakim Eriksson](https://github.com/joakimeriksson/fm1-emu):
el motor, el panel interactivo y la pantalla del firmware proceden de ese proyecto.
Los créditos y licencias originales se conservan. La cuenta que mantiene este
repositorio no se presenta como autora de ese trabajo; consulta
[autoría y procedencia](AUTHORS.md).

## Descargar e instalar

**Windows 0.2.8:** recuperación de audio sin recargar el firmware, knobs con
posición 0–127 guardada, biblioteca actualizada y correcciones del arranque y los
loops con reloj DAW. Compilación/instalador y pruebas nativas superados; archivos
publicados descargados y verificados. Sigue siendo una preview: confirma el
comportamiento en un Set vacío antes de usarla en tus proyectos importantes.

- [Instalador Windows 0.2.8](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/releases/download/v0.2.8-preview.3/Wave-FM-1-0.2.8-Windows-x64-Setup.exe)
- [ZIP portátil Windows](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/releases/download/v0.2.8-preview.3/Wave-FM-1-0.2.8-Windows-x64-Portable.zip)
- [macOS Universal: instalador PKG](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/releases/download/v0.2.6-preview.6/Wave-FM-1-0.2.6-macOS-Universal.pkg)
- [macOS Universal: ZIP por usuario](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/releases/download/v0.2.6-preview.6/Wave-FM-1-0.2.6-macOS-Universal.zip)
- [Windows: versión y hashes](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/releases/tag/v0.2.8-preview.3) · [macOS: versión y hashes](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/releases/tag/v0.2.6-preview.6)
- [Instalar/desinstalar en Windows](docs/WINDOWS.md) · [macOS Intel y Apple Silicon](docs/MACOS.md)

Si la descarga directa falla, abre la página de la versión y selecciona el
archivo en **Assets**. Los paquetes macOS solo se publican si su compilación y
validación nativa terminan correctamente; su ausencia indica que siguen pendientes.

Windows instala por defecto en
`C:\Program Files\Common Files\VST3\Wave FM-1.vst3`, requiere permisos de
administrador y se elimina desde **Agregar o quitar programas**. La instalación
no cambia archivos de Live, sus preferencias ni proyectos. Al estar en la ruta
VST3 estándar, Live puede descubrirlo automáticamente en su siguiente escaneo.
Si tienes instalada la preview 0.1.0, desinstálala antes para evitar duplicados;
conserva tus proyectos y archivos exportados.

**Preview experimental.** El usuario ha confirmado la carga en Live Windows y
ha confirmado la mejora al volver a 512 muestras y comunicó bloqueos tras errores.
Las correcciones actuales deben confirmarse todavía en ese equipo. Esto no
valida todas las configuraciones ni todos los firmwares. Prueba la actualización en un Set vacío con tus proyectos
importantes cerrados y respaldados; no se garantiza riesgo cero.

## Firmwares y aspecto

**0.2.8** añade cinco versiones
(Skiff 2.0.1, DUB-1 1.3.0, Chipbox 1.3.1-stable, TRIBE 0.3-beta y GammaWave
1.1.1-GW) y SIDOU 0.4.0. Conserva todos los paquetes anteriores. El ratón y MIDI
comparten una referencia 0–127 guardada; Shift afina y Alt permite giros sin fin.
La recuperación tras una interrupción larga conserva el firmware y sus patches.
También corrige el arranque del reloj DAW y la resincronización de loops en X0X,
sin modificar sus bytes. [Detalles y configuración](docs/DAW_SYNC.md).

**0.2.6** declara **cero latencia sin firmware** y añade **Audio buffer**:
Low latency, Balanced y Safe / legacy. A 48 kHz / bloque máximo 512, sus latencias
son 12,1 / 22,3 / 33,9 ms; Safe conserva la reserva anterior. Cambia con Live
detenido. Los modos cortos requieren más margen y pueden aumentar los cortes.
La interfaz reutiliza controles dibujados para consumir menos CPU y el worker
Windows usa prioridad moderada, sin cambiar la prioridad del proceso ni ajustes
de Live. El algoritmo de sonido se conserva.

**0.2.7 corrige un atasco de recuperación durante el arranque del firmware.**
El silencio inicial podía dejar al plugin descartando audio indefinidamente
cuando la producción posterior alcanzaba exactamente 1.00x. La corrección
prepara ese silencio en el productor y conserva el primer bloque real, sin
aumentar la reserva ni modificar los algoritmos de sonido. Además, entrega
los bloques ya terminados antes del siguiente trabajo del segundo núcleo
virtual y reutiliza su memoria. Una comparación de 88.576 muestras por canal de X0X
confirma muestras, registros, reloj, RAM, flash y pantalla idénticos.

X0X real pasa las pruebas nativas a **48 kHz / 64 y 512 muestras** en Windows,
con cero muestras ausentes durante la reproducción estable. La
regresión determinista comprueba también la producción a velocidad exacta 1x.
Esto no sustituye la prueba dentro de Ableton en el equipo del usuario ni
establece que todos sus cortes estén resueltos. Consulta las
[fuentes, resultados y límites](downloads/windows/VALIDATION.md).

**Musical clock** permite elegir el reloj propio del firmware o sincronizar tempo
y reproducción con **Ableton / DAW**. El firmware debe admitir clock externo por
TRS/MIDI y tenerlo seleccionado. La versión 0.2.4 también corrige la recuperación
de faltas de audio para conservar el tiempo y programa el MIDI por ciclos del
emulador. Los cortes se indican en **Audio late**; ese contador por sí solo no identifica
la causa. Consulta [configuración y límites de la sincronización](docs/DAW_SYNC.md).

**MIDI / Live** muestra etiquetas de alto contraste y una columna con Master,
los siete encoders y 14 botones asignables. Incluye instrucciones de Configure
y MIDI Map; los nombres coinciden con los parámetros de Live. Consulta
[cómo mapear tu controlador y qué representan los valores](docs/MIDI_LIVE.md).

Desde **0.2.3**, los indicadores de los siete encoders del panel, incluidos
**Knob 1–4**, siguen la referencia 0–127 recibida desde Live mediante MIDI o
automatización. Se recuperan al reabrir la ventana. El dibujo usa un recorrido
de 270°; la función musical de cada encoder depende del firmware.

**Community firmwares…** permite buscar entre 96 referencias conocidas,
consultar novedades en Firmware Hub y guardar un `.fwsc` sin actualizar un Set
automáticamente. Incluye enlaces alternativos a **78 publicaciones originales
de GitHub**, con hash de referencia. Los paquetes externos no forman parte
del instalador ni equivalen a firmwares completamente verificados. Consulta
[el explorador, las descargas y sus límites](docs/COMMUNITY_BROWSER.md).

En **0.2.2**, el botón **Download .fwsc…** utiliza preferentemente la publicación
original del autor y muestra la ruta guardada o el error. Corrige la descarga de
Melodee, bloqueada en el enlace del Hub. **Download in browser** permite guardar
manualmente desde el navegador; después se puede abrir el archivo en el plugin.

El menú **Firmware…** incluye **29 versiones de 24 instrumentos sin conexión**, con versiones
fijadas y pruebas reales de arranque, pantalla y audio. Entre ellos: Felucca,
X0X, AMB-1, FoMni-1, Polly, Skiff, DUB-1, Chipbox, Frodhes y Niagara Drift.
Consulta [la lista completa, autores, licencias y fuentes](firmware/README.md)
y [los candidatos pendientes o excluidos](firmware/RESEARCH.md).
Cada paquete se comprueba por tamaño y SHA-256 antes de cargarlo.

El menú conserva **Open firmware / save file…** y el botón **Load firmware /
save…** para `.fwsc` y `.fm1save` propios. Cambiar de firmware reinicia solo esa
instancia y sus patches; exporta primero para conservarlos. Cada instancia
mantiene motor, firmware, flash y skin independientes. Guarda los cambios desde
el firmware antes de guardar el Set: se conserva firmware y flash, no toda la RAM.

**Skin** ofrece los seis acabados de la imagen de referencia: **Graphite**,
**Ivory / charcoal**, **Orange**, **Lilac**, **Ivory / blue** y
**Graphite / turquoise**. Son paletas dibujadas a partir de esa referencia;
no se incluye la fotografía ni se afirma una equivalencia colorimétrica exacta.
Cambian carcasa, teclas y controles; pantalla y LED conservan sus colores.
La selección se guarda con cada instancia del proyecto.

![Las seis paletas dibujadas por el panel del plugin](docs/images/skins.png)

Vista del panel original de fm1-emu (Joakim Eriksson, MIT) con las paletas de
esta integración. La pantalla de muestra es uniforme; al cargar un firmware
mostrará su interfaz. No se ha utilizado la fotografía del fabricante.

El instrumento sigue apareciendo como **FM-1**, fabricante **fm1-emu**, con
la identidad VST3 de la preview para recuperar proyectos anteriores.
No se garantiza un número ilimitado de instancias; el consumo de CPU varía.

## Desarrollo y compilación

Requisitos Windows: Git, Python 3, PowerShell 7, Rust 1.99.1, CMake >=3.27,
Visual Studio 2022 C++/Windows SDK e Inno Setup 7.1.0.

```powershell
./tools/build-windows.ps1
./tools/package-windows.ps1
```

También hay una compilación cruzada Linux/MinGW-w64 POSIX en
`tools/build-windows-cross.sh`. `cmake/windows-mingw.cmake` configura Windows
x64. El emulador se descarga en `.deps/fm1-emu` con revisión fija y la copia
original permanece intacta. Las adaptaciones de interfaz y temporización se
aplican a un árbol de compilación separado con el origen verificado por hash;
consulta [los cambios y su procedencia](desktop/README.md).
El wrapper recibe un parche local verificado por hash para cargar
únicamente su módulo CLAP privado. El bundle VST3 incluye ambos binarios y las
bibliotecas de ejecución se enlazan estáticamente.

Las pruebas de integridad propias se ejecutan con:

```sh
python3 -m unittest discover -s tests -v
```

El fork de interfaz está en `desktop/`; `tools/prepare-desktop.py` verifica la
base y genera un árbol separado sin modificar el emulador fijado. Para macOS:
`bash tools/build-macos.sh` y `bash tools/package-macos.sh` desde un Mac.

La preparación del motor en la nube sigue en `tools/setup-cloud.sh`. Usa el
checkout existente; no crees worktrees salvo petición explícita. GitHub Actions
repite las auditorías y prepara builds; un fallo de CI no convierte las pruebas
locales en una validación nativa. Los tags `v*-preview.*` publican los assets
verificados como prerelease, sin considerarlos versiones estables.

## Créditos, seguridad y limitaciones

- [Créditos y licencias de terceros](THIRD_PARTY_NOTICES.md)
- [Política de seguridad](SECURITY.md) e [informe de revisión](docs/SECURITY_REVIEW.md)
- [Contribuciones](CONTRIBUTING.md)

Este proyecto no está afiliado a M-VAVE ni a Ableton. Solo abre firmwares,
proyectos y estados de fuentes de confianza: el emulador se ejecuta dentro del
DAW y **no es un sandbox**. La preview no tiene firma Authenticode propia.
Consulta las comprobaciones y limitaciones; no desactives protecciones del sistema.
