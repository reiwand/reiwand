package io.github.reiwand

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class AppUpdateTest {
    @Test
    fun onlyAStrictlyNewerReleaseIsOffered() {
        assertTrue(newerRelease("14.1.0", "v14.2.0"))
        assertTrue(newerRelease("14.1.0+3.abc", "v15.0.0"))
        assertTrue(newerRelease("9.9.9", "v10.0.0"))
        assertFalse(newerRelease("14.1.0", "v14.1.0"))
        assertFalse(newerRelease("14.1.0+3.abc", "v14.1.0"))
        assertFalse(newerRelease("14.1.0", "v13.9.9"))
        assertFalse(newerRelease("14.1.0", "latest-build"))
        assertFalse(newerRelease("14.1.0", "v14.2"))
        assertFalse(newerRelease("14.1.0", "v014.2.0"))
    }

    @Test
    fun theDownloadMustBeTheTagsRelease() {
        assertTrue(sameRelease("14.2.0", "v14.2.0"))
        assertTrue(sameRelease("14.2.0+7.def", "v14.2.0"))
        assertFalse(sameRelease("14.1.0", "v14.2.0"))
        assertFalse(sameRelease(null, "v14.2.0"))
        assertFalse(sameRelease(null, "latest-build"))
    }

    @Test
    fun theApkFollowsTheFirstAbiWithItsOwnFile() {
        assertEquals("reiwand-android-arm64-v8a.apk", apkFor(listOf("arm64-v8a", "armeabi-v7a")))
        assertEquals("reiwand-android-x86_64.apk", apkFor(listOf("x86_64", "x86")))
        assertEquals("reiwand-android.apk", apkFor(listOf("armeabi-v7a")))
        assertEquals("reiwand-android.apk", apkFor(emptyList()))
    }

    @Test
    fun theChecksumIsTheOneOfThatExactName() {
        val a = "a".repeat(64)
        val b = "b".repeat(64)
        val sums = "$a  reiwand-android.apk\n$b  reiwand-android-x86_64.apk\n"
        assertEquals(b, listedSum(sums, "reiwand-android-x86_64.apk"))
        assertEquals(a, listedSum(sums, "reiwand-android.apk"))
        assertNull(listedSum(sums, "reiwand-android-arm64-v8a.apk"))
        assertNull(listedSum("xyz  reiwand-android.apk", "reiwand-android.apk"))
    }
}
