# Qué probar al instalar DexDeckCraft

Instala el APK (`DexDeckCraft-…-arm64-v8a.apk` o el de debug de la CI) en un móvil o tableta con
Android 10 o posterior. El release no muestra herramientas de diagnóstico. Si algo falla y hace
falta el informe, instala el APK de debug de la CI (artefacto `dexdeckcraft-debug-apk`), repite el
fallo, pulsa **Ctrl+Mayús+D** (o abre el icono «DexDeckCraft diagnostics»), copia el informe y
pégalo en el issue o el chat.

Marca cada punto con ✅ / ❌ y anota lo que veas. Los marcados con **(nuevo)** usan código que las
otras apps Dex no tienen, así que son los que más conviene mirar.

## 1. Arranque y cierre (DeX, con ratón)
- [ ] Abre la app desde el escritorio de DeX: aparece en una ventana de ~1280×800 con la pantalla
      de inicio de DeckCraft (plantillas y recientes).
- [ ] Ciérrala con la X de la ventana y vuelve a abrirla. **Tres veces seguidas**, sin que se cierre
      sola, se cuelgue o quede en negro.
- [ ] Maximiza, redimensiona y mueve la ventana entre la pantalla del móvil y el monitor: la
      interfaz se redibuja a su tamaño y no se reinicia.
- [ ] El título en la barra de tareas de DeX es «Presentation1 — DexDeckCraft» (o el nombre del
      archivo) y añade «— Edited» al hacer cambios.
- [ ] El primer arranque sale en claro u oscuro según el sistema; el botón Claro/Oscuro de la barra
      de título lo cambia y se recuerda.

## 2. Ratón
- [ ] Clic en una forma para seleccionarla, arrastrar para moverla, tiradores para redimensionar y
      girar, doble clic para editar el texto.
- [ ] Clic derecho en una forma, en la diapositiva y en una miniatura: sale el menú contextual.
- [ ] Rueda: pasa de diapositiva o desplaza; Ctrl+rueda hace zoom.
- [ ] Cursor: flecha, I sobre texto, flechas de redimensionar en los tiradores, mano sobre enlaces.
- [ ] La cinta cambia de pestaña (Home, Insert, Design…) y los botones responden.
- [ ] Arrastrar miniaturas para reordenar diapositivas.

## 3. Teclado físico
- [ ] Escribe en un cuadro de texto: letras, números, `ñ`, acentos (´ + a = á), `@`, `€` con AltGr.
- [ ] Atajos: Ctrl+Z / Ctrl+Y, Ctrl+N, Ctrl+O, Ctrl+S, Ctrl+B, Ctrl+M (diapositiva nueva), Ctrl+D
      (duplicar), F5 (presentación), Esc.
- [ ] **Copiar y pegar**:
  - [ ] Ctrl+C / Ctrl+V de una forma dentro de la app, y de texto dentro de un cuadro.
  - [ ] Copia texto en otra app y pégalo con Ctrl+V en un cuadro de texto.
  - [ ] Copia texto de DexDeckCraft y pégalo en otra app.
  - [ ] Copia una imagen (Galería, navegador) y pégala: se inserta en la diapositiva.

## 4. Archivos **(nuevo: selectores que responden más tarde)**
- [ ] File ▸ Open (y Ctrl+O): sale el selector de Android; abre un `.pptx`. Se ve bien.
- [ ] Abre también un `.ppsx` (con File ▸ Open se abre para editar) y un `.deckcraft`.
- [ ] Edita el `.pptx` y pulsa Ctrl+S: **el cambio aparece en el archivo original**.
- [ ] Presentación nueva ▸ Ctrl+S: sale el selector para crear el archivo; tras elegirlo se guarda
      y el título cambia al nombre nuevo.
- [ ] File ▸ Save As como `.pptx` y como `.deckcraft`.
- [ ] File ▸ Export: PDF, una diapositiva como PNG, GIF. Cada uno pide dónde guardarlo y el archivo
      aparece con su contenido.
- [ ] En cualquier selector, pulsa Atrás o Cancelar: no pasa nada y la app sigue normal. Después,
      otra operación con selector funciona bien (no se aplica la respuesta a lo anterior).
- [ ] Insert ▸ Picture: elige una imagen y se inserta. Lo mismo con Audio y Video.
- [ ] Clic derecho en una imagen ▸ Change Picture.
- [ ] Format Shape ▸ Fill ▸ Picture, y Format Background ▸ Picture: la imagen elegida se aplica.
- [ ] Home ▸ New Slide ▸ Reuse Slides: elige otra presentación y sale la lista de sus diapositivas.
- [ ] Design ▸ Browse for Themes con un `.pptx` o `.thmx`.
- [ ] Insert ▸ Photo Album ▸ Add Pictures: la imagen aparece en la lista del diálogo.
- [ ] Desde «Mis archivos», «Abrir con» un `.pptx` → DexDeckCraft: se abre. Con un `.ppsx`,
      empieza la presentación.
- [ ] Desde otra app, «Compartir» un `.pptx` → DexDeckCraft.
- [ ] En DeX, arrastra un `.pptx` desde «Mis archivos» a la ventana: se abre. Arrastra una imagen:
      se inserta.
- [ ] Recientes en la pantalla de inicio: vuelven a abrirse.

## 5. Pestañas de presentaciones
- [ ] Bajo la cinta hay una pestaña por presentación abierta; la que ves está resaltada.
- [ ] Abre otra (o pulsa +): aparece su pestaña y pasa a ser la activa.
- [ ] Clic en otra pestaña: se muestra esa presentación, en su diapositiva.
- [ ] Una presentación con cambios muestra un punto; al pasar el ratón, la ×.
- [ ] × (o clic central) sin cambios la cierra; con cambios, pregunta Save / Don't Save / Cancel.
- [ ] Las pestañas no salen en la vista File (Backstage) ni durante la presentación.

## 6. Presentación **(nuevo)**
- [ ] «Present» (o F5): la presentación ocupa la ventana; clic, flechas y Espacio avanzan; Esc sale.
- [ ] Transiciones y animaciones se ven fluidas.
- [ ] Slide Show ▸ Presenter View: aparece dentro de la misma ventana (no en otra) y no se cuelga.
- [ ] View ▸ New Window: aparece como ventana interna.
- [ ] Una diapositiva con **audio**: suena al reproducirlo, y se para al salir.
- [ ] Una diapositiva con **vídeo** (mp4): se ve y se oye, sincronizado.
- [ ] Transitions ▸ Sound: suena al pasar de diapositiva.

## 7. Cerrar con cambios sin guardar
- [ ] Haz un cambio y pulsa Atrás: pregunta si guardar. Cancel deja la app abierta.
- [ ] Haz un cambio y cierra con la X de DeX (no se puede interceptar): al volver a abrir, la
      presentación se recupera («Recovered 1 unsaved presentation…»).
- [ ] Haz un cambio, ve a la pantalla de inicio de Android, mata la app desde Recientes y ábrela:
      también se recupera.

## 8. Funciones del sistema
- [ ] File ▸ Print **(nuevo)**: se abre el diálogo de impresión de Android con la vista previa del
      PDF (diseño y diapositivas elegidos); «Guardar como PDF» funciona.
- [ ] Un hipervínculo `https://…` en una diapositiva abre el navegador.
- [ ] Help ▸ Check for Updates: dice que la app la actualiza su tienda o instalador.
- [ ] Textos en chino, japonés, coreano, árabe, hebreo y tailandés se ven con sus letras (no ▯).
- [ ] Una presentación con fuentes de Office (Calibri, Aptos): mira si el texto se desborda o
      cambia mucho de tamaño respecto a PowerPoint (ver «límites»).

## 9. Móvil o tableta (sin DeX)
- [ ] Toca un cuadro de texto y escribe: sale el teclado en pantalla, con predicción.
- [ ] La interfaz no queda debajo de la barra de estado ni de la de navegación.
- [ ] Gira la pantalla: se adapta sin reiniciarse ni perder la presentación.
- [ ] Sal a la pantalla de inicio y vuelve: la presentación sigue igual.
- [ ] En un móvil estrecho la cinta se desplaza; anota si algo queda inaccesible.

## 10. Rendimiento
- [ ] Una presentación grande (50+ diapositivas con imágenes) se abre y se desplaza con fluidez.
- [ ] (APK de debug) El informe de diagnóstico dice `renderer wgpu, … (Vulkan…)` o `renderer glow`;
      si dice glow, anótalo.

## Límites conocidos (no son fallos)
- Tras «Save» en la pregunta de cerrar con una presentación sin archivo, la app no se cierra sola
  cuando eliges dónde guardarla: vuelve a cerrar.
- Exportar **todas** las diapositivas como PNG/JPEG no deja los archivos donde eliges (Android da
  un solo documento por selector). Exporta de una en una, o a PDF.
- La Vista del presentador y «Nueva ventana» son internas, no ventanas ni pantallas aparte.
- Las fuentes que no estén en el dispositivo se sustituyen por las del sistema.
- La interfaz está en inglés, como DeckCraft.
