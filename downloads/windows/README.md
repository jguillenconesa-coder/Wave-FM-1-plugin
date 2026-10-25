# Windows Preview 0.2.8

Incluye recuperación tras interrupciones sin recargar el firmware, referencias de
knob 0–127 compartidas entre ratón/MIDI, y correcciones del reloj de Live: arranque
al cargar un firmware y resincronización de loops sin la colisión Stop/Start de X0X.
Biblioteca: 29 versiones de 24 instrumentos; 96 referencias y 78 enlaces de autor.

- [Instalador Windows x64](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/releases/download/v0.2.8-preview.3/Wave-FM-1-0.2.8-Windows-x64-Setup.exe)
- [ZIP portátil](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/releases/download/v0.2.8-preview.3/Wave-FM-1-0.2.8-Windows-x64-Portable.zip)
- [Archivos, fuentes y hashes de esta versión](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/releases/tag/v0.2.8-preview.3)
- [Instalación/desinstalación](../../docs/WINDOWS.md) · [Pruebas y límites](VALIDATION.md)

Fuente del binario: `29835a041d102eca7bf0da6dc57eca07629c7fc5`. Los archivos descargados se han comprobado contra
SHA256SUMS y el manifiesto completo del bundle. La prueba nativa instala el EXE,
carga su factory y elimina su instalación y registro. No modifica Live ni sus
proyectos. Instala con Live cerrado y prueba primero en un Set vacío.

Los proyectos conservan los bytes del firmware con que se guardaron. Las versiones
anteriores del catálogo siguen disponibles; cambiar de firmware es una acción
explícita que sustituye los patches de esa instancia. Se conserva también el
[instalador 0.2.5](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/releases/download/v0.2.5-preview.3/Wave-FM-1-0.2.5-Windows-x64-Setup.exe)
como referencia anterior. Los instaladores se alojan en Releases; no se usan
enlaces raw a ejecutables.
