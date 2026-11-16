package io.github.reiwand

import android.webkit.WebView
import androidx.test.platform.app.InstrumentationRegistry
import java.io.File
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit
import kotlinx.coroutines.runBlocking
import org.json.JSONArray
import org.json.JSONObject
import org.json.JSONTokener
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class PrivatePageTest {
    private val instrumentation = InstrumentationRegistry.getInstrumentation()
    private val context = instrumentation.targetContext
    private val readers = Readers("cut")
    private val reader = readers.make(SIDE)
    private val pages = Pages(context)
    private val painter = Painter(false, Videos(context), Tilt(context), Hearing(context), pages)
    private val listener = Doorbell()
    private val other = Doorbell()
    private var uid = ""

    @After
    fun detach() {
        instrumentation.runOnMainSync { pages.drop(painter) }
        Render.call { painter.detach() }
        Net.choose(false)
        if (uid.isNotEmpty()) runBlocking { ask("remove", "uids" to listOf(uid)) }
        listener.close()
        other.close()
        readers.close()
    }

    private fun said(script: String): String? {
        var result: String? = null
        val done = CountDownLatch(1)
        instrumentation.runOnMainSync {
            val view: WebView? = pages.page(painter)?.web
            if (view == null) done.countDown() else view.evaluateJavascript(script) { result = it; done.countDown() }
        }
        done.await(5, TimeUnit.SECONDS)
        return result?.let { JSONTokener(it).nextValue() as? String }
    }

    private fun own(): String? {
        val text = said("JSON.stringify(window.probe || null)") ?: return null
        return runCatching { JSONObject(text).optString("own") }.getOrNull()
    }

    private fun reaching(): List<String> {
        val ways = said("JSON.stringify(window.ways)")?.let(::JSONArray) ?: return listOf("no page")
        return (0 until ways.length()).map(ways::getString).mapNotNull { way ->
            listener.seen.set(0)
            said("window.go('$way'); 'ok'")
            Thread.sleep(1200)
            said("window.go(null); 'ok'")
            Thread.sleep(300)
            listener.seen.get().takeIf { it > 0 }?.let { "$way: $it" }
        }
    }

    private suspend fun placed(private: Boolean, extra: String = "") {
        val dir = File(storage(), "web/cut").apply { deleteRecursively(); mkdirs() }
        File(dir, "manifest.json").writeText(privateManifest(listener.base))
        File(dir, "index.html").writeText(PRIVATE_PAGE.replace("ORIGIN", listener.base).replace("</body>", "$extra</body>"))
        File(dir, "own.txt").writeText("read")
        uid = (ask("examples", "dir" to dir.parent) as JSONArray).getString(0)
        ask("grant", "uid" to uid, "input" to "network", "on" to true)
        Net.choose(private)
        Native.library(JSONObject(mapOf("op" to "pick", "choice" to choice("item", uid))).toString())
        Render.call {
            Native.commit(BOTH)
            painter.attach(reader.surface)
            painter.resize(SIDE, SIDE, 0)
            painter.show(true)
        }
        assertTrue("the page reads its own file: ${own()}", waitFor { own() == "read" })
    }

    @Test
    fun aReachingPageReachesNothingOnceThePhoneTurnsPrivate() = runBlocking {
        placed(false)
        said("window.go('fetch'); 'ok'")
        assertTrue("the granted origin is reached with the mode off", waitFor { listener.seen.get() > 0 })
        Net.choose(true)
        Render.call { painter.refresh() }
        Thread.sleep(1000)
        assertTrue("the page comes back after the change", waitFor { own() == "read" })
        assertEquals("a page made private reaches nothing", emptyList<String>(), reaching())
    }

    @Test
    fun aWebWallpaperLoadsOnlyItsOwnFilesWhileThePhoneIsPrivate() = runBlocking {
        placed(true)
        assertEquals("a private page reaches nothing", emptyList<String>(), reaching())
        Net.choose(false)
        Render.call { painter.refresh() }
        Thread.sleep(1000)
        assertTrue("the page comes back after the change", waitFor { own() == "read" })
        said("window.go('fetch'); 'ok'")
        assertTrue("the granted origin is reached with the mode off", waitFor { listener.seen.get() > 0 })
    }

    @Test
    fun aFrameInADeclaredShadowTreeReachesNoOriginThatWasNotGranted() = runBlocking {
        val at = "https://${other.base}"
        val fetching = "<script>setInterval(() => fetch('$at/x').catch(() => 0), 200)</script>"
        placed(false, "<div><template shadowrootmode=\"open\"><iframe src=\"$at/frame\"></iframe><iframe srcdoc=\"$fetching\"></iframe></template></div>")
        said("window.go('fetch'); 'ok'")
        assertTrue("the granted origin is reached", waitFor { listener.seen.get() > 0 })
        Thread.sleep(1500)
        assertEquals("the origin that was not granted", 0, other.seen.get())
    }

    private companion object {
        const val SIDE = 128
    }
}
