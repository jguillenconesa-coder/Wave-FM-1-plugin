# Autoría, mantenimiento y procedencia

Este repositorio reúne y empaqueta trabajo de terceros. **La titularidad del
repositorio `jguillenconesa-coder/Wave-FM-1-plugin` no atribuye a su mantenedor
la creación del emulador, el código original del plugin, los firmwares, sus
recursos ni el instrumento físico.** Tampoco se atribuye al mantenedor la
programación de las tareas de integración realizadas con asistencia de IA.

`jguillenconesa-coder` solicita, mantiene y publica esta integración. Las tareas
de preparación, adaptación de interfaz, empaquetado, documentación y verificación
se han realizado con asistencia de OpenAI Codex. Esto describe el proceso de
trabajo: no transfiere derechos de terceros ni supone respaldo de OpenAI o de
los autores originales. Los nombres de cuentas en el historial Git documentan
operaciones de mantenimiento/publicación; las atribuciones y licencias de los
archivos originales siguen siendo las que constan en sus avisos.

- **Joakim Eriksson y colaboradores**: fm1-emu, emulación de CPU/SoC, motor de
  instrumento, carga de firmware, plugin CLAP y panel interactivo original, MIT.
  `desktop/README.md` identifica la revisión y las adaptaciones de esta distribución.
- **Catálogo comunitario**: las 83 referencias iniciales proceden del índice de
  fm1-emu de Joakim Eriksson (MIT), que combina el Firmware Hub de zvenson con
  referencias propias. `firmware/community-index.json` conserva los autores y
  repositorios de cada firmware. Los enlaces adicionales apuntan a publicaciones
  originales de sus autores; no se atribuye su creación a esta integración ni
  se afirma que todos sus recursos permitan redistribución.
- **Timo Kaluza, Paul Walker y colaboradores de free-audio/clap-wrapper**:
  conversión del CLAP a VST3, MIT.
- **Robbert van der Helm y colaboradores**: nih-plug, ISC.
- **M-VAVE**: fabricante y diseño del instrumento FM-1. Las seis paletas se
  dibujan a partir de la referencia de acabados proporcionada; no se distribuye
  la fotografía. Las marcas no pertenecen a este repositorio.
- **Autores de los 23 firmwares y de sus componentes**: nombres, versiones,
  repositorios y licencias en [firmware/README.md](https://github.com/jguillenconesa-coder/Wave-FM-1-plugin/blob/main/firmware/README.md).
  Los avisos íntegros en `firmware/licenses/<id>/` conservan también los créditos
  de motores DSP, muestras, presets, SDK, fuentes y otros recursos heredados.
- **Otras bibliotecas, SDK y herramientas**: [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
  Las distribuciones incluyen el inventario de dependencias Rust con autores,
  versiones y repositorios, junto con sus textos de licencia.

La licencia MIT de la raíz se limita al material de integración propio al que
resulte aplicable. **No sustituye las licencias de terceros ni reclama derechos
sobre su obra.** Se conservan sus avisos de copyright, fuentes y condiciones.
El proyecto no está afiliado ni avalado por M-VAVE, Ableton o los autores citados.
