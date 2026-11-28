package io.github.reiwand

import java.io.File
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class NetGateTest {
    private val sources = File("src/main/kotlin").walk().filter { it.extension == "kt" }.associate { it.name to it.readText() }

    private fun holding(token: String) = sources.filter { token in it.value }.keys.sorted()

    @Test
    fun noConnectionOpensBesideTheGate() {
        assertTrue(sources.isNotEmpty())
        listOf("openConnection(", "Socket(", "SocketChannel", "getByName(", "getAllByName(", "DownloadManager").forEach { token ->
            assertEquals(token, listOf("Net.kt").filter { token in sources.getValue(it) }, holding(token))
        }
        holding("discoverServices(").forEach { file ->
            sources.getValue(file).lines().filter { "discoverServices(" in it }.forEach { assertTrue(file, "Net.open()" in it) }
        }
        assertEquals(listOf("Page.kt"), holding("loadUrl("))
        assertTrue("blockNetworkLoads = !spec.getBoolean(\"network\") || !Net.open()" in sources.getValue("Page.kt"))
        assertTrue("reply.optBoolean(\"network\") && Net.open()" in sources.getValue("Page.kt"))
        holding(": Worker(").filter { "Native.account" in sources.getValue(it) }.forEach { assertTrue(it, "Net.open()" in sources.getValue(it)) }
    }
}
