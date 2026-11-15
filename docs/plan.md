# Plan y estado

DexDeckCraft sigue el [prompt de portado de DexPdfCraft](https://github.com/manoloGith/DexPdfCraft/blob/main/docs/PORTING-PROMPT.md).
Como DeckCraft usa el mismo egui (`6d518fc`) y el mismo winit (`a517b38`) que las otras apps, toda
la capa Android viene hecha en craft-android y las fases se han hecho de una vez, sin pasar por el
dispositivo entre ellas. La lista de pruebas en el dispositivo está en [pruebas.md](pruebas.md).

## Lo que se encontró al estudiar DeckCraft (sección 6 del prompt)

| Punto | En DeckCraft | En DexDeckCraft |
|---|---|---|
| Arranque (`apps/deckcraft/src/main.rs`) | `SlideApp::new(Session, Services)`, `ui.json`/`prefs.json`, AutoRecuperación, canal de control, Apple events, autoactualización, elección X11/Wayland | `DeckApp` (`src/lib.rs`) hace lo mismo sin canal de control, Apple events ni autoactualización |
| `rfd` / `arboard` | Solo en el binario: la interfaz los recibe por `Services` | No hacen falta shims: `src/android/mod.rs` rellena `Services` con craft-android |
| Diálogos de archivo | `pick_open` / `pick_save` bloquean el hilo de la interfaz | `Services::picked_later` del fork: responden más tarde |
| `Command::new` | Nada que afecte (solo `fsutil` en Windows y tests) | — |
| Viewports secundarios | Vista del presentador y Vista ▸ Nueva ventana (`show_viewport_immediate`) | `set_embed_viewports(true)`: ventanas internas |
| Entradas de compilación | `CRAFT_FONTS_DIR` opcional (fuentes de presentación y de respaldo) | No se usa: fuentes de `/system/fonts` (fork). Ver «límites» |
| Marcas | Logotipos de ArtCraft solo en `docs/brand/`; el logotipo de «Acerca de» se dibuja en código | Icono propio |
| Datos y configuración | `$XDG_CONFIG_HOME/deckcraft`, recuperación en `$XDG_DATA_HOME/deckcraft/recovery` | craft-android apunta XDG al almacenamiento privado |
| Formatos | deckcraft, pptx, potx, ppsx; esquemas txt, md | Filtros de intents `VIEW`/`EDIT`/`SEND` con los tipos MIME de PowerPoint; `.deckcraft` por DexFiles y el selector |
| Autoguardado | AutoRecuperación cada minuto (configurable) | Igual; además se escribe al perder el foco |
| Hardware | Sonido con cpal; vídeo decodificado por software (craft-video) | cpal con AAudio; el mismo decodificador |
| Idioma y tema | Interfaz solo en inglés; tema claro u oscuro, sin «como el sistema» | Igual; el primer arranque toma el tema del sistema |

## Fases

| Fase | Hecho |
|---|---|
| 0 | Repositorio, fork con rama `dexcraft/android`, submódulos `upstream` y `craft-android`, workspace con los `[patch]` de upstream copiados y winit de craft-android, Gradle + cargo-ndk, CI, NOTICE, licencias, icono propio |
| 1 | eframe en GameActivity (CraftActivity), wgpu con glow de reserva, el motor en arm64, diagnóstico de craft-android (renderizador, versión) |
| 2 | La interfaz real de DeckCraft, arrancada como en escritorio; entorno HOME/XDG (craft-android); preferencias; fuentes de `/system/fonts` (fork) |
| 3 | Selectores del SAF con escritura de vuelta: Abrir, Guardar como, exportar, insertar imagen, audio y vídeo; portapapeles (texto, HTML, imagen) y Ctrl+C/X/V; «Abrir con» y «Compartir»; ratón, teclado y ciclo de vida (craft-android); informe de diagnóstico (Ctrl+Mayús+D, solo en el APK de debug) |
| 4 | Cursores, título de la tarea («Charla — DexDeckCraft»), arrastrar y soltar (craft-android y `droppableMimeTypes`), pestañas de presentaciones |
| 5 | Imprimir con el diálogo de Android; enlaces con Intent; sonido de las diapositivas; sin autoactualización |
| 6 | Teclado en pantalla e insets (craft-android) |
| 7 | Perfil de release (LTO, `codegen-units = 1`), workflow de release con firma, metadatos de F-Droid/Obtainium |

## Qué se ha comprobado y qué no

Comprobado sin dispositivo:

- Tests del fork (6 nuevos; la suite de la interfaz da los mismos 13 fallos que el `main` de
  storytold en este entorno, por falta de GPU y diccionarios) y de la app (8).
- La interfaz completa en una ventana de escritorio (`--features desktop`), con las pestañas.
- Comprobación de tipos de toda la app para `aarch64-linux-android`, incluido el código de Android.
- El APK, en la CI de GitHub.

**No comprobado** (no hay dispositivo ni emulador): todo lo que ocurre en Android. En especial lo
que es nuevo respecto a las otras apps Dex: los selectores diferidos, el sonido (cpal/AAudio), el
vídeo, la presentación a pantalla completa y la impresión interceptada.

## Decisiones y límites conocidos

- **Los selectores no bloquean** (ver [fork.md](fork.md)). Consecuencia: tras «Guardar» en la
  pregunta de cerrar con una presentación sin archivo, la app no se cierra sola al elegir dónde
  guardarla; hay que volver a cerrar.
- **Exportar todas las diapositivas como imágenes** escribe `Nombre1.png`, `Nombre2.png`… junto al
  archivo elegido. En Android el selector da un solo documento, así que esas imágenes se quedan en
  el almacenamiento privado de la app. Exportar una diapositiva, PDF, GIF y PPTX sí funcionan.
- **Una sola ventana y una sola pantalla.** La Vista del presentador y «Nueva ventana» son internas;
  no se usa el monitor externo como segunda pantalla de la presentación.
- **Fuentes.** No se incrusta craft-fonts (decenas de MB). Las fuentes de Office que no estén en el
  dispositivo se sustituyen por las del sistema, así que el texto puede ocupar distinto que en
  escritorio. Se puede compilar con `CRAFT_FONTS_DIR` para incrustarlas.
- **Tema.** DeckCraft no tiene opción «como el sistema»: se toma el del sistema en el primer
  arranque y después manda el botón Claro/Oscuro de la barra de título.
- **Interfaz en inglés**, como DeckCraft.
- **«Acerca de»** es el de DeckCraft, sin cambios.
