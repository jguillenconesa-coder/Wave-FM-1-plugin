# Wave FM-1 Preview 0.2.8 · recuperación, knobs y biblioteca

- Recuperación de la cola tras una interrupción larga sin recargar el firmware
  ni borrar sus patches. El reloj interno se conserva; el modo DAW resincroniza
  su transporte. Un halt real del firmware se identifica por separado.
- Ratón y MIDI comparten una referencia 0–127 guardada, con límites y sin vuelta
  circular. Shift: ajuste fino. Alt-arrastre/rueda: navegación relativa sin fin.
- Nuevas versiones: Skiff 2.0.1, DUB-1 1.3.0, Chipbox 1.3.1-stable,
  TRIBE 0.3-beta, GammaWave 1.1.1-GW. Nuevo instrumento: SIDOU 0.4.0.
  Se conservan los 23 paquetes previos: 29 versiones de 24 instrumentos.
- Explorador comunitario: 96 referencias, 78 enlaces originales de autor,
  versiones preferidas claramente identificadas y hashes actualizados.
- Emulador de Joakim Eriksson fijado a `ba11e2e`, con correcciones originales de
  interrupciones/JIT. Se conservan avisos, autores y fuentes de cada firmware.

El usuario confirmó que DUB funcionaba mayormente al volver de 64 a 512 muestras,
pero comunicó bloqueos después de un error. La comparación anterior no demostró
una regresión de DUB. Esta preview aborda la recuperación; las pruebas automáticas
no sustituyen la comprobación dentro de Live con su controlador y configuración.
No se promete eliminar todos los cortes de X0X ni reducir la reserva de latencia.

Instala con Live cerrado y prueba primero en un Set vacío. La instalación usa
la ruta VST3 estándar y ofrece desinstalación en Agregar o quitar programas.
Las sesiones guardadas conservan su firmware; no se actualizan automáticamente.
Consulta los resultados concretos y los límites en `downloads/windows/VALIDATION.md`.
macOS solo se publica si supera sus propias comprobaciones nativas.

---

# Wave FM-1 Preview 0.2.7 · recuperación del audio al arrancar

**Comparación en revisión, 2026-10-25:** tras comunicar un posible empeoramiento
de DUB-1, el usuario ha observado que Live estaba a 64 muestras y está repitiendo
la prueba. Se comparará con 0.2.5 bajo los mismos ajustes antes de atribuir una
regresión de versión. X0X sigue pendiente de corregir en el equipo del usuario.

Corrige un atasco reproducible del plugin: el tiempo durante el que un firmware
arranca sin producir audio se acumulaba como muestras pendientes de descartar.
Con producción posterior a velocidad normal, el descarte podía continuar
indefinidamente. Ahora el productor suministra silencio durante ese arranque y
conserva el primer bloque real del firmware. Después del primer bloque no se
rellena artificialmente la producción del firmware.

Además, entrega el audio DMA ya terminado antes del siguiente trabajo del
segundo núcleo virtual, con fallback para bloques pendientes, y reutiliza la
memoria del bloque. Una regresión con X0X compara 88.576 muestras por canal,
registros de ambos núcleos, ciclos, RAM, flash y pantalla con el recorrido
original: todos coinciden. La prueba de Windows usa un temporizador privado
solo en el ejecutable de pruebas; se conserva el host anterior como control.

Se conservan el tamaño de la reserva elegida, la latencia real declarada, los
algoritmos de sonido, el reloj del firmware, la identidad VST3 y los parámetros
guardados. El callback añade la publicación atómica de su posición; no espera
al productor. La prioridad de Ableton y su configuración no cambian.

La regresión determinista reproduce el fallo a velocidad de producción exacta
1.00x, tanto a 64 como a 512 muestras. Las puertas de publicación ejecutan X0X
real a 48 kHz en ambas configuraciones, con el mismo contenido musical y cero
muestras ausentes durante la reproducción estable. Los resultados nativos de
cada plataforma deben pasar antes de publicar sus instaladores.

Esta corrección de arranque no prueba que todos los cortes comunicados por el
usuario estén resueltos dentro de Ableton. Los enlaces del README solo se
actualizan tras comprobar los archivos publicados. Los autores y licencias de
los 23 firmwares y de fm1-emu se conservan; no se añaden firmwares nuevos.
Windows `v0.2.7-preview.2` superó sus puertas nativas y la comprobación de los
archivos descargados. macOS 0.2.7 no superó audio a 64/512 y no se publicó;
su descarga recomendada sigue siendo `v0.2.6-preview.6`.

## Historial: Wave FM-1 Preview 0.2.6 · Windows y macOS

Paquetes recomendados verificados por plataforma:

- Windows: `v0.2.6-preview.4`, fuente `53c3966`.
- macOS: `v0.2.6-preview.6`, fuente `123425b`.

Los enlaces del README apuntan a esas compilaciones. `main` conserva la base
Windows verificada; la investigación posterior queda en una rama aparte.
Los candidatos Windows posteriores fallaron la comprobación de audio y no se
recomiendan. **El retraso persistente de X0X en el equipo del usuario sigue sin
darse por resuelto.** Pruebas y límites en `downloads/windows/VALIDATION.md`.

Corrige la latencia declarada por una instancia sin firmware: ahora es
**0 muestras**, con salida silenciosa. Al cargar un firmware se comunica al host
la reserva real de audio; al restaurar un estado vacío se detiene la máquina y
se vuelve a cero. La compensación de Live recibe las actualizaciones.

Nuevo selector **Audio buffer**, independiente del buffer ASIO de Live:

| Modo | Latencia del plugin a 48 kHz / bloque máximo 512 |
| --- | --- |
| Low latency | 582 muestras · 12,1 ms |
| Balanced | 1070 muestras · 22,3 ms |
| Safe / legacy (predeterminado) | 1627 muestras · 33,9 ms |

Cambia de modo con el transporte detenido y deja que se estabilice la cola.
Live puede reiniciar brevemente el procesamiento al cambiar la compensación.
Los modos más cortos requieren margen de CPU y planificación; pueden producir
más cortes. Safe conserva la reserva anterior y la compatibilidad de Sets.
Los valores dependen de la frecuencia y del bloque máximo negociado con el host;
no incluyen el buffer del dispositivo ni la respuesta interna del firmware.

El hilo de emulación solicita prioridad «Above normal» en Windows o QoS
«user-interactive» en macOS, con fallback si no está disponible. Windows no
usa prioridad de tiempo real ni las cuotas de MMCSS para el worker. No cambia la
prioridad global, el plan de energía ni los ajustes de Live. Cada instancia
mantiene su propio hilo y restaura su prioridad al cerrarse.

La interfaz FM-1 reutiliza los controles dibujados que no han cambiado y evita
subir una textura idéntica. Pantalla, LEDs y controles cambiados conservan sus
píxeles y su respuesta; una regresión compara el dibujo completo y el incremental
con los seis acabados. Esta optimización ahorra CPU gráfica; no acelera por sí
sola las instrucciones del firmware ni añade calidad de audio u oversampling.
**CPU clock** sigue siendo la frecuencia virtual: subirla cuesta más CPU real.

La franja inferior añade latencia efectiva y modo de planificación al diagnóstico
copiable. Se conservan los controles MIDI, flashes deliberados y correcciones de
recuperación de audio de las versiones anteriores. «Engine: 1.00x» y audio tarde
persistente todavía indican falta de margen; no se da por resuelto ese caso en
el equipo del usuario sin probarlo allí.

Las comprobaciones incluyen X0X real a 48 kHz / 512 durante 12 segundos y un
host construido con el SDK oficial que carga el VST3 vacío, procesa audio y
consulta latencia antes/después en nueve combinaciones de frecuencia y bloque.
Los ZIP incluyen `REALTIME_X0X.txt` y `EMPTY_LATENCY.txt`. Son pruebas acotadas;
no equivalen a una sesión interactiva de Ableton ni garantizan todos los equipos.

Se conserva **Musical clock → Internal / firmware o Ableton / DAW** por instancia.
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

- **Windows x64**: `Wave-FM-1-0.2.6-Windows-x64-Setup.exe`, con desinstalador
  en Agregar o quitar programas. Ruta predeterminada:
  `C:\Program Files\Common Files\VST3\Wave FM-1.vst3`.
- **Windows portátil**: `Wave-FM-1-0.2.6-Windows-x64-Portable.zip`. Mantén unido
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
