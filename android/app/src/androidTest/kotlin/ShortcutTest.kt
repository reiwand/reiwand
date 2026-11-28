package io.github.reiwand

import android.content.Intent
import android.content.IntentFilter
import androidx.test.platform.app.InstrumentationRegistry
import java.io.File
import kotlinx.coroutines.runBlocking
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class ShortcutTest : SceneBase() {
    private val instrumentation = InstrumentationRegistry.getInstrumentation()

    @After
    fun forgotten() = Shortcuts.forget()

    private fun tapped(target: JSONObject) = runBlocking {
        val dir = File(storage(), "shortcuts/shortcuts").apply { mkdirs() }
        File(dir, "scene.json").writeText(SHORTCUT_SCENE)
        File(dir, "open.js").writeText(SHORTCUT_SCRIPT)
        File(dir, "manifest.json").writeText(SHORTCUT_MANIFEST)
        Native.library(JSONObject().put("op", "examples").put("dir", dir.parent).toString())
        painter.opened = { Shortcuts.open(context, it) }
        val uid = show("Shortcuts")
        assertTrue(waitFor { frames.get() > 0 })
        ask("grant", "uid" to uid, "input" to "taps", "on" to true)
        ask("property", "uid" to uid, "key" to "open", "value" to target)
        Render.call { painter.refresh() }
        settled(frames)
        Thread.sleep(APART_MS)
        tap(SIDE / 4f)
    }

    @Test
    fun aTapOnTheWallpaperOpensTheWebAddressTheUserChose() {
        val filter = IntentFilter(Intent.ACTION_VIEW).apply { addDataScheme("https"); addCategory(Intent.CATEGORY_BROWSABLE) }
        val monitor = instrumentation.addMonitor(filter, null, true)
        try {
            tapped(JSONObject().put("uri", "https://example.org/a"))
            assertTrue("the tap did not reach the browser", waitFor { monitor.hits == 1 })
            assertNull(Shortcuts.missed)
        } finally {
            instrumentation.removeMonitor(monitor)
        }
    }

    @Test
    fun anAppThatIsNotThereIsReportedInsteadOfSwallowed() {
        tapped(JSONObject().put("app", "org.example.absent"))
        assertTrue("nothing was reported", waitFor { Shortcuts.missed != null })
        assertEquals("app" to "org.example.absent", Shortcuts.missed)
    }

    @Test
    fun theFailuresAreToldApartAndAGoodOpeningClearsThem() {
        val filter = IntentFilter(Intent.ACTION_VIEW).apply { addDataScheme("https"); addCategory(Intent.CATEGORY_BROWSABLE) }
        val monitor = instrumentation.addMonitor(filter, null, true)
        try {
            assertTrue(!Shortcuts.open(context, JSONObject().put("app", "org.example.absent")))
            assertEquals("app", Shortcuts.missed?.first)
            assertTrue(Shortcuts.open(context, JSONObject().put("uri", "https://example.org/")))
            assertNull(Shortcuts.missed)
        } finally {
            instrumentation.removeMonitor(monitor)
        }
    }

    @Test
    fun aChosenDocumentOpensThroughAViewIntentWithReadAccess() {
        val filter = IntentFilter(Intent.ACTION_VIEW).apply { addDataScheme("content"); addDataType("*/*") }
        val monitor = instrumentation.addMonitor(filter, null, true)
        try {
            val file = "content://com.android.externalstorage.documents/document/primary%3ADocuments%2Fa.txt"
            assertTrue(Shortcuts.open(context, JSONObject().put("file", file)))
            assertTrue("the document did not reach a viewer", waitFor { monitor.hits == 1 })
            assertNull(Shortcuts.missed)
        } finally {
            instrumentation.removeMonitor(monitor)
        }
    }

    @Test
    fun theNameOfAChosenDocumentIsItsLastPart() {
        assertEquals("a.txt", Shortcuts.named("content://x.documents/document/primary%3ADocuments%2Fa.txt"))
        assertEquals("Documents", Shortcuts.named("content://x.documents/tree/primary%3ADocuments"))
    }

    @Test
    fun theLastFailureIsKeptForTheNextTimeTheProcessStarts() {
        assertTrue(!Shortcuts.open(context, JSONObject().put("app", "org.example.absent")))
        val kept = context.getSharedPreferences("shortcuts", 0)
        assertEquals("app" to "org.example.absent", kept.getString("reason", null) to kept.getString("shown", null))
        Shortcuts.forget()
        assertNull(kept.getString("reason", null))
    }

    private companion object {
        const val APART_MS = 1_100L
        val SHORTCUT_SCENE: String get() = sample("shortcut", "scene")
        const val SHORTCUT_SCRIPT = """import { scene, openShortcut } from "reiwand";
export function init() { scene.layer("button").on("click", () => { openShortcut("open"); }); }
"""
        val SHORTCUT_MANIFEST: String get() = sample("shortcut", "manifest")
    }
}
