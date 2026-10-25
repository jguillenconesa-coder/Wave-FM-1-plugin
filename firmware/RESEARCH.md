# Revisión de firmwares · 25-10-2026

El índice actualizado de [fm1-emu](https://github.com/joakimeriksson/fm1-emu/blob/ba11e2e68f3934a2c5c98a0eb55607387e262a49/web/firmwares.json)
aporta **96 referencias**. Se revisaron las publicaciones originales de los 23
instrumentos incluidos y los candidatos nuevos/cambiados. El explorador ofrece
**78 enlaces de autor**, con versión y SHA-256 fijados. No es un inventario
exhaustivo del Hub: su acceso directo sigue bloqueado en este entorno.

Se incorporan **cinco versiones nuevas** (Skiff 2.0.1, DUB-1 1.3.0, Chipbox
1.3.1-stable, TRIBE 0.3-beta y GammaWave 1.1.1-GW) y **SIDOU 0.4.0** como nuevo
instrumento. Los 23 paquetes anteriores se conservan. Las seis incorporaciones
tienen avisos originales, fuentes correspondientes y hashes en el manifiesto.
SIDOU incluye el submódulo libresidfp, con sus licencias y autores.

Los ocho candidatos descargados (los seis anteriores, MONK 0.9.1 y LOOM 0.7)
pasaron arranque, pantalla y ocho segundos de audio finito en la prueba inicial.
MONK y LOOM **no se empaquetan**: sigue pendiente aclarar, respectivamente,
recursos gráficos y loops incluidos. Permanecen disponibles como enlaces a sus
autores. El registro [de esta revisión](../docs/validation/firmware-review-2026-10-25.json)
identifica paquetes y alcance de las pruebas. No se atribuyen resultados anteriores
a un archivo nuevo cuyo hash cambie. Una comprobación básica no valida USB/BLE,
todas las funciones, el rendimiento a tiempo real ni sesiones largas de Live.

Los otros nuevos identificadores del índice son Clipper, Carnac, Drift, Frasil,
Tessera, Del Rio, FM1 Dock, Hera, Golana DZ y Modelar. Se muestran con su autor y
sus enlaces cuando existe un paquete FM-1 inequívoco; su redistribución no se da
por autorizada por figurar en el índice.

## Revisión anterior conservada · 23-10-2026

Se localizaron **83 candidatos** en `web/firmwares.json` del emulador fijado
`72adb810c4409c4b7302ca56906c05083834faee`, que combina un listado del Hub y
entradas propias. **No equivale a un inventario actual exhaustivo del Hub**:
The Firmware Hub, YouTube y tiendas devolvieron HTTP 403 desde este entorno.
Se consultaron los repositorios originales y los avisos de la versión concreta.
La etiqueta general del catálogo no sustituye las licencias de recursos.

**23 incluidos**: paquetes originales, hashes y fuentes completas en el manifiesto;
arranque, LCD y 352.800 frames de audio finito por paquete comprobados con el
motor Windows. Esto no prueba todas sus funciones, entrada de audio, USB/BLE,
SD, cada canal MIDI, ni estabilidad prolongada en Ableton. CTL es la variante
USB, no BLE. Firmware con funciones ligadas al hardware puede tener límites.

«Pendiente» significa que no se ha completado la autorización documental o la
prueba funcional; no significa que el autor haya cometido una infracción.
«No incluido» se refiere a esta distribución y esta versión, no a todas las
versiones futuras. La carga manual permanece disponible para archivos propios.

| Candidato y versión examinada | Procedencia | Resultado |
| --- | --- | --- |
| M-VAVE stock firmware  | [Repositorio](https://github.com/joakimeriksson/fm1-emu/blob/master/docs/stock-firmware.md) | No incluido: firmware propietario de fábrica; Open V14 incorpora condiciones de SDK limitadas al dispositivo. |
| Felucca 1.6.1 | [Repositorio](https://github.com/hugelton/Felucca) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| Felucca [Salt] 0.9-salt14 | [Repositorio](https://github.com/ChanceTheMaker/Felucca) | No incluido: documentación de esta versión reserva recursos heredados sin permiso de redistribución aclarado. |
| 386DX 1.0.2 | [Repositorio](https://github.com/seajaysec/386-DX-FM1-KaraokeBot) | Pendiente: revisión de recursos y prueba de ejecución sin completar; no se distribuye. |
| AMB-1 1.3 | [Repositorio](https://github.com/charlesvestal/fm1-amb) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| April OS BETA 1.1 | [Repositorio](https://github.com/beashoko/april-os) | Pendiente: el catálogo indica fuentes de firmware no publicadas; la licencia de la web no licencia por sí sola el firmware. |
| BEETLE 1.3.8 | [Repositorio](https://github.com/SikkenWorks/BEETLE) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Bunker Evolved 1.0.0 | [Repositorio](https://github.com/kidbee85-beep/bunker-evolved) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Bytemania 1.2.0 | [Repositorio](https://github.com/llozd/bytemania) | Pendiente: faltan permisos claros para todas las fórmulas bytebeat incorporadas. |
| Chipbox 0.4.2-alpha | [Repositorio](https://github.com/pbarilla/chipbox) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| ChoralRoot 0.26.1 | [Repositorio](https://github.com/Quixotic7/ChoralRootFM1) | No incluido: LICENSING declara presets de Casio/Sequential sin licencia concedida. |
| fm1-chord 0.3 | [Repositorio](https://github.com/math0ne/fm1-chord) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| CoralFlower 0.4.1 | [Repositorio](https://github.com/FicusFicusov/CoralFlower) | Pendiente: revisar avisos heredados de ChoralRoot y los datos concretos de esta versión. |
| CS-1 2026.10.20 | [Repositorio](https://github.com/twochannels/CS-1) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| CTL-1 1.0 | [Repositorio](https://github.com/charlesvestal/fm1-ctl) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| Cut-A-Dor 1.2.1 | [Repositorio](https://github.com/kidbee85-beep/cut-a-dor-release) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Dinghy 0.3 | [Repositorio](https://github.com/zvenson/dinghy) | No incluido: gráficos de designburg declarados con derechos reservados; requiere permiso o reconstrucción sin ellos. |
| DUB-1 1.2.0 | [Repositorio](https://github.com/sbegueria/fm1-dub) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| DUELL 0.2.20-beta | [Repositorio](https://github.com/oscillr/DUELL) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Evander's Music Box 1.5.2 | [Repositorio](https://github.com/MikeTsak/Evanders-Music-Box--FM-1-LARP-firmware) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| ETCH 1.50 | [Repositorio](https://github.com/SikkenWorks/ETCH) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Fart Machine One 1.0.0 | [Repositorio](https://github.com/adamcharleswilsonmusic-code/Fart-Machine-One) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Felucca 8 Patterns 1.6.1_and_sequences | [Repositorio](https://github.com/echoe/Felucca_8_patterns) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| FiMba-1 0.6.1 | [Repositorio](https://github.com/jadamsowers/fm1-fimba) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| Flyfish 1.0.1-4e5efb8 | [Repositorio](https://github.com/delirioux/flyfish) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| FM-106 Beta4 | [Repositorio](https://github.com/PatttF/fm-106) | No incluido: gráficos de designburg declarados con derechos reservados. |
| FM-1 Doom 0.5 | [Repositorio](https://github.com/zvenson/fm1doom) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| FM1-GM 0.9.6 | [Repositorio](https://github.com/jartificer/fm1-gm) | Pendiente: no se localizó un paquete .fwsc independiente en la publicación examinada. |
| FMEL-1 1.0 | [Repositorio](https://github.com/AntonioAltea/fmel-1) | No incluido: gráficos de designburg declarados con derechos reservados. |
| FMini-1 1.8.0 | [Repositorio](https://github.com/ruppert/fmini-1) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| FOMAJAM 0.1.3 | [Repositorio](https://github.com/konsumer/fomajam) | No incluido: LICENSING declara grabaciones de Tidal incluidas sin permiso confirmado. |
| FoMni 0.4 | [Repositorio](https://github.com/charlesvestal/fm1-fomni) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| Frodhes 0.9.4 | [Repositorio](https://github.com/leonardoMirabal/frodhes) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| FuMi-1 1.0.7 | [Repositorio](https://github.com/cartesive/fumi-1) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Pocket GammaWave 1.0-GW | [Repositorio](https://github.com/fvalente/GammaWave-FM1) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| GHOULBOX 2.3.2 | [Repositorio](https://github.com/jasonpersinger/ghoulbox-fm1-dungeon-synth) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Golana 1.2 | [Repositorio](https://github.com/Gadlele/Golana) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| GR!MM 1.0 | [Repositorio](https://github.com/GRiMMBIENT/grimm) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| Hortator 0.16.0 | [Repositorio](https://github.com/DeadActive/hortator) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| HYP-1 1.5.46 | [Repositorio](https://github.com/RustyPawz/fm1-HYP1) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Jangada 1.0.1 | [Repositorio](https://github.com/zednaked/jangada) | No incluido por ahora: falta aclarar la redistribución de recursos heredados reservados; revisar la versión exacta antes de integrar. |
| JIANT FM 0.9.9-6968b20 | [Repositorio](https://github.com/juanjiant-bit/JIANT) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| KAWA 0.5 | [Repositorio](https://github.com/Guizmoh/Kawa-Tracker-Fm1) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Ketch 0.1-beta | [Repositorio](https://github.com/risseraka/fm1-tracker-ketch) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| MAKU 1.0 | [Repositorio](https://github.com/okumurari2/maku) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| MC-1 1.2.0 | [Repositorio](https://github.com/dspaudio/mc-1) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Melodee 1.4.5 | [Repositorio](https://github.com/keremimo/melodee) | No incluido: documentación declara presets de fábrica de terceros sin licencia concedida. |
| MIN-1 0.2.1 | [Repositorio](https://github.com/Reytian/fm1-min1) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| Mini-Tone 1.0.1 | [Repositorio](https://github.com/eldontyrellcorp/Mini-Tone) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| MORPHEUS 1.0.0 | [Repositorio](https://github.com/benbadio/morpheus-fm1) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Mutable Dreams 1.0 | [Repositorio](https://github.com/kidbee85-beep/mutable-dreams) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| MYCELIUM 1.0-test | [Repositorio](https://github.com/megagott/mycelium-fm1) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Niagara Drift 0.2 | [Repositorio](https://github.com/evanyerburgh/niagara-drift) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| No Man Sky FM 0.5 | [Repositorio](https://github.com/hericdk/nomanskyFM) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| NOIZ DEMONS 666 | [Repositorio](https://github.com/SPIKE-the-Percussionist/noiz_demons) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| OberVave 1.5.3 | [Repositorio](https://github.com/zvenson/obervave) | Pendiente: el aviso URW concede excepción para PostScript/PDF, mientras el logo usa SVG/bitmap; aclarar condiciones de esos gráficos antes de distribuir. |
| OPN-1 2.0.0 | [Repositorio](https://github.com/putchdev/opn-1) | Pendiente: completar revisión y prueba con datos musicales de prueba redistribuibles. |
| ORBIT 0.7.1 | [Repositorio](https://github.com/dspaudio/orbit) | Pendiente: revisión de recursos y prueba de ejecución sin completar; no se distribuye. |
| PIGEON 0.4.1 | [Repositorio](https://github.com/TheShrimpBlimp/pigeon) | Pendiente: la documentación acredita el logo de Dendy0097 sin especificar su permiso de redistribución. |
| Polly 0.5 | [Repositorio](https://github.com/charlesvestal/fm1-polly) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| PurpleMonkey 0.11 | [Repositorio](https://github.com/Quixotic7/PurpleMonkeyFM1) | No incluido: documentación declara presets de fábrica de terceros sin licencia concedida. |
| FM1 Quest 38 | [Repositorio](https://github.com/hericdk/fm1-quest) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Rainbow Mode 1.1.5-rb | [Repositorio](https://github.com/joshbartlettband-web/felucca-rainbow) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Shine On 1.2.3 | [Repositorio](https://github.com/zvenson/shineon) | Pendiente: el aviso URW concede excepción para PostScript/PDF, mientras el logo usa SVG/bitmap; aclarar condiciones de esos gráficos antes de distribuir. |
| FM-1 SID 0.1 | [Repositorio](https://github.com/joakimeriksson/fm1-emu/tree/master/fm1-sid) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| SID-1 1.0.0 | [Repositorio](https://github.com/putchdev/sid-1) | Pendiente: completar revisión y prueba con datos SID redistribuibles. |
| Skiff 1.4 | [Repositorio](https://github.com/illuminus5/skiff) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| SLOOP 2.8 | [Repositorio](https://github.com/isod89/sloop-fm1) | No incluido por ahora: falta aclarar la redistribución de recursos heredados reservados; revisar la versión exacta antes de integrar. |
| SLOOP ALG ALG05-TEST | [Repositorio](https://github.com/shaw-core/Sloop_ALG02) | No incluido por ahora: falta aclarar la redistribución de recursos heredados reservados; revisar la versión exacta antes de integrar. |
| SLOOP DX7 Banks 2.5-b28-test | [Repositorio](https://github.com/majnikool/sloop-fm1) | No incluido por ahora: falta aclarar la redistribución de recursos heredados reservados; revisar la versión exacta antes de integrar. |
| SLOOP Floyd FM 2.4 | [Repositorio](https://github.com/renebohne/sloop-fm1) | No incluido por ahora: falta aclarar la redistribución de recursos heredados reservados; revisar la versión exacta antes de integrar. |
| sloopDX 4.0 | [Repositorio](https://github.com/zvenson/dxsloop) | No incluido por ahora: falta aclarar la redistribución de recursos heredados reservados; revisar la versión exacta antes de integrar. |
| Tonitor 2026.10.21 | [Repositorio](https://github.com/nzimas/tonitor) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| TRIBE 0.2-beta | [Repositorio](https://github.com/apiel/fm1-tribe) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| Tuna 1.0-fbc56d3 | [Repositorio](https://github.com/delirioux/tuna) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| Unholy Union 0.2.3 | [Repositorio](https://github.com/actuallysparky/FM-1-Unholy-Union-generative-drone-firmware) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| Vector37 0.3.1 | [Repositorio](https://github.com/genkigenki/Vector37) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| WaveLoop FM-1 1.0.5.2-midi23 | [Repositorio](https://github.com/ELI7VH/Felucca) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| winampFM Synth 0.12 | [Repositorio](https://github.com/hericdk/winampFM) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |
| X0X 1.1.0 | [Repositorio](https://github.com/charlesvestal/fm1-x0x) | Incluido; licencia, fuentes y pruebas de arranque/LCD/audio comprobadas. |
| X0X-RMX 1.1.0 | [Repositorio](https://github.com/Mr-Roboto/fm1-x0x-rmx) | No incluido: LICENSING deja sin resolver derechos de patrones transcritos de Future Retro Orb. |
| ZEN-1 1.1 | [Repositorio](https://github.com/aceyprime/zen-1) | Pendiente: contiene bancos comunitarios de autor desconocido o identificado, sin permiso explícito de redistribución documentado en sus generadores. |
| zp12 2.4.4 | [Repositorio](https://github.com/zvenson/zp12) | Pendiente: no se localizó LICENSING.md específico de la versión; revisar licencia de código, recursos y paquete antes de probar. |

Los avisos de los paquetes incluidos están en `licenses/<id>/`; el resto no se
copia al plugin ni se descarga en segundo plano. Para ampliar el catálogo se
necesitan un paquete identificable, licencia de código y recursos, fuentes
correspondientes cuando proceda, y pruebas reales. Las incidencias de licencia
no se resuelven asignando una licencia nueva a archivos ajenos.
