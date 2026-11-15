# Avisos

DexDeckCraft usa el motor y la interfaz de **DeckCraft**
(<https://github.com/storytold/deckcraft>, © 2026 ArtCraft Team and the DeckCraft contributors), con
licencia **MIT o Apache-2.0**, incluidos como submódulo en `upstream/` desde el fork
[manoloGith/deckcraft](https://github.com/manoloGith/deckcraft) (rama `dexcraft/android`, con unos
pocos cambios para Android: ver `docs/fork.md`). Los avisos y licencias de terceros de DeckCraft
están en `upstream/NOTICE`, `upstream/ATTRIBUTION.md` y `upstream/licenses/`, y se aplican también
aquí.

La capa Android (entrada, ciclo de vida, documentos, portapapeles, impresión y winit con sus
parches) viene del submódulo `craft-android/`
([manoloGith/craft-android](https://github.com/manoloGith/craft-android), MIT o Apache-2.0; su
`winit/` conserva la licencia Apache-2.0 de winit).

El proceso de compilación con Gradle y cargo-ndk viene de
[DexPdfCraft](https://github.com/manoloGith/DexPdfCraft), que a su vez parte del de
[PdfCraft for Android](https://github.com/artcraft-android/pdfcraft-android) (MIT o Apache-2.0).
`apps/dexdeckcraft/src/android/audio.rs` es la salida de sonido del binario de escritorio de
DeckCraft (`upstream/apps/deckcraft/src/audio.rs`).

## Marcas

El nombre y los logotipos de **ArtCraft** (`upstream/docs/brand/`) son marcas del ArtCraft Team y
**no** son de código abierto: no pueden usarse en versiones modificadas
(`upstream/docs/brand/LICENSE-brand.txt`). DexDeckCraft no incluye ni muestra esos logotipos: la
interfaz de DeckCraft no los incrusta, y la app usa un icono propio (una diapositiva sobre un
monitor, en ámbar). El icono de DeckCraft no es el icono de la app (la interfaz lo lleva dentro
para el aviso de actualización de escritorio, que en Android no se muestra).

«Acerca de» es el de DeckCraft: dice sobre qué está hecha la app y enlaza a su proyecto.

El código propio de DexDeckCraft se distribuye con la misma licencia doble, MIT o Apache-2.0.
