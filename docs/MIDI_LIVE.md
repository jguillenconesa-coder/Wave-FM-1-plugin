# Identificar y asignar controles en Ableton Live

Abre **MIDI / Live** en la barra del plugin. El panel añade etiquetas de alto
contraste y la columna derecha muestra los controles que Live puede asignar.
Los nombres son iguales en ambos lugares: **Master**, **Knob 1–4**, **Select**,
**Presets**, **Algorithm** y los 14 botones, nombrados como **FX button**,
**PLAY button**, etc. Los nombres identifican controles físicos; cada firmware
y su pantalla actual deciden qué hacen. No se promete un control fijo de filtro
o envolvente para todos los firmwares.

1. En las preferencias MIDI de Live, activa **Remote** para la entrada del
   controlador que quieras usar. **Track** permite enviar notas al instrumento.
2. En el dispositivo FM-1 de Live, abre **Configure**.
3. Mueve un deslizador o pulsa un botón en la columna **MIDI / Live** del plugin.
   Live añadirá ese parámetro a su dispositivo. También puedes mover un knob del dibujo: ambos usan el mismo parámetro.
4. Desactiva Configure y entra en **MIDI Map** con **Ctrl+M** en Windows o
   **Cmd+M** en macOS. Selecciona el parámetro que apareció en Live y mueve el
   knob o pulsa el botón de tu controlador. Sal de MIDI Map.

Para los siete encoders, configura el controlador como **CC absoluto 0–127** y
usa el modo absoluto correspondiente en Live. Los números son una referencia
de movimiento: pasar de 64 a 65 envía un clic horario; volver a 64 envía uno
antihorario. No representan el valor del parámetro del firmware. Los cambios
rápidos entre dos lecturas del motor se agrupan por su diferencia neta. Desde **0.2.8**, el panel manual y MIDI comparten esa referencia guardada
en el Set, con límites 0 y 127 y sin vuelta circular. El valor inicial es 64.
Un arrastre normal usa dos píxeles por paso; **Shift** permite ajuste fino.
**Alt-arrastre/Alt-rueda** conserva el giro relativo sin fin para recorrer menús
largos, y no cambia la referencia absoluta. La pantalla del firmware sigue
siendo la fuente de su valor musical real. La opción Takeover de Live puede ayudar a evitar saltos al conectar
un controlador cuya posición física no coincide.

Desde **0.2.3**, los indicadores del dibujo siguen esas referencias del host:
0 apunta abajo a la izquierda, 64 aproximadamente hacia arriba y 127 abajo a
la derecha, con un recorrido de 270°. Se actualizan los siete encoders, incluidos
**Knob 1–4**, tanto con MIDI asignado como con automatización o los deslizadores
de la columna. Al reabrir la ventana se recuperan sus posiciones actuales.
El ratón y los cambios del host colocan el indicador en esa misma referencia.
Volver desde un límite responde inmediatamente, sin acumular giros fuera de rango.
La referencia se conserva al cerrar la interfaz y al guardar/restaurar el Set.
No permite leer automáticamente el valor musical interno de todos los firmwares.

Los botones asignables generan un pulso de **100 ms** al pasar de Off a On y
se liberan automáticamente. Usa botones momentáneos que envíen **127 al pulsar
y 0 al soltar**. Un controlador que solo mande On debe volver a Off antes de
otra activación. No son controles de secuenciación con precisión de muestra;
los pulsos muy próximos pueden agruparse. Para mantener un botón o usar una
pulsación larga, utiliza el panel físico dibujado.

Las asignaciones las guarda Live en el Set. Los IDs previos de Master, MIDI
channel y CPU clock y la identidad VST3 se conservan; se añaden 21 parámetros.
Cada instancia tiene referencias y pulsos independientes. Cambiar de firmware
establece de nuevo las referencias, sin recorrer todas sus posiciones. Los
controles FM-1 adicionales están desactivados con un firmware SMK-37 importado.

Los controles asignables se aplican cuando el host procesa el instrumento,
también con el transporte detenido si Live mantiene activa la pista. Al recuperar
un Set se publica una referencia coherente después de restaurar sus parámetros:
los valores guardados no se interpretan como nuevos giros o pulsaciones. No se
leen parámetros a medio restaurar desde el hilo del emulador.

No se ha automatizado una sesión real de Live desde este entorno: las pruebas
comprueban las rutas del motor y los parámetros VST3; confirma el mapeo con tu
controlador en un Set vacío antes de incorporarlo a un proyecto importante.
