# El fork de DeckCraft

`upstream/` es un submódulo de **[manoloGith/deckcraft](https://github.com/manoloGith/deckcraft)**,
un fork de `storytold/deckcraft`, en la rama **`dexcraft/android`**: el `main` de upstream (DeckCraft
0.19.0, `e86c25a`) más cuatro commits que DexDeckCraft necesita en Android. En escritorio y en la
web no cambia nada: lo nuevo está apagado por defecto o solo actúa en Android.

| Cambio | Por qué |
|---|---|
| Carpetas de fuentes de Android (`crates/fonts/src/fontdb.rs`: `/system/fonts` y `/product/fonts`) | Android guarda ahí sus fuentes (Roboto, Noto, CJK, árabe, hebreo, tailandés…) y no tiene fontconfig. Sin esto las diapositivas solo tendrían las fuentes incrustadas. |
| «Ayuda ▸ Buscar actualizaciones» en Android (`crates/ui-egui/src/app/commands.rs`, dos commits) | La autoactualización de escritorio no sirve para una app Android: dice que la actualiza la tienda o el instalador. |
| Selectores que responden más tarde (`Services::picked_later`, `crates/ui-egui/src/app/files.rs`) | DeckCraft espera la respuesta de `pick_open` y `pick_save` en el hilo de la interfaz. En Android el selector es otra actividad y la pausa de la actividad espera a ese hilo: se colgaría. Con `picked_later`, el selector se abre, devuelve `None`, y cuando llega la ruta DeckCraft vuelve a ejecutar lo que lo pidió (Abrir, Guardar como, exportar, insertar imagen, audio o vídeo, cambiar imagen, reutilizar diapositivas, temas, comparar, rellenos de imagen, álbum de fotos). Genérico y con tests (`picked_later_tests.rs`): se podría proponer a storytold. |
| Enganche bajo la cinta (`Services::below_ribbon`, `crates/ui-egui/src/app.rs`) | Para dibujar las pestañas de presentaciones de craft-android entre la cinta y las diapositivas. Genérico: el host dibuja lo que quiera y usa los comandos de la app. Con test (kittest). |

Como en GridCraft y WordCraft, **no hacen falta los shims de `rfd` y `arboard`**: la interfaz de
DeckCraft recibe los diálogos de archivos, el portapapeles y el sonido del binario que la arranca
(`Services`), así que la app Android le da los suyos (`apps/dexdeckcraft/src/android/mod.rs`).
Tampoco hizo falta tocar DeckCraft para:

- **Imprimir**: DeckCraft imprime abriendo un PDF (`file://…pdf`); la app intercepta ese enlace y
  manda el PDF al diálogo de impresión de Android (`DeckApp::ui`).
- **Una sola ventana**: `Context::set_embed_viewports(true)` hace que la Vista del presentador y
  Vista ▸ Nueva ventana sean ventanas internas.
- **Recuperación al perder el foco**: `deckcraft_engine::recovery::save` ya es público.
- **Discord**: DeckCraft no lo tiene en la barra de título, solo en «Acerca de» y Backstage.

## Trabajar en el fork

En `upstream/`, `origin` es el fork y `storytold` el proyecto original:

```bash
git -C upstream checkout dexcraft/android
```

Las normas de `upstream/AGENTS.md` siguen valiendo (`cargo fmt` con su `rustfmt.toml`, clippy sin
avisos). Después de hacer commit en `upstream/`, sube la rama y registra aquí el nuevo commit del
submódulo:

```bash
git -C upstream push origin dexcraft/android
```

```bash
git add upstream && git commit -m "upstream: …"
```

## Actualizar a una versión nueva de DeckCraft

```bash
scripts/sync-upstream.sh
```

Rebasa `dexcraft/android` sobre el `main` de storytold, comprueba que los `[patch.crates-io]` de
este workspace siguen coincidiendo con los de upstream (y que winit sigue en la revisión de
craft-android) y vuelve a tomar su `Cargo.lock`. Luego: compilar, probar, subir la rama con
`--force-with-lease` y hacer commit del submódulo.
