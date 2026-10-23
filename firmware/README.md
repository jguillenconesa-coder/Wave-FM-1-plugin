# Firmwares incluidos y procedencia

Catálogo sin conexión de Wave FM-1 Preview 0.2.0. Los archivos originales se
distribuyen como programas separados, sin modificaciones, junto a sus fuentes
y avisos. No se descarga ni ejecuta código del sistema operativo al elegirlos.
El emulador interpreta el firmware dentro del proceso del host; no es un sandbox.

| Firmware | Autor | Uso | Licencia de esta versión |
| --- | --- | --- | --- |
| [Felucca 1.6.1](https://github.com/hugelton/Felucca/releases/tag/v1.6.1) | Leo Kuroshita / Hügelton Instruments | Sintetizador multiengine y secuenciador | GPL-3.0-only |
| [X0X 1.1.0](https://github.com/charlesvestal/fm1-x0x/releases/tag/v1.1.0) | Charles Vestal y colaboradores | Bajos 303, baterías 808/909 y breaks | GPL-3.0-only |
| [AMB-1 1.3](https://github.com/charlesvestal/fm1-amb/releases/tag/v1.3) | Charles Vestal y colaboradores | Ambient generativo | MPL-2.0 |
| [FoMni-1 0.4](https://github.com/charlesvestal/fm1-fomni/releases/tag/v0.4) | Charles Vestal y colaboradores; Chordian de Jan125 | Arpa de acordes y ritmos | GPL-3.0-only |
| [Polly 0.5](https://github.com/charlesvestal/fm1-polly/releases/tag/v0.5) | Charles Vestal; Sonivox EAS; Google Magenta | Síntesis General MIDI y acompañamiento | GPL-3.0-only; plataforma MPL-2.0; EAS Apache-2.0; ritmos CC BY 4.0 |
| [Skiff 1.4](https://github.com/illuminus5/skiff/releases/tag/v1.4) | illuminus5; Hügelton Instruments y colaboradores | Groovebox de seis pistas y múltiples motores | GPL-3.0-only |
| [DUB-1 1.2.0](https://github.com/sbegueria/fm1-dub/releases/tag/v1.2.0) | sbegueria; Charles Vestal y colaboradores | Dub techno generativo | GPL-3.0-only |


`manifest.json` fija versión, revisión de fuente, URL original, tamaño y SHA-256
del paquete y de sus fuentes. Las licencias de recursos, bibliotecas, muestras,
tipografías y SDK se conservan en `licenses/<id>/`. Consulta sus `LICENSING.md`
y `CREDITS.md`, que acreditan también a los autores de componentes anteriores.
Estos programas no son productos oficiales ni están avalados por M-VAVE,
Ableton, Roland o Suzuki. Sus marcas corresponden a sus propietarios.

## Fuentes completas

`sources/` contiene las fuentes correspondientes a cada versión. Felucca usa
el archivo completo publicado por su autor, con Felucca-Engines incluido.
AMB-1 y Polly contienen también sus submódulos fm1-base y felucca-bsp en sus revisiones
fijadas; no se usa el ZIP incompleto generado automáticamente por GitHub.
X0X, FoMni-1, Skiff y DUB-1 incluyen sus scripts y recursos del tag original.
De Polly solo se omiten los vídeos de demostración `docs/video`; no son entradas
de compilación. Se incluyen los datos Sonivox EAS y el README, licencia, autores
y descripción de modificaciones de los ritmos de Google Magenta (CC BY 4.0).
Los requisitos de compilación y la obtención del toolchain y SDK externos se
explican en los `BUILDING.md` de cada archivo. Los archivos de fuentes conservan
sus licencias originales; la licencia MIT del empaquetador no las sustituye.

El instalador y el ZIP incluyen estas mismas fuentes en
`Contents/Resources/firmware-sources`, las licencias en `firmware-licenses` y el
manifiesto en `firmware-manifest.json`. También están disponibles gratuitamente
en este repositorio junto a los binarios. No se restringe el uso de versiones
modificadas: **Load firmware / save…** permite abrir un paquete propio sin
exigir que figure en el catálogo ni que tenga el hash de uno de los incluidos.

## Comprobación y selección

El catálogo exige coincidencia exacta de tamaño y SHA-256 antes de importar.
El cambio usa el mismo arranque validado y almacenamiento por instancia que
la carga manual. Sustituir un firmware reinicia esa instancia y sus patches;
exporta primero si quieres conservarlos. Un archivo `.fm1save` o un proyecto
conserva los bytes del firmware para restaurarlo sin depender del menú.

Las pruebas de arranque, LCD y audio se ejecutan con los paquetes reales, no
con sustitutos. La validación concreta y sus límites se publican en
`downloads/windows/VALIDATION.md`; no implican haber probado todas las funciones
del firmware ni certificación del fabricante.

Investigación del 23-10-2026: se consultaron las publicaciones originales y el
[directorio de aroum](https://github.com/aroum/fm1-custom-fw). La búsqueda de
YouTube `firmware Wave-FM1` y la demo de Felucca enlazada por ese directorio
devolvieron HTTP 403 desde el entorno; no se afirma haber visto los vídeos.
The Firmware Hub y ambas búsquedas de YouTube aportadas por el usuario devolvieron
HTTP 403 desde el entorno. El catálogo del emulador sirvió para localizar
candidatos del Hub; las comprobaciones se hicieron sobre publicaciones originales.
Esta selección no es una auditoría exhaustiva de todos los proyectos del Hub.
SLOOP 2.8 y Dinghy 0.3 se excluyen por la incertidumbre sobre recursos
identificados como derechos reservados en su documentación. ChoralRoot 0.26.1
se excluye porque declara presets de Casio y Sequential sin licencia concedida.
El firmware oficial Open V14 contiene SDK con restricciones de dispositivo;
no se distribuye como si fuera enteramente GPL sin restricciones. No se incluyen
firmwares propietarios, de pago, con licencia ausente ni el firmware de fábrica.
