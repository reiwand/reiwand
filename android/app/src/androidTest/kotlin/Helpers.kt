package io.github.reiwand

import android.app.NotificationManager
import android.graphics.Bitmap
import android.os.ParcelFileDescriptor
import androidx.test.platform.app.InstrumentationRegistry
import java.io.File
import java.util.concurrent.atomic.AtomicInteger
import org.json.JSONObject

const val IMAGE = "image"
const val VIDEO = "video"

fun storage(): File = InstrumentationRegistry.getInstrumentation().targetContext.cacheDir.resolve("render-test")
    .apply { mkdirs() }
    .also { Native.prepare(it.path) }

fun exampleScenes() =
    withExamples(InstrumentationRegistry.getInstrumentation().targetContext) { answer(Native::library, "examples", arrayOf("dir" to it.path)) }

fun still(color: Int): File = File(storage(), "still.png").apply {
    outputStream().use { out ->
        Bitmap.createBitmap(8, 8, Bitmap.Config.ARGB_8888).apply { eraseColor(color) }
            .compress(Bitmap.CompressFormat.PNG, 100, out)
    }
}

fun fixture(name: String): File = File(storage(), name).apply {
    InstrumentationRegistry.getInstrumentation().context.assets.open(name).use { input ->
        outputStream().use { input.copyTo(it) }
    }
}

fun install(file: File): String {
    val item = ParcelFileDescriptor.open(file, ParcelFileDescriptor.MODE_READ_ONLY)
        .use { JSONObject(Native.store(it.fd, file.name)!!) }
    Native.library(JSONObject(mapOf("op" to "pick", "choice" to choice("item", item.getString("uid")))).toString())
    Render.call { Native.commit(BOTH) }
    return item.getString("kind")
}

fun settled(frames: AtomicInteger) = Thread.sleep(300).let { frames.get() }

fun quiet(frames: AtomicInteger) = Thread.sleep(1500).let { frames.get() }

fun hex(color: Int): String = Integer.toHexString(color)

fun near(expected: Int, drawn: Int, tolerance: Int) =
    listOf(16, 8, 0).all { shift -> kotlin.math.abs((expected shr shift and 0xff) - (drawn shr shift and 0xff)) <= tolerance }

fun waitFor(limit: Long = 20_000, done: () -> Boolean): Boolean {
    val end = System.currentTimeMillis() + limit
    while (true) {
        if (done()) return true
        if (System.currentTimeMillis() >= end) return false
        Thread.sleep(20)
    }
}

fun <T> zoned(zone: String, body: () -> T): T {
    val instrumentation = InstrumentationRegistry.getInstrumentation()
    val alarms = instrumentation.targetContext.getSystemService(android.app.AlarmManager::class.java)
    val before = java.util.TimeZone.getDefault().id
    instrumentation.uiAutomation.adoptShellPermissionIdentity("android.permission.SET_TIME_ZONE")
    try {
        alarms.setTimeZone(zone)
        org.junit.Assert.assertTrue("the zone of the phone is $zone", waitFor { java.util.TimeZone.getDefault().id == zone })
        return body()
    } finally {
        alarms.setTimeZone(before)
        instrumentation.uiAutomation.dropShellPermissionIdentity()
    }
}

private val FILTERS = mapOf(
    NotificationManager.INTERRUPTION_FILTER_ALL to "all",
    NotificationManager.INTERRUPTION_FILTER_PRIORITY to "priority",
    NotificationManager.INTERRUPTION_FILTER_ALARMS to "alarms",
    NotificationManager.INTERRUPTION_FILTER_NONE to "none",
)

private const val NOTIFICATION_SOUND_MS = 2_500L

fun hush(): AutoCloseable {
    val instrumentation = InstrumentationRegistry.getInstrumentation()
    val manager = instrumentation.targetContext.getSystemService(NotificationManager::class.java)
    val before = FILTERS[manager.currentInterruptionFilter] ?: "all"
    val set = { filter: String ->
        ParcelFileDescriptor.AutoCloseInputStream(instrumentation.uiAutomation.executeShellCommand("cmd notification set_dnd $filter"))
            .use { it.readBytes() }
        org.junit.Assert.assertTrue("the phone follows $filter", waitFor { FILTERS[manager.currentInterruptionFilter] == filter })
    }
    set("none")
    Thread.sleep(NOTIFICATION_SOUND_MS)
    return AutoCloseable { set(before) }
}

fun sample(name: String, kind: String): String =
    InstrumentationRegistry.getInstrumentation().context.assets.open("phone-scenes/$name.$kind.json").use { String(it.readBytes()) }
