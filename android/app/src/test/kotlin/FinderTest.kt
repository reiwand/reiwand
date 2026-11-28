package io.github.reiwand

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class FinderTest {
    @Test
    fun theSearchIgnoresCaseAndAccents() {
        assertTrue(finds("cafe", "Order at the café"))
        assertTrue(finds("  ORDER ", "Order at the café"))
        assertTrue(finds("", "Quality"))
        assertFalse(finds("zzqx", "Quality"))
    }
}
