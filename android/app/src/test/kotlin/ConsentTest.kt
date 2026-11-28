package io.github.reiwand

import org.junit.Assert.assertEquals
import org.junit.Test

class ConsentTest {
    private val key = Triple("API key", false, listOf("api.example.org"))
    private val hosts = listOf("api.example.org", "images.example.org")

    @Test
    fun aNewPluginTellsItsHostsAndItsKey() {
        assertEquals(
            listOf(
                Said(R.string.plugin_version, listOf("1.0.0", "CC0-1.0")),
                Said(R.string.plugin_contacts, listOf("api.example.org, images.example.org")),
                Said(R.string.plugin_key_maybe, listOf("API key", "api.example.org")),
            ),
            consent("1.0.0", "CC0-1.0", hosts, null, hosts, key, false),
        )
        assertEquals(Said(R.string.plugin_no_key), consent("1.0.0", "MIT", hosts, null, hosts, null, false).last())
        assertEquals(Said(R.string.plugin_key, listOf("Token", "api.example.org")), consent("1.0.0", "MIT", hosts, null, hosts, key.copy("Token", true), false).last())
    }

    @Test
    fun anUpdateTellsWhatIsNew() {
        val same = consent("1.1.0", "MIT", hosts, "1.0.0", emptyList(), key, false)
        assertEquals(Said(R.string.plugin_replaces, listOf("1.0.0")), same[2])
        val more = consent("1.1.0", "MIT", hosts + "cdn.example.org", "1.0.0", listOf("cdn.example.org"), key, true)
        assertEquals(Said(R.string.plugin_replaces_more, listOf("1.0.0", "cdn.example.org")), more[2])
        assertEquals(Said(R.string.plugin_key_changed), more.last())
    }

    @Test
    fun goingBackOrInstallingTheSameVersionWarns() {
        assertEquals(Said(R.string.plugin_older, listOf("1.1.0")), consent("1.0.0", "MIT", hosts, "1.1.0", emptyList(), key, false, "older")[3])
        assertEquals(Said(R.string.plugin_same), consent("1.1.0", "MIT", hosts, "1.1.0", emptyList(), key, false, "same")[3])
        assertEquals(4, consent("1.2.0", "MIT", hosts, "1.1.0", emptyList(), key, false, "newer").size)
    }
}
