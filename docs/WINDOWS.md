# Instalar y probar Wave FM-1 Preview en Windows

Esta es una versión experimental para Windows 10/11 de 64 bits. El VST3 supera
47 pruebas del validador oficial bajo Wine 10 y el motor Windows tiene pruebas
de firmware, MIDI/audio y guardado de estado. Eso **no sustituye una prueba en
Ableton Live real** ni permite garantizar riesgo cero.

## Instalación independiente

1. Cierra Ableton Live y cualquier otro host de audio.
2. Ejecuta `Wave-FM-1-0.1.0-Windows-x64-Setup.exe` desde la descarga del proyecto.
3. El instalador copia el paquete privado en
   `%LOCALAPPDATA%\Wave FM-1 Preview\VST3` y registra **Wave FM-1 Preview**
   en Agregar o quitar programas. No necesita permisos de administrador.

El instalador no modifica Ableton, sus preferencias, proyectos, licencias,
controladores ni plugins existentes. Tampoco instala runtimes compartidos,
crea servicios, cambia PATH, añade exclusiones al antivirus ni activa el plugin.
El módulo CLAP va dentro del paquete VST3, aislado de otros módulos del equipo.
No se instala firmware y no se escribe en el FM-1 físico.

La carpeta de esta preview queda fuera de las carpetas estándar de exploración
automática: instalarla no hace que Ableton la cargue por sí solo.

## Primera prueba sin abrir proyectos importantes

Haz una copia de seguridad de tus proyectos y abre un **Set vacío**. Ningún
plugin nuevo ofrece garantía de riesgo cero al cargarse dentro de un DAW.

- Si no utilizas una carpeta VST3 personalizada, puedes seleccionar
  `%LOCALAPPDATA%\Wave FM-1 Preview\VST3` en las preferencias de plugins de
  Live para esta prueba. Anota el ajuste anterior para restaurarlo después.
- **Si ya utilizas una carpeta VST3 personalizada, no la sustituyas.** Para
  preservar tus preferencias actuales, prueba desde un perfil de Windows
  separado antes de añadir esta preview a tu configuración habitual.
- No mantengas dos versiones del mismo emulador activadas simultáneamente.
  El instrumento conserva la identidad de upstream: **FM-1**, fabricante **fm1-emu**.
  El nombre procede de upstream; esta preview tiene un identificador VST3 propio
  para no reemplazar las referencias a otro FM-1 en tus proyectos.

En el Set vacío, añade FM-1 a una pista MIDI, abre su interfaz y selecciona
**Load firmware / save…** para abrir un `.fwsc` obtenido de su autor. Activa la
monitorización y envía notas en el canal que espere ese firmware. Para el
firmware original, upstream documenta el canal 2; el plugin permite remapearlo.
Empieza con un volumen bajo. Los archivos de firmware no están incluidos.

## Desinstalación completa de esta preview

1. Cierra Live y los demás hosts de audio.
2. Abre **Configuración de Windows > Aplicaciones > Aplicaciones instaladas**
   (o **Agregar o quitar programas**).
3. Busca **Wave FM-1 Preview**, elige **Desinstalar** y sigue el asistente.

Se eliminan el paquete VST3 privado, su módulo interno, documentación, licencias
instaladas y la entrada de desinstalación. Tus proyectos de Live y los firmwares
que guardaste aparte se conservan. No se borran plugins de otros fabricantes ni
se modifican las preferencias de Live. Si habilitaste manualmente la carpeta
para la prueba, restaura tú el ajuste anterior en Live.

## Qué falta probar en Ableton

- Dos instancias con firmwares distintos, controles y ajustes independientes.
- Guardar desde el propio firmware, guardar el Set, cerrar y recuperar ambos.
- Audio a 44,1/48 kHz y buffers de 128/256/512 muestras; CPU, latencia y exportación.
- Abrir/cerrar ventanas y quitar instancias sin fallos.

Las pruebas bajo Wine no verifican tu instalación de Live, drivers ni hardware.
No pruebes esta preview por primera vez sobre un proyecto importante.

## Desarrolladores

En Windows, utiliza PowerShell 7, Python 3, Git, Rust 1.99.1, CMake >=3.27,
VS2022 C++/Windows SDK e Inno Setup 6.3+:

```powershell
./tools/build-windows.ps1
./tools/package-windows.ps1
```

La compilación cruzada usa `tools/build-windows-cross.sh` con MinGW-w64 POSIX.
Las bibliotecas de ejecución se enlazan estáticamente; se conservan las llamadas
necesarias a DLL de Windows. El instalador no requiere descargas al ejecutarse.
Los hashes SHA-256 sirven para comprobar integridad, no identidad del editor.
La preview no tiene firma Authenticode propia: no desactives SmartScreen ni el
antivirus para probarla.
