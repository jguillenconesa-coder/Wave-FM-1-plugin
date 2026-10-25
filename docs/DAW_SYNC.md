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

Desde 0.2.5, la franja inferior muestra siempre **Engine**, **Audio late**,
**CPU** y la frecuencia/bloque del **Host**. **Copy audio status** copia esos
datos junto con la versión y el firmware. **Audio late** mide la cola de audio
del plugin, no el contador de errores del controlador ASIO. Puede aumentar
brevemente al arrancar o cargar un firmware; el retraso persistente requiere
investigación. **Engine** inferior a 1.00x indica sobrecarga sostenida; un
promedio superior a 1.00x no descarta pausas breves del sistema.

La emulación entrega audio en tandas más cortas y la recuperación tiene un
presupuesto proporcional al bloque de audio. No modifica el buffer de Live ni
añade latencia. Para diagnosticar, empieza con **CPU clock: 240 MHz**: subir ese
valor aumenta el trabajo del ordenador, aunque pueda ayudar al firmware con
sus propios plazos. Ante sobrecarga, reduce otras cargas/instancias o prueba
un buffer mayor. El plugin no modifica esos ajustes por su cuenta.
**MIDI queue full** indica
que falta capacidad para entregar eventos; la sincronización intenta recuperarse
con Stop/Start cuando vuelve a haber espacio.

La lectura de BPM de algunos firmwares estima intervalos MIDI y puede variar
por su resolución o filtrado. Comprueba también el ritmo grabado, no solo ese
número. Las pruebas automatizadas verifican frecuencia, deriva, entrega de
eventos, cambios de transporte y recuperación de audio; la prueba con tu Live,
controlador y carga de proyecto sigue siendo necesaria. Empieza con un Set vacío.

## Latencia y carga de CPU (0.2.6)

**Audio buffer** elige la reserva del plugin. A 48 kHz / bloque máximo 512,
**Low latency** declara 582 muestras (12,1 ms), **Balanced** 1070 (22,3 ms) y
**Safe / legacy** 1627 (33,9 ms). Una instancia vacía declara cero. El cálculo
usa el bloque máximo del host, que puede diferir del último bloque observado.
Esta cifra no incluye el buffer ASIO ni la respuesta del firmware.

Safe sigue siendo el valor predeterminado. Detén el transporte para cambiarlo,
dejando que se estabilicen la cola y la compensación del host. Los modos cortos
pueden aumentar los underruns: una reserva menor no corrige una emulación que
solo alcanza 1.00x. Empieza por Balanced y comprueba **Audio late** antes de
probar Low latency. El plugin no cambia el buffer de Live.

El worker solicita prioridad «Above normal» en Windows y QoS en macOS, solo para
su hilo. La interfaz también evita redibujar controles inmóviles. Estas medidas
reducen pausas y trabajo ajeno al motor; su efecto depende del sistema. La CPU
virtual de una instancia no se reparte automáticamente entre todos los núcleos
del ordenador. Elevar **CPU clock** exige más trabajo al emulador; no equivale
a concederle más recursos del ordenador. El algoritmo de sonido y el resampler
se mantienen: no se promete una mejora de fidelidad ni se activa oversampling.

La versión 0.2.7 corrige la deuda de audio creada durante el arranque sin DMA.
Antes, una producción posterior a 1.00x podía mantener el descarte indefinido.
El productor prepara únicamente el silencio de arranque y conserva el primer
bloque real; no oculta faltas posteriores ni modifica el tempo. X0X se comprueba
ahora a 48 kHz / 64 y 512 muestras. Engine y Audio late, por sí solos, no
establecen que el procesador del usuario sea insuficiente. Los resultados y
límites de cada paquete están en el registro de validación.

## Recuperación después de un error (0.2.8)

Una interrupción que agota la reserva inicia un acuerdo de pausa entre la cola
y el emulador. El plugin descarta el audio obsoleto y prepara una nueva reserva
sin recargar el firmware ni borrar patches. Puede haber un breve silencio durante
esta operación. El reloj interno del firmware sigue su estado actual; en modo
DAW se envía Stop/Start para alinear de nuevo el transporte.

**Recoveries** cuenta recuperaciones terminadas; **rebuffering** identifica una
en curso. **Firmware halted** distingue una parada real del emulador, que no se
soluciona solo vaciando una cola. Estos datos se incluyen en Copy audio status.
Un contador de recuperaciones que siga aumentando indica un problema persistente.
