# Wave FM-1 Preview 0.2.0 · Windows

VST3 de 64 bits para Windows 10/11. Conserva la identidad VST3 de la preview
0.1.0 para recuperar sus proyectos. El usuario confirmó que la versión anterior
funciona en Live. Consulta las pruebas concretas de esta versión en
`downloads/windows/VALIDATION.md`; no se garantiza riesgo cero.

## Instalar y actualizar

1. Guarda tus proyectos, haz copia de seguridad y cierra Live y otros hosts.
2. Si tienes la preview **0.1.0**, desinstálala desde **Agregar o quitar programas**.
   Conserva tus Sets y exportaciones. El nuevo instalador detecta esa instalación
   y pide retirarla para evitar dos copias del mismo VST3.
3. Ejecuta `Wave-FM-1-0.2.0-Windows-x64-Setup.exe`.
4. La ruta predeterminada es
   `C:\Program Files\Common Files\VST3\Wave FM-1.vst3`. Requiere permisos de
   administrador. Puedes cambiarla manteniendo el nombre final `Wave FM-1.vst3`.
   El instalador rechaza una carpeta existente que no gestione.
5. Abre un **Set vacío**. Con la exploración de carpetas VST3 del sistema activa,
   Live debe mostrar **FM-1**, fabricante **fm1-emu**. Si hace falta, vuelve a
   explorar plugins. No sustituyas tu carpeta personalizada existente.

El instalador escribe únicamente su bundle y su registro de desinstalación.
No edita Live, sus preferencias, licencias ni proyectos. No instala runtimes
compartidos, servicios, controladores, cambios de PATH ni exclusiones al antivirus.
Al usar la ruta estándar, Live puede descubrirlo en su siguiente escaneo.
No se flashea ningún dispositivo físico. El módulo CLAP se carga exclusivamente
desde el propio bundle.

Para el ZIP portátil, copia **todo** `VST3/FM-1.vst3` a una carpeta de plugins de
tu elección con Live cerrado. No genera entrada en Aplicaciones. No mantengas
copias simultáneas de la preview en carpetas estándar y personalizadas.

## Firmware y skin por instancia

**Firmware…** incluye [23 paquetes originales](../firmware/README.md), sin
conexión y con tamaño y SHA-256 comprobados al abrirlos. **Open firmware / save
file…** y **Load firmware / save…** conservan la carga manual `.fwsc` y `.fm1save`.
**Export save…** exporta el estado portátil.

Cambiar de firmware reinicia esa instancia y sustituye sus patches. El menú
pide confirmación si hay uno cargado. Exporta antes lo que quieras conservar.
Si falla la integridad o el arranque inicial, se conserva la configuración
anterior. La carga manual permite versiones modificadas fuera del catálogo.

**Skin** ofrece los seis acabados de la referencia: Graphite, Ivory / charcoal,
Orange, Lilac, Ivory / blue y Graphite / turquoise. Son paletas dibujadas, sin
incluir la fotografía del fabricante. Cambian carcasa, teclas y controles;
pantalla y LED conservan sus colores. La selección se guarda por instancia.
En firmwares SMK-37 manuales se conserva el panel SMK-37 sin estas skins.

Empieza a volumen bajo y envía MIDI al canal esperado. El plugin permite
remapearlo. Guarda los cambios desde el firmware antes de guardar el Set:
se conservan firmware y flash, no toda la RAM. Fuentes, créditos y licencias
están en `Contents/Resources/` dentro del bundle.

## Desinstalar

Cierra Live. En **Configuración > Aplicaciones > Aplicaciones instaladas** o
**Agregar o quitar programas**, elige **Wave FM-1 Preview > Desinstalar**.
Elimina sus binarios, firmwares incluidos, fuentes, licencias, documentación y
registro. Conserva tus proyectos y exportaciones guardados aparte. Los archivos
personales añadidos manualmente al bundle no se borran con comodines.
Para el ZIP, elimina solamente la carpeta del bundle que copiaste.

## Desarrollo y límites

PowerShell 7, Python 3, Git, Rust 1.99.1, CMake >=3.27, VS2022 C++/Windows SDK
e Inno Setup 7.1.0: `./tools/build-windows.ps1` y `./tools/package-windows.ps1`.
La compilación cruzada está en `tools/build-windows-cross.sh`.

La preview no tiene firma Authenticode propia. Los hashes comprueban integridad,
no identidad del editor. No desactives las protecciones del sistema. Los
firmwares y el motor se ejecutan dentro del DAW; no son un sandbox. Usa archivos
de confianza. Prueba dos instancias, guardar/reabrir y tu configuración de audio
antes de abrir un proyecto importante con la actualización.
