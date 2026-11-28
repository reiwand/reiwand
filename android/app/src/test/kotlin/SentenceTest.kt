package io.github.reiwand

import org.junit.Assert.assertEquals
import org.junit.Test

class SentenceTest {
    @Test
    fun firstLineWithCapital() {
        assertEquals("The plugin did not start: a call it needs is missing", sentence("the plugin did not start: a call it needs is missing\n  at main.js:3"))
        assertEquals("Already fine", sentence("Already fine"))
        assertEquals("", sentence(""))
    }
}
