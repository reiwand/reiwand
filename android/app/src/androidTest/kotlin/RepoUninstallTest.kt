package io.github.reiwand

import androidx.activity.compose.setContent
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithTag
import androidx.compose.ui.test.onAllNodesWithText
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.performClick
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import kotlinx.coroutines.runBlocking
import org.json.JSONArray
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class RepoUninstallTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null

    @After
    fun closed() {
        scenario?.close()
    }

    private fun installed() = runBlocking { (account("browse", "query" to "") as JSONArray).objects().count { it.optBoolean("installed") } }

    @Test
    fun anInstalledPackageIsUninstalledFromTheRepositoryListAndCanBeInstalledAgain() {
        examples(context)
        assertEquals(0, installed())
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main ->
            main.setContent { Repos(main, { _, work -> main.act { work() } }) {} }
        }
        compose.onNodeWithTag("explore_repos").performClick()
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodesWithText(context.getString(R.string.repo_install)).fetchSemanticsNodes().isNotEmpty() }
        compose.onAllNodesWithText(context.getString(R.string.repo_install))[0].performClick()
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodesWithTag("uninstall").fetchSemanticsNodes().isNotEmpty() }
        assertEquals(1, installed())
        compose.onAllNodesWithTag("uninstall")[0].performClick()
        compose.waitUntil(TIMEOUT_MS) { compose.onAllNodesWithTag("uninstall").fetchSemanticsNodes().isEmpty() }
        assertTrue(waitFor { installed() == 0 })
    }

    private companion object {
        const val TIMEOUT_MS = 20_000L
    }
}
