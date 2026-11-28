package io.github.reiwand

import android.os.ParcelFileDescriptor
import androidx.activity.compose.setContent
import androidx.compose.runtime.mutableStateOf
import androidx.compose.ui.test.hasAnyAncestor
import androidx.compose.ui.test.hasAnyDescendant
import androidx.compose.ui.test.hasClickAction
import androidx.compose.ui.test.hasTestTag
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onFirst
import androidx.compose.ui.test.performClick
import androidx.compose.ui.unit.dp
import androidx.test.core.app.ActivityScenario
import kotlinx.coroutines.runBlocking
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class ShelfNewTest {
    @get:Rule
    val compose = createEmptyComposeRule()

    private var scenario: ActivityScenario<Main>? = null
    private val made = mutableListOf<String>()

    @After
    fun closed() {
        scenario?.close()
        runBlocking { if (made.isNotEmpty()) ask("remove", "uids" to made.toList()) }
    }

    private fun stored(index: Int): Item = ParcelFileDescriptor.open(still(0xff506070.toInt() + index * 0x020202), ParcelFileDescriptor.MODE_READ_ONLY)
        .use { JSONObject(Native.store(it.fd, "Shelf $index.png")!!).getString("uid") }
        .also(made::add)
        .let { uid -> runBlocking { Item(ask("item", "uid" to uid) as JSONObject) } }

    @Test
    fun anItemAddedInFrontIsTheFocusedCardThatATapOpens() {
        storage()
        val (old, other, added) = List(3, ::stored)
        val items = mutableStateOf(listOf(old, other))
        var opened: String? = null
        scenario = ActivityScenario.launch(Main::class.java).onActivity { main ->
            main.setContent { Cards(main, items.value, old.uid, 360.dp) { opened = it } }
        }
        compose.waitForIdle()
        compose.runOnIdle { items.value = listOf(added, old, other) }
        compose.waitForIdle()
        compose.onAllNodes(hasTestTag("focused") and hasAnyDescendant(hasText(added.title))).onFirst().assertExists()
        compose.onAllNodes(hasClickAction() and hasAnyAncestor(hasTestTag("focused"))).onFirst().performClick()
        compose.waitForIdle()
        assertEquals(added.uid, opened)
    }
}
