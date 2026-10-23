# Sincronizar tempo y reproducción con Ableton Live

Desde 0.2.4, **Musical clock** permite elegir por instancia:

- **Internal / firmware**: el firmware usa su tempo y sus controles de reproducción.
  Es la opción predeterminada, también para proyectos de versiones anteriores.
- **Ableton / DAW**: el plugin convierte el tempo y el transporte de Live en
  MIDI Clock de 24 pulsos por negra, Start y Stop para la entrada MIDI emulada.

## Configuración

1. Carga el firmware antes de iniciar la reproducción de Live.
2. En el firmware selecciona el reloj externo **TRS / MIDI IN**, si dispone de
   esa selección. **USB** es otra entrada y no recibe este clock emulado.
3. En la barra del plugin elige **Musical clock → Ableton / DAW**.
4. Inicia Live. Los firmwares compatibles reciben Start y siguen el tempo del
   proyecto; al detener Live reciben Stop. El estado junto al selector muestra
   el BPM del host, si está reproduciendo o si faltan datos del host.

No hace falta activar Sync en una salida MIDI física de Live: se utiliza el
transporte del dispositivo VST3. No se cambian las preferencias de Live.
**CPU clock** configura la velocidad de la CPU emulada en MHz y es independiente
del tempo musical; subirlo puede aumentar el consumo de CPU.

Felucca 1.6.1 y Skiff 1.4 ofrecen **INT / USB / TRS** en sus ajustes de reloj;
en Skiff se encuentra en **MENU → MIDI → CLOCK**. X0X 1.1.0 procesa los mensajes
Clock/Start/Continue/Stop recibidos por MIDI IN. Estas capacidades están en sus
fuentes originales incluidas. Las opciones y la respuesta de otros firmwares
pueden ser distintas; un firmware que no admita clock externo no puede seguirlo
solo por cambiar el selector del plugin.

Para volver al reloj interno, selecciona **Internal / firmware** en el plugin y
**INT / Internal** en el firmware. Al salir de la sincronización activa se envía
Stop; el plugin no cambia automáticamente ajustes internos del firmware.

## Inicio, loops y posición

Cada inicio de reproducción de Live envía Start y reinicia la secuencia del
firmware. Los saltos de posición y retornos de loop durante la reproducción
envían Stop/Start y vuelven a alinearla al clock. No se transmite Song Position
Pointer ni se recupera automáticamente un paso correspondiente al compás de
Arrangement. Si el host solo proporciona tempo, se mantiene la frecuencia del
clock sin una referencia absoluta de posición.

El tempo se actualiza a partir de la información recibida en cada bloque del
host. Los pulsos y las notas conservan sus marcas de tiempo y se programan sobre
el reloj de la máquina emulada, evitando agruparlos en los bordes de sus bloques
de audio. Se mantiene la latencia declarada del plugin. Esto no promete precisión
de una muestra en la salida: el UART MIDI, el procesamiento del firmware y su
generación de audio tienen su propia resolución temporal.

## Si el ritmo fluctúa o hay cortes

La recuperación de faltas de audio mantiene la línea de tiempo y descarta las
muestras que ya llegan tarde, en vez de retenerlas y retrasar cada vez más el
instrumento. Esto se aplica tanto al reloj interno como al modo DAW y a todos
los firmwares. No crea las muestras que un equipo no consigue calcular.

Un aviso amarillo **Audio underruns** indica muestras que faltaron. Prueba a
aumentar el tamaño de buffer de audio en Live o reducir instancias/carga de CPU.
El plugin no modifica esos ajustes por su cuenta. **MIDI queue full** indica
que falta capacidad para entregar eventos; la sincronización intenta recuperarse
con Stop/Start cuando vuelve a haber espacio.

La lectura de BPM de algunos firmwares estima intervalos MIDI y puede variar
por su resolución o filtrado. Comprueba también el ritmo grabado, no solo ese
número. Las pruebas automatizadas verifican frecuencia, deriva, entrega de
eventos, cambios de transporte y recuperación de audio; la prueba con tu Live,
controlador y carga de proyecto sigue siendo necesaria. Empieza con un Set vacío.
