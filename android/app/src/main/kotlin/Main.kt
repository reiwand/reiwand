package io.github.reiwand

import android.app.Activity
import android.content.pm.ActivityInfo
import android.content.Intent
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.PickVisualMediaRequest
import androidx.activity.result.contract.ActivityResultContracts.OpenDocument
import androidx.activity.result.contract.ActivityResultContracts.OpenDocumentTree
import androidx.activity.result.contract.ActivityResultContracts.OpenMultipleDocuments
import androidx.activity.result.contract.ActivityResultContracts.PickMultipleVisualMedia
import androidx.activity.result.contract.ActivityResultContracts.PickVisualMedia
import androidx.activity.result.contract.ActivityResultContracts.StartActivityForResult
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.ui.unit.IntSize
import java.util.concurrent.atomic.AtomicBoolean
import kotlin.concurrent.thread
import kotlinx.coroutines.MainScope
import kotlinx.coroutines.cancel
import kotlinx.coroutines.launch
import org.json.JSONArray
import org.json.JSONObject

class Main : ComponentActivity() {
    val revision = mutableIntStateOf(0)
    val importing = mutableIntStateOf(0)
    val imports = mutableIntStateOf(0)
    val pendingAddress = mutableStateOf("")
    private val playing = lazy { Videos(applicationContext) }
    val videos by playing
    val tilt by lazy { Tilt(applicationContext) }
    val hearing by lazy { Hearing(applicationContext) }
    val players by lazy { Players(applicationContext) }
    private val paging = lazy { Pages(applicationContext) }
    val pages by paging
    private val power by lazy { Power(applicationContext) }
    private val scope = MainScope()
    private val pick = registerForActivityResult(PickMultipleVisualMedia()) { store(it) }
    private val files = registerForActivityResult(OpenMultipleDocuments()) { store(it) }
    private val folder = registerForActivityResult(OpenDocumentTree()) { uri -> uri?.let(::importFolder) }
    private val packages = registerForActivityResult(OpenDocument()) { uri -> uri?.let { store(listOf(it)) } }
    val placing = mutableStateOf(false)
    val search = mutableStateOf<String?>(null)
    val noticing = mutableStateOf(false)
    val fault = mutableStateOf<String?>(null)
    private val noticed = { runOnUiThread { Notice.asking(this).also { noticing.value = true } } }
    private val saverSkipped = AtomicBoolean()
    private val conversions = { _: String -> runOnUiThread { revision.intValue++ } }
    val preview = registerForActivityResult(StartActivityForResult()) { placed(it.resultCode == Activity.RESULT_OK) }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge(DARK_BARS, DARK_BARS)
        Native.prepare(noBackupFilesDir.path)
        Unveil.load()
        Converting.finished = conversions
        Converting.asking = noticed
        if (savedInstanceState == null) {
            thread(isDaemon = true) {
                remirror(); runCatching { examples(this, false) }; runOnUiThread { revision.intValue++ }
                Updates.found(Updates.count())
            }
        }
        addressOf(intent)
        setContent {
            App(this)
            NoticeAsk(this)
            Fault(this)
        }
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        addressOf(intent)
    }

    private fun addressOf(intent: Intent?) {
        val link = intent?.data?.takeIf { it.scheme == "reiwand" && it.host == "add" } ?: return AppInstall.opened(this, intent)
        link.getQueryParameter("url")?.let { pendingAddress.value = it }
    }

    override fun onStart() {
        super.onStart()
        if (Build.VERSION.SDK_INT >= 34) {
            val lit = Native.setting("playback.hdr", null) != "off"
            window.colorMode = if (lit) ActivityInfo.COLOR_MODE_HDR else ActivityInfo.COLOR_MODE_DEFAULT
        }
        power.show(this, true)
    }

    override fun onStop() {
        power.show(this, false)
        super.onStop()
    }

    override fun onDestroy() {
        scope.cancel()
        if (Converting.finished === conversions) Converting.finished = null
        if (Converting.asking === noticed) Converting.asking = null
        forget()
        if (playing.isInitialized()) videos.close()
        if (paging.isInitialized()) pages.close()
        super.onDestroy()
    }

    fun choose() = pick.launch(PickVisualMediaRequest(PickVisualMedia.ImageAndVideo))

    fun chooseFiles() = files.launch(arrayOf("image/*", "video/*", "application/*"))

    fun chooseFolder() = folder.launch(null)

    fun openPackage() = packages.launch(arrayOf("*/*"))

    fun wallpaperSize(): IntSize {
        val bounds = windowManager.maximumWindowMetrics.bounds
        val turned = display.rotation % 2 == 1
        return if (turned) IntSize(bounds.height(), bounds.width()) else IntSize(bounds.width(), bounds.height())
    }

    fun examples(explore: () -> Unit) {
        thread {
            val failure = runCatching { examples(this) }.exceptionOrNull()
            runOnUiThread {
                revision.intValue++
                failure?.let(::toast) ?: explore()
            }
        }
    }

    fun changed() {
        revision.intValue++
        runCatching { Pages.forget() }
        Render.post { Wallpaper.reload(false) }
    }

    fun act(work: suspend () -> Unit) = scope.launch {
        runCatching { work() }.onFailure(::toast)
        changed()
    }

    fun unpick(choice: JSONObject) {
        if (!isChangingConfigurations && Wallpaper.commitWhenLive == null) Render.post {
            runCatching { answer(Native::library, "unpick", arrayOf("choice" to choice)) }
            Wallpaper.reload(false)
        }
    }

    fun toast(text: String) =
        Toast.makeText(this, text.ifBlank { getString(R.string.failed_without_reason) }, Toast.LENGTH_LONG).show()

    fun toast(error: Throwable) = fault.run { value = told(error).ifBlank { getString(R.string.failed_without_reason) } }

    fun store(uris: List<Uri>) {
        if (uris.isEmpty()) return
        importing.intValue += uris.size
        imports.intValue += uris.size
        thread {
            val failed = uris.withIndex().count { (index, uri) ->
                val stored = runCatching { store(uri) }.getOrDefault(false)
                runOnUiThread {
                    importing.intValue--
                    if (index % REFRESH == 0 || index == uris.lastIndex) revision.intValue++
                    if (importing.intValue == 0) imports.intValue = 0
                }
                !stored
            }
            if (failed > 0) runOnUiThread { toast(resources.getQuantityString(R.plurals.unreadable, failed, failed)) }
            if (saverSkipped.getAndSet(false)) runOnUiThread { toast(getString(R.string.convert_saver)) }
        }
    }

    private fun store(uri: Uri): Boolean {
        val item = open(uri)?.use { Native.store(it.fd, name(uri)) }?.let(::JSONObject) ?: return false
        val uid = item.getString("uid")
        val played = when {
            item.getString("kind") == "video" -> open(uri)?.use { file ->
                playable { setDataSource(file.fileDescriptor) } && poster(uid) { setDataSource(file.fileDescriptor) } != null
            } == true
            !item.isNull("poster") -> item.getString("poster").let { file -> playable { setDataSource(file) } && poster(uid) { setDataSource(file) } != null }
            else -> true
        }
        if (!played) runCatching { answer(Native::library, "remove", arrayOf("uids" to JSONArray(listOf(uid)))) }
        if (played && item.getString("kind") == "video" && convert(uid, name(uri), false) == Convert.SAVER) saverSkipped.set(true)
        notice(strings(item.optJSONArray("exposes")))?.let { told -> runOnUiThread { toast(told) } }
        return played
    }

    private fun open(uri: Uri) = runCatching { contentResolver.openFileDescriptor(uri, "r") }.getOrNull()

    fun name(uri: Uri) = titled(uri)
}

private const val REFRESH = 10
