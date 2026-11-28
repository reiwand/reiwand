package io.github.reiwand

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class SettingRowTest {
    @Test
    fun aShortValueSitsBesideItsNameAndALongOneGoesUnder() {
        assertTrue(valueBeside("Automatic"))
        assertTrue(valueBeside("Convert them"))
        assertFalse(valueBeside("Detail first: half the frames before going down a level"))
        assertFalse(valueBeside(null))
    }
}
