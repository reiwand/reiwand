package io.github.reiwand

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class IntervalTest {
    @Test
    fun aStoredIntervalComesBackAsMinutes() {
        assertEquals(45, intervalMinutes("45m"))
        assertEquals(90, intervalMinutes("1h30m"))
        assertEquals(2880 + 120, intervalMinutes("2d2h"))
        assertEquals("90", customMinutes("1h30m"))
        assertEquals("", customMinutes("soon"))
        assertNull(intervalMinutes("1h30"))
        assertNull(intervalMinutes(""))
    }

    @Test
    fun anIntervalIsSaidInDaysHoursAndMinutes() {
        assertEquals(listOf("h" to 1, "m" to 30), intervalParts(90))
        assertEquals(listOf("d" to 1, "m" to 5), intervalParts(1445))
        assertEquals(listOf("h" to 2), intervalParts(120))
    }
}
