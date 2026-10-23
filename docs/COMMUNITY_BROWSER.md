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
- **Download .fwsc…** descarga el paquete que indica el Hub. **GitHub [versión]…**
  usa una publicación original del autor como alternativa. Se localizaron y
  descargaron 64 publicaciones con un archivo FM-1 identificable en esta revisión.
  Las versiones alternativas pueden diferir del índice del Hub: se muestra
  expresamente la etiqueta de GitHub y se comprueba su SHA-256 de referencia.
- Se pide una ubicación y nombre para guardar un archivo nuevo `.fwsc`.
  No se sobrescriben archivos existentes ni se modifica el bundle del plugin.
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
