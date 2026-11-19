package io.github.reiwand

import androidx.test.platform.app.InstrumentationRegistry
import kotlinx.coroutines.runBlocking
import org.json.JSONArray
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class RepoUninstallTest {
    private val context = InstrumentationRegistry.getInstrumentation().targetContext

    private fun offered() = runBlocking { (account("browse", "query" to "") as JSONArray).objects() }

    @Test
    fun anInstalledPackageIsUninstalledAndTheRepositoryOffersItAgain() {
        storage()
        examples(context)
        assertEquals(0, offered().count { it.optBoolean("installed") })
        val first = offered().first()
        val (repo, id) = first.getString("repo_id") to first.getString("id")
        runBlocking { account("install", "repo_id" to repo, "id" to id) }
        assertTrue(waitFor { offered().any { it.getString("id") == id && it.optBoolean("installed") } })
        runBlocking { account("uninstall", "repo_id" to repo, "id" to id) }
        assertTrue(waitFor { offered().none { it.optBoolean("installed") } })
        runBlocking { account("install", "repo_id" to repo, "id" to id) }
        assertTrue(waitFor { offered().any { it.getString("id") == id && it.optBoolean("installed") } })
        runBlocking { account("uninstall", "repo_id" to repo, "id" to id) }
    }
}
