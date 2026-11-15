package io.github.dexdeckcraft

import io.github.craftandroid.CraftActivity

/**
 * DexDeckCraft's activity. Everything Android-side (documents through the Storage Access
 * Framework, clipboard, printing, mouse and keyboard, the on-screen keyboard, lifecycle,
 * diagnostics) is craft-android's [CraftActivity]; the library it loads (`libdexdeckcraft.so`,
 * the manifest's `android.app.lib_name`) runs DeckCraft's interface.
 */
class MainActivity : CraftActivity() {
    /**
     * Presentations dragged onto the window open as DeckCraft's File › Open opens them. (A
     * `.deckcraft` file has no MIME type of its own: craft-android also takes
     * `application/octet-stream`.)
     */
    override fun droppableMimeTypes(): List<String> = PRESENTATION_TYPES

    companion object {
        /** What DeckCraft opens: PowerPoint presentations, shows and templates. */
        val PRESENTATION_TYPES = listOf(
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            "application/vnd.openxmlformats-officedocument.presentationml.slideshow",
            "application/vnd.openxmlformats-officedocument.presentationml.template",
        )
    }
}
