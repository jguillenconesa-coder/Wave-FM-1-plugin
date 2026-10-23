# Wave FM-1 Preview para Windows

Plugin **VST3 de 64 bits** que emula el M-VAVE FM-1 y permite cargar firmwares
`.fwsc`. Se basa en [fm1-emu de Joakim Eriksson](https://github.com/joakimeriksson/fm1-emu):
el motor, el panel interactivo y la pantalla del firmware proceden de ese proyecto.
Los créditos y licencias originales se conservan.

## Descargar e instalar

- [Instalador Windows](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/releases/download/v0.1.0-preview.1/Wave-FM-1-0.1.0-Windows-x64-Setup.exe)
- [Paquete portátil](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/releases/download/v0.1.0-preview.1/Wave-FM-1-0.1.0-Windows-x64-Portable.zip)
- [Hashes SHA-256](downloads/windows/SHA256SUMS)
- [Versiones publicadas](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/releases)

Si la descarga directa falla, abre [la página de esta versión](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/releases/tag/v0.1.0-preview.1)
y selecciona el archivo `.exe` en **Assets**.

El instalador coloca **Wave FM-1 Preview** en tu carpeta de usuario, sin permisos
administrativos, y ofrece desinstalación desde **Agregar o quitar programas**.
No modifica la instalación de Ableton, sus preferencias, proyectos ni otros
plugins. Tampoco activa el plugin en Live ni cambia sus carpetas de exploración.

**Es una preview experimental, no una versión validada en Ableton Live.**
El paquete Windows supera las 47 pruebas del validador oficial bajo Wine 10,
pero ningún plugin nuevo puede prometer riesgo cero dentro de un DAW. Prueba
primero con un Set vacío y tus proyectos importantes cerrados y respaldados.

Sigue [las instrucciones de instalación, prueba y desinstalación](docs/WINDOWS.md).
La carpeta es `%LOCALAPPDATA%\Wave FM-1 Preview\VST3`.
Si ya usas una carpeta VST3 personalizada, **no la sustituyas**: prueba desde
un perfil Windows separado para mantener tus preferencias actuales.

El instrumento aparece como **FM-1**, fabricante **fm1-emu**. Su panel conserva
los controles y la pantalla emulada del aparato. Cada instancia tiene su propio
motor, firmware y estado. No se garantiza compatibilidad con todos los firmwares
ni un número ilimitado de instancias; su consumo de CPU varía.

No se incluye firmware. Puedes obtener, por ejemplo,
[Felucca 1.6.1 de su autor](https://github.com/hugelton/Felucca/releases/tag/v1.6.1)
y abrir el `.fwsc` desde **Load firmware / save…**. Los firmwares tienen sus
propias licencias. Guarda los cambios desde el propio firmware antes de guardar
el Set: el estado conserva firmware y flash, no una captura completa de la RAM.

## Desarrollo y compilación

Requisitos Windows: Git, Python 3, PowerShell 7, Rust 1.99.1, CMake >=3.27,
Visual Studio 2022 C++/Windows SDK e Inno Setup 7.1.0.

```powershell
./tools/build-windows.ps1
./tools/package-windows.ps1
```

También hay una compilación cruzada Linux/MinGW-w64 POSIX en
`tools/build-windows-cross.sh`. `cmake/windows-mingw.cmake` configura Windows
x64. El emulador se descarga en `.deps/fm1-emu` con revisión fija, sin modificar
sus fuentes. El wrapper recibe un parche local verificado por hash para cargar
únicamente su módulo CLAP privado. El bundle VST3 incluye ambos binarios y las
bibliotecas de ejecución se enlazan estáticamente.

Las pruebas de integridad propias se ejecutan con:

```sh
python3 -m unittest discover -s tests -v
```

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
