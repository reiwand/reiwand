package io.github.reiwand

import androidx.activity.compose.setContent
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.test.core.app.ActivityScenario
import androidx.test.platform.app.InstrumentationRegistry
import org.json.JSONObject
import org.json.JSONTokener
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class CarryTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private var scenario: ActivityScenario<Main>? = null

    @Before
    fun prepare() {
        storage()
    }

    @After
    fun closed() {
        scenario?.close()
    }

    private fun asked(vararg pairs: Pair<String, Any?>) = JSONTokener(Native.account(JSONObject(mapOf(*pairs)).toString())).nextValue() as JSONObject

    @Test
    fun aFileExportedHereIsReadAndImportedAgainWithoutReplacingAnything() {
        val file = context.cacheDir.resolve("carry-test.reiwand").also { it.delete() }
        val exported = asked("op" to "carry_export", "path" to file.path, "own" to false, "secrets" to false, "password" to "")
        assertFalse(exported.toString(), exported.has("error"))
        assertTrue(file.length() > 0)
        val preview = asked("op" to "carry_preview", "path" to file.path, "password" to "")
        assertFalse(preview.toString(), preview.has("error"))
        assertTrue(preview.getJSONObject("file").getJSONArray("sections").length() > 0)
        val imported = asked("op" to "carry_import", "path" to file.path, "password" to "", "take" to org.json.JSONArray(listOf("lists", "tags")), "lists" to "join")
        assertFalse(imported.toString(), imported.has("error"))
        assertEquals(0, imported.getJSONArray("pending").length())
        file.delete()
    }

    @Test
    fun anExistingFileIsNeverOverwrittenAndSecretsNeedAPassword() {
        val file = context.cacheDir.resolve("carry-kept.reiwand").also { it.writeText("kept") }
        val over = asked("op" to "carry_export", "path" to file.path, "own" to false, "secrets" to false, "password" to "")
        assertEquals("android.carry_file", over.getString("id"))
        assertEquals("kept", file.readText())
        file.delete()
        val bare = asked("op" to "carry_export", "path" to file.path, "own" to false, "secrets" to true, "password" to "")
        assertEquals("android.carry_secrets_password", bare.getString("id"))
        assertFalse(file.exists())
    }

    @Test
    fun theAccountPageOffersTheExportAndTheImport() {
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main -> main.setContent { Theme { Account(main, {}) {} } } }
        compose.onNodeWithText(context.getString(R.string.carry_export)).assertIsDisplayed()
        compose.onNodeWithTag("carry_import").assertIsDisplayed()
    }
}
