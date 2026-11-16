package io.github.reiwand

import androidx.activity.compose.setContent
import androidx.compose.ui.test.assertIsEnabled
import androidx.compose.ui.test.assertIsNotEnabled
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performScrollToNode
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import androidx.work.WorkManager
import kotlinx.coroutines.runBlocking
import org.json.JSONArray
import org.json.JSONObject
import org.json.JSONTokener
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@CleanApp
@RunWith(Baselined::class)
class PrivateModeTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val instrumentation = InstrumentationRegistry.getInstrumentation()
    private val context = instrumentation.targetContext
    private val listener = Doorbell()
    private var scenario: ActivityScenario<Main>? = null

    @After
    fun closed() {
        scenario?.close()
        Net.choose(false)
        listener.close()
    }

    private fun asked(vararg pairs: Pair<String, Any?>) = JSONTokener(Native.account(JSONObject(mapOf(*pairs)).toString())).nextValue()

    private fun refused(answer: Any?) = (answer as? JSONObject)?.optString("id") == "android.private_mode"

    private fun everyAction(): List<Boolean> = listOf(
        refused(asked("op" to "add_anchored", "url" to "http://${listener.base}/", "fingerprint" to "a".repeat(64))),
        refused(asked("op" to "add", "url" to "http://${listener.base}")),
        asked("op" to "refresh_repos", "force" to true) is JSONArray,
        runCatching { Net.https("https://${listener.base}/").connect() }.isFailure,
        Updates.check(context).toString().contains("Success"),
    )

    private fun shown(content: @androidx.compose.runtime.Composable (Main) -> Unit) {
        scenario?.close()
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main -> main.setContent { content(main) } }
    }

    @Test
    fun nothingLeavesWhileThePhoneIsPrivate() {
        storage()
        Net.choose(true)
        assertEquals(listOf(true, true, true, true, true), everyAction())
        Sync.soon(context, 0)
        assertTrue(WorkManager.getInstance(context).getWorkInfosForUniqueWork("sync-soon").get().isEmpty())
        val line = context.getString(R.string.private_mode_line)
        shown { main -> RepoAdd(main, { _, work -> runBlocking { work() } }) {} }
        compose.onNodeWithText(line).assertExists()
        compose.onNodeWithTag("repo_add").assertIsNotEnabled()
        shown { main -> Explore(main, {}, {}, {}) }
        compose.onNodeWithTag("private_line").assertExists()
        Thread.sleep(1000)
        assertEquals("connections while private", 0, listener.seen.get())
        Net.choose(false)
        everyAction()
        assertTrue("the same listener is reached with the mode off", waitFor { listener.seen.get() > 0 })
    }

    private fun reached(tag: String) = compose.waitUntil(10_000) {
        runCatching { compose.onNodeWithTag("sources").performScrollToNode(hasTestTag(tag)) }.isSuccess
    }

    private fun sourcesControls(on: Boolean) = listOf("repo_check", "connect_server").forEach { tag ->
        reached(tag)
        compose.onNodeWithTag(tag).apply { if (on) assertIsEnabled() else assertIsNotEnabled() }
    }

    @Test
    fun theSourcesListTurnsItsNetworkControlsOff() {
        storage()
        Net.choose(true)
        shown { main -> Sources(main, connect = {}) {} }
        sourcesControls(false)
        Net.choose(false)
        compose.waitForIdle()
        sourcesControls(true)
    }
}
