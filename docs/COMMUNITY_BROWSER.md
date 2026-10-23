# Explorar y descargar firmwares comunitarios

**Community firmwares…** abre un explorador separado del menú Firmware.
El menú Firmware sigue incluyendo los **23 paquetes con fuentes y avisos
completos**, probados con arranque, pantalla y audio. El explorador contiene
**83 referencias** del índice de fm1-emu de Joakim Eriksson (MIT), con el autor
y el repositorio original de cada proyecto. No es una copia exhaustiva y en
tiempo real del Hub.

En esta revisión, **62 de las 64 descargas alternativas** pasaron la prueba
básica de arranque, LCD y ocho segundos de audio. BEETLE y PIGEON arrancaron
pero quedaron en silencio con la secuencia genérica. El explorador distingue
estos resultados: [informe, versiones y hashes](COMMUNITY_COMPATIBILITY.md).

- **Check Hub for new firmwares** consulta
  `https://thefirmwarehub.com/catalog.json` y muestra cuántos nombres/versiones
  han cambiado respecto del índice mostrado. Solo compara etiquetas de versión;
  no certifica qué versión es más reciente ni detecta cambios bajo la misma etiqueta.
- **Download .fwsc…** usa preferentemente la publicación original del autor en
  GitHub, con versión y SHA-256 de referencia. Así no depende de un enlace del
  Hub que pueda devolver HTTP 403. La fuente y versión elegidas aparecen junto
  al botón. Se localizaron y
  descargaron 64 publicaciones con un archivo FM-1 identificable en esta revisión.
  Las versiones del autor pueden diferir del índice del Hub: se muestra
  expresamente la etiqueta de GitHub y se comprueba su SHA-256 de referencia.
- Si la conexión con el autor falla, solo se intenta el Hub cuando ofrece la
  misma versión y el resultado puede comprobarse con el mismo hash. Un error
  de integridad detiene la operación; no se elude usando otra fuente.
- Se pide una ubicación y nombre para guardar un `.fwsc`. Si ya existe un
  archivo idéntico, se verifica y se reutiliza. Si es distinto, se conserva y
  se indica que elijas otro nombre. No se sobrescribe ni se modifica el bundle.
- El resultado aparece en el explorador: confirmación y ruta al guardarse,
  cancelación si cierras el diálogo, o error visible en rojo si falla. Una
  descarga fallida no deja disponible el botón para cargar un archivo anterior.
- **Download in browser** permite usar el navegador si un proxy, Windows o
  el servidor impiden la conexión del plugin. Después abre el archivo con
  **Load firmware / save…**. Ese enlace no verifica por sí mismo la descarga
  realizada por el navegador.
- Tras guardar, **Load downloaded firmware into this instance…** permite
  cargarlo expresamente. Sustituye firmware y patches de esa instancia;
  exporta antes el estado que quieras conservar. Descargar no lo ejecuta ni
  actualiza automáticamente un Set.

La consulta y descarga solo contactan con Internet cuando pulsas esas opciones.
No hay comprobaciones periódicas, telemetría, cuentas ni instalación de software
adicional. Se usa HTTPS con verificación TLS del sistema, límite de tamaño y
tiempo, destinos públicos permitidos y validación de cada redirección. Los
archivos pasan el parser de paquetes antes de guardarse; su hash se muestra en
pantalla. Un hash de una descarga nueva sin referencia no autentica al autor.

Desde este entorno, el Hub y su JSON devuelven HTTP 403. Las descargas
alternativas se localizaron en las publicaciones públicas de GitHub, sin usar
credenciales. Un error no elimina el índice ni los firmwares incluidos. No se
ha comprobado una conexión al Hub desde tu ordenador ni todas las funciones
de cada firmware. Algunos proyectos requieren archivos musicales propios,
entrada de audio, una tarjeta SD o hardware que el plugin no ofrece.

Las referencias externas **no son una autorización para redistribuir sus
binarios**. Los paquetes externos no se incorporan a los instaladores ni se
relicencian. Revisa las condiciones del autor, especialmente las de muestras,
presets y gráficos, antes de compartirlos. El plugin sigue aceptando archivos
propios mediante **Load firmware / save…**. Un firmware se ejecuta dentro del
DAW, sin sandbox; carga solo archivos de confianza.
