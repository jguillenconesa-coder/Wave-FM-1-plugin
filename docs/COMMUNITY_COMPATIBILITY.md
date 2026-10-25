# Revisión actual · 25-10-2026

La biblioteca actual tiene **96 referencias y 78 enlaces directos de autor**.
Los hashes y versiones de cada enlace están en `firmware/community-index.json`.
Se eliminaron los resultados básicos antiguos cuando cambió el archivo: un enlace
nuevo no hereda la validación del anterior. Se comprobaron los ocho candidatos
de la [revisión actual](validation/firmware-review-2026-10-25.json).
Los demás enlaces nuevos o actualizados que carecen de resultado no se anuncian
como funcionalmente verificados. El instalador incluye solo las **29 versiones**
documentadas en [el catálogo](../firmware/README.md).

El siguiente registro es histórico: sus versiones y hashes son los comprobados
el 23-10-2026; no describe necesariamente el archivo que se ofrece hoy.

# Descargas alternativas y comprobación básica · 23-10-2026

Se descargaron **64 paquetes FM-1 originales de GitHub** fuera del árbol
publicado. **62 pasaron** arranque, pantalla y ocho segundos de audio finito.
**BEETLE y PIGEON quedaron en silencio con esta secuencia de prueba**: no se
consideran funcionalmente verificados; eso no demuestra que estén averiados,
pues pueden necesitar secuencias o archivos musicales adicionales.

Los 83 proyectos del índice no equivalen al catálogo actual del Hub. Sus
19 referencias restantes no tenían una publicación `.fwsc` FM-1 identificable
por este método. No se afirma haber implementado todos los firmwares del Hub.

Se comprobaron 352.800 frames por paquete, pantalla con más de 1000 píxeles
no nulos y ausencia de parada de CPU. Se enviaron notas MIDI en 16 canales,
PLAY y una nota mantenida del panel; Ketch usa REC para habilitar la audición.
Se ejecutaron cuatro motores independientes simultáneos en Windows x64 bajo
Wine 10. No se han verificado todas las funciones, sesiones largas ni un Set
real de Ableton. Los resultados identifican exactamente versión, URL y SHA-256
en [el registro JSON](validation/community-2026-10-23.json).

Los binarios externos **no se redistribuyen** dentro del plugin/instalador.
El explorador ofrece descargas directas de los autores y muestra sus condiciones.
Un resultado técnico satisfactorio no concede permiso para redistribuir sus
muestras, presets, gráficos o código. Créditos conservados por cada referencia.

| Firmware | Publicación del autor | Resultado básico |
| --- | --- | --- |
| Felucca · hugelton | [v1.6.1](https://github.com/hugelton/Felucca/releases/download/v1.6.1/felucca-1.6.1.fwsc) | Arranque/LCD/audio: pasa |
| Felucca [Salt] · ChanceTheMaker | [v0.9-salt14](https://github.com/ChanceTheMaker/Felucca/releases/download/v0.9-salt14/felucca-0.9-salt14.fwsc) | Arranque/LCD/audio: pasa |
| 386DX · seajaysec | [v1.0.2](https://github.com/seajaysec/386-DX-FM1-KaraokeBot/releases/download/v1.0.2/386dx-1.0.2.fwsc) | Arranque/LCD/audio: pasa |
| AMB-1 · charlesvestal | [v1.3](https://github.com/charlesvestal/fm1-amb/releases/download/v1.3/amb-1.3.fwsc) | Arranque/LCD/audio: pasa |
| BEETLE · Joppe Sikken (SikkenWorks) | [V1.4.25](https://github.com/SikkenWorks/BEETLE/releases/download/V1.4.25/BEETLE-V1.4.25.fwsc) | Sin audio con esta prueba |
| Bunker Evolved · Kid Bunka | [v1.0.0](https://github.com/kidbee85-beep/bunker-evolved/releases/download/v1.0.0/bunker-evolved-rc2.fwsc) | Arranque/LCD/audio: pasa |
| Bytemania · lloyd (llozd) | [v1.2.0](https://github.com/llozd/bytemania/releases/download/v1.2.0/bytemania-1.2.0.fwsc) | Arranque/LCD/audio: pasa |
| Chipbox · Pasquale Barilla | [v1.2.4](https://github.com/pbarilla/chipbox/releases/download/v1.2.4/chipbox-1.2.4.fwsc) | Arranque/LCD/audio: pasa |
| ChoralRoot · Quixotic7 | [v0.26.1](https://github.com/Quixotic7/ChoralRootFM1/releases/download/v0.26.1/choralroot-0.26.1.fwsc) | Arranque/LCD/audio: pasa |
| fm1-chord · math0ne | [v0.3](https://github.com/math0ne/fm1-chord/releases/download/v0.3/fm1-chord-0.3.fwsc) | Arranque/LCD/audio: pasa |
| CoralFlower · FicusFicusov | [v0.5](https://github.com/FicusFicusov/CoralFlower/releases/download/v0.5/coralflower-0.5.fwsc) | Arranque/LCD/audio: pasa |
| CS-1 · twochannels | [v0.3.2](https://github.com/twochannels/CS-1/releases/download/v0.3.2/cs1-0.3.2.fwsc) | Arranque/LCD/audio: pasa |
| CTL-1 · charlesvestal | [v1.0](https://github.com/charlesvestal/fm1-ctl/releases/download/v1.0/ctl-usb-1.0.fwsc) | Arranque/LCD/audio: pasa |
| Cut-A-Dor · Kid Bunka | [v1.2.1](https://github.com/kidbee85-beep/cut-a-dor-release/releases/download/v1.2.1/Cut-A-Dor-1.2.1-Sensei-FM1.fwsc) | Arranque/LCD/audio: pasa |
| Dinghy · zvenson | [v0.3](https://github.com/zvenson/dinghy/releases/download/v0.3/dinghy-0.3.fwsc) | Arranque/LCD/audio: pasa |
| DUB-1 · sbegueria | [v1.3.0](https://github.com/sbegueria/fm1-dub/releases/download/v1.3.0/dub-1.3.0.fwsc) | Arranque/LCD/audio: pasa |
| DUELL · OSCILL/R | [v1.0](https://github.com/oscillr/DUELL/releases/download/v1.0/duell-1.0.fwsc) | Arranque/LCD/audio: pasa |
| Evander's Music Box · Michail Tsakiroglou | [v1.5.2](https://github.com/MikeTsak/Evanders-Music-Box--FM-1-LARP-firmware/releases/download/v1.5.2/emb.fwsc) | Arranque/LCD/audio: pasa |
| ETCH · SikkenWorks | [V1.61](https://github.com/SikkenWorks/ETCH/releases/download/V1.61/ETCH-V1.61.fwsc) | Arranque/LCD/audio: pasa |
| Fart Machine One · adamcharleswilsonmusic-code | [V1.1.0](https://github.com/adamcharleswilsonmusic-code/Fart-Machine-One/releases/download/V1.1.0/fart-machine-one.fwsc) | Arranque/LCD/audio: pasa |
| FiMba-1 · jadamsowers | [v0.6.1](https://github.com/jadamsowers/fm1-fimba/releases/download/v0.6.1/fimba-0.6.1.fwsc) | Arranque/LCD/audio: pasa |
| FM1-GM · jartificer | [v1.0.1](https://github.com/jartificer/fm1-gm/releases/download/v1.0.1/fm1gm.fwsc) | Arranque/LCD/audio: pasa |
| FMEL-1 · Antonio Altea | [v1.0](https://github.com/AntonioAltea/fmel-1/releases/download/v1.0/mel-1.0.fwsc) | Arranque/LCD/audio: pasa |
| FMini-1 · Ruppert | [v1.8.0](https://github.com/ruppert/fmini-1/releases/download/v1.8.0/FMini-1-v1.8.0.fwsc) | Arranque/LCD/audio: pasa |
| FOMAJAM · konsumer | [v0.1.6](https://github.com/konsumer/fomajam/releases/download/v0.1.6/fomajam-0.1.fwsc) | Arranque/LCD/audio: pasa |
| FoMni · charlesvestal | [v0.4](https://github.com/charlesvestal/fm1-fomni/releases/download/v0.4/omni-0.4.fwsc) | Arranque/LCD/audio: pasa |
| Frodhes · Leonardo Mirabal | [v0.9.4](https://github.com/leonardoMirabal/frodhes/releases/download/v0.9.4/frodhes-0.9.4.fwsc) | Arranque/LCD/audio: pasa |
| FuMi-1 · cartesive | [fumi-1.1.0](https://github.com/cartesive/fumi-1/releases/download/fumi-1.1.0/fumi-1.1.0.fwsc) | Arranque/LCD/audio: pasa |
| Pocket GammaWave · ShadowTune (GammaWave Audio) | [v1.0-GW](https://github.com/fvalente/GammaWave-FM1/releases/download/v1.0-GW/felucca-1.0-GW.fwsc) | Arranque/LCD/audio: pasa |
| GHOULBOX · jasonpersinger | [ghoulbox-v2.3.2](https://github.com/jasonpersinger/ghoulbox-fm1-dungeon-synth/releases/download/ghoulbox-v2.3.2/ghoulbox-2.3.2.fwsc) | Arranque/LCD/audio: pasa |
| Golana · Gadlele | [golana-v1.2](https://github.com/Gadlele/Golana/releases/download/golana-v1.2/Golana-1.2.fwsc) | Arranque/LCD/audio: pasa |
| GR!MM · GRiMMBIENT | [v1.0](https://github.com/GRiMMBIENT/grimm/releases/download/v1.0/grimm.fwsc) | Arranque/LCD/audio: pasa |
| Hortator · DeadActive | [drum-v0.23.2](https://github.com/DeadActive/hortator/releases/download/drum-v0.23.2/hortator-0.23.2.fwsc) | Arranque/LCD/audio: pasa |
| Jangada · zednaked | [v1.0.1](https://github.com/zednaked/jangada/releases/download/v1.0.1/jangada-1.0.1.fwsc) | Arranque/LCD/audio: pasa |
| Ketch · risseraka | [v0.1-beta](https://github.com/risseraka/fm1-tracker-ketch/releases/download/v0.1-beta/ketch-0.1-beta.fwsc) | Arranque/LCD/audio: pasa |
| MAKU · okumurari | [maku-v1.0](https://github.com/okumurari2/maku/releases/download/maku-v1.0/maku-1.0.fwsc) | Arranque/LCD/audio: pasa |
| MC-1 · dspaudio | [v1.2.0](https://github.com/dspaudio/mc-1/releases/download/v1.2.0/mc-1-1.2.0.fwsc) | Arranque/LCD/audio: pasa |
| Melodee · keremimo | [v1.4.5](https://github.com/keremimo/melodee/releases/download/v1.4.5/melodee-1.4.5.fwsc) | Arranque/LCD/audio: pasa |
| MIN-1 · Reytian | [v0.2.1](https://github.com/Reytian/fm1-min1/releases/download/v0.2.1/min-0.2.1.fwsc) | Arranque/LCD/audio: pasa |
| Mini-Tone · Stéphane Schmitz | [minitone-1.0.2](https://github.com/eldontyrellcorp/Mini-Tone/releases/download/minitone-1.0.2/minitone-1.0.2.fwsc) | Arranque/LCD/audio: pasa |
| MYCELIUM · Mandalars | [v1.0-test](https://github.com/megagott/mycelium-fm1/releases/download/v1.0-test/Mycelium-FM1-1.0.fwsc) | Arranque/LCD/audio: pasa |
| Niagara Drift · evanyerburgh | [v0.2](https://github.com/evanyerburgh/niagara-drift/releases/download/v0.2/niagara-drift-0.2.fwsc) | Arranque/LCD/audio: pasa |
| NOIZ DEMONS · SPIKE the Percussionist | [v666](https://github.com/SPIKE-the-Percussionist/noiz_demons/releases/download/v666/noiz_demons.fwsc) | Arranque/LCD/audio: pasa |
| OberVave · zvenson | [v1.5.3](https://github.com/zvenson/obervave/releases/download/v1.5.3/obervave-1.5.3.fwsc) | Arranque/LCD/audio: pasa |
| OPN-1 · putchdev | [v2.0.0](https://github.com/putchdev/opn-1/releases/download/v2.0.0/opn1-2.0.0.fwsc) | Arranque/LCD/audio: pasa |
| ORBIT · dspaudio | [v0.7.1](https://github.com/dspaudio/orbit/releases/download/v0.7.1/orbit.fwsc) | Arranque/LCD/audio: pasa |
| PIGEON · The Shrimp Blimp | [v0.5.0-beta.1](https://github.com/TheShrimpBlimp/pigeon/releases/download/v0.5.0-beta.1/pigeon-0.5.0-beta.1.fwsc) | Sin audio con esta prueba |
| Polly · charlesvestal | [v0.5](https://github.com/charlesvestal/fm1-polly/releases/download/v0.5/polly-0.5.fwsc) | Arranque/LCD/audio: pasa |
| PurpleMonkey · Quixotic7 | [v0.11](https://github.com/Quixotic7/PurpleMonkeyFM1/releases/download/v0.11/purplemonkey-0.11.fwsc) | Arranque/LCD/audio: pasa |
| FM1 Quest · hericdk | [quest-38](https://github.com/hericdk/fm1-quest/releases/download/quest-38/fm1-quest-38.fwsc) | Arranque/LCD/audio: pasa |
| Shine On · zvenson | [v1.2.3](https://github.com/zvenson/shineon/releases/download/v1.2.3/shineon-1.2.3.fwsc) | Arranque/LCD/audio: pasa |
| SID-1 · putchdev | [v1.0.0](https://github.com/putchdev/sid-1/releases/download/v1.0.0/sid1-1.0.0.fwsc) | Arranque/LCD/audio: pasa |
| Skiff · illuminus5 | [v1.4](https://github.com/illuminus5/skiff/releases/download/v1.4/skiff-1.4.fwsc) | Arranque/LCD/audio: pasa |
| SLOOP · isod89 | [v2.8.1](https://github.com/isod89/sloop-fm1/releases/download/v2.8.1/sloop-2.8.1.fwsc) | Arranque/LCD/audio: pasa |
| SLOOP DX7 Banks · majnikool | [v2.5-b28-test](https://github.com/majnikool/sloop-fm1/releases/download/v2.5-b28-test/sloop-2.5-b28.fwsc) | Arranque/LCD/audio: pasa |
| SLOOP Floyd FM · renebohne | [v2.4](https://github.com/renebohne/sloop-fm1/releases/download/v2.4/sloop-2.4.fwsc) | Arranque/LCD/audio: pasa |
| sloopDX · zvenson | [v4.0](https://github.com/zvenson/dxsloop/releases/download/v4.0/felucca-4.0.fwsc) | Arranque/LCD/audio: pasa |
| TRIBE · apiel | [v0.3-beta](https://github.com/apiel/fm1-tribe/releases/download/v0.3-beta/tribe-0.3-beta.fwsc) | Arranque/LCD/audio: pasa |
| Unholy Union · actuallysparky | [v0.2.3](https://github.com/actuallysparky/FM-1-Unholy-Union-generative-drone-firmware/releases/download/v0.2.3/unholyunion-permutation-0.2.3.fwsc) | Arranque/LCD/audio: pasa |
| Vector37 · genkigenki | [v0.3.2](https://github.com/genkigenki/Vector37/releases/download/v0.3.2/vector37-fm1-0.3.2.fwsc) | Arranque/LCD/audio: pasa |
| WaveLoop FM-1 · ELI7VH | [1.0.5.2-midi23](https://github.com/ELI7VH/Felucca/releases/download/1.0.5.2-midi23/felucca-1.0.5.2-midi23.fwsc) | Arranque/LCD/audio: pasa |
| X0X · charlesvestal | [v1.1.0](https://github.com/charlesvestal/fm1-x0x/releases/download/v1.1.0/x0x-1.1.0.fwsc) | Arranque/LCD/audio: pasa |
| X0X-RMX · Mr-Roboto (Oliver) | [v1.1.0](https://github.com/Mr-Roboto/fm1-x0x-rmx/releases/download/v1.1.0/x0x-1.1.0.fwsc) | Arranque/LCD/audio: pasa |
| ZEN-1 · aceyprime | [v1.2](https://github.com/aceyprime/zen-1/releases/download/v1.2/zen-1-1.2.fwsc) | Arranque/LCD/audio: pasa |
