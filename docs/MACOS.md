# Wave FM-1 Preview 0.2.6 · macOS

VST3 Universal 2 para **Intel y Apple Silicon**, mínimo de compilación **macOS 11**.
Requiere un host VST3 como Ableton Live. No incluye AU. Los paquetes aparecen en
Assets de la versión solo después de compilar y validar en un runner macOS.

## Instalar y retirar

Cierra Live y conserva copia de tus proyectos. Elige un único método:

- **ZIP**: copia `macos/VST3/Wave FM-1.vst3` completo a
  `~/Library/Audio/Plug-Ins/VST3/` para tu usuario. Crea la carpeta si no existe.
  Elimina ese mismo bundle con Live cerrado para desinstalarlo.
- **PKG**: instala en `/Library/Audio/Plug-Ins/VST3/Wave FM-1.vst3` para todos
  los usuarios y requiere autorización de administrador. No contiene scripts ni
  descarga dependencias. Para retirarlo, elimina únicamente ese bundle con Live
  cerrado. El recibo puede retirarse con
  `sudo pkgutil --forget io.github.jguillenconesa.wave-fm1.preview`; esto elimina
  el recibo, no archivos, y no es necesario para dejar de cargar el plugin.

No instales ambas copias. El paquete no cambia Live, preferencias, proyectos,
controladores, permisos de seguridad ni otros plugins. Abre primero un Set vacío,
vuelve a explorar VST3 si hace falta y añade **FM-1**, fabricante **fm1-emu**.

La firma es **ad-hoc**, útil para integridad local y Apple Silicon; **no es
Developer ID ni notarización de Apple**. El PKG no lleva firma de instalador
de Apple. Gatekeeper puede bloquearlo. No desactives Gatekeeper ni elimines la
cuarentena globalmente. Si macOS no permite autorizar ese paquete de forma
individual, espera una versión firmada.

## Uso y validación

El menú **Firmware…**, la carga manual, exportación y **Skin** funcionan como
en Windows. Cada instancia conserva firmware, flash y skin con el proyecto.
Fuentes y licencias están en `Contents/Resources`. El CLAP privado está dentro
de `Contents/PlugIns`; no se instala CLAP por separado.

Las pruebas automáticas no sustituyen una prueba en Live para tu Mac, versión
del host y dispositivo de audio. Compilar ambas arquitecturas no implica haber
ejecutado ambas: las pruebas nativas usan la arquitectura del runner, indicada
en GitHub Actions. No se garantiza riesgo cero.

**Audio buffer** ofrece Low latency, Balanced y Safe / legacy. Cambia con el
transporte detenido; Safe conserva la reserva anterior y sigue siendo el valor
predeterminado. Los modos cortos pueden aumentar los cortes si falta margen de
CPU. Una instancia sin firmware declara cero latencia. Consulta
[latencia, reloj y diagnóstico](DAW_SYNC.md#latencia-y-carga-de-cpu-026).
La interfaz reutiliza controles inmóviles para consumir menos CPU.

## Compilar

En un Mac con herramientas de Xcode, Rust 1.99.1, Python 3, CMake >=3.27 y Ninja:
Para reproducir el paquete recomendado, selecciona primero la etiqueta
`v0.2.6-preview.6` (fuente `123425b`); `main` conserva la base Windows preview.4.
Ejecuta `bash tools/build-macos.sh`, las pruebas nativas de `.github/workflows/macos.yml`
y `bash tools/package-macos.sh`. No se redistribuye ningún SDK de Apple.
