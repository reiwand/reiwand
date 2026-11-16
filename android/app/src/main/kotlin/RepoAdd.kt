package io.github.reiwand

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.height
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.KeyboardType
import java.net.URI
import org.json.JSONObject

@Composable
fun RepoAdd(main: Main, run: Run, close: (() -> Unit)? = null, connect: ((JSONObject) -> Unit)? = null, lens: Lens = Camera, added: () -> Unit) {
    var url by rememberSaveable { mutableStateOf("") }
    var print by rememberSaveable { mutableStateOf("") }
    var problem by rememberSaveable { mutableStateOf<String?>(null) }
    var wrong by rememberSaveable { mutableStateOf("") }
    var scanning by rememberSaveable { mutableStateOf(false) }
    var blind by rememberSaveable { mutableStateOf<Int?>(null) }
    var server by rememberSaveable { mutableStateOf<String?>(null) }
    if (scanning) {
        BackHandler { scanning = false }
        Scan(lens, { code ->
            scanning = false
            problem = null
            if (code.has("server")) {
                server = code.toString()
            } else {
                server = null
                url = code.getString("url")
                print = ""
            }
        }, { blind = if (it == R.string.camera_refused) R.string.repo_camera_refused else R.string.repo_camera_missing; scanning = false }, ::repoCode) { scanning = false }
        return
    }
    val checking = stringResource(R.string.repo_checking)
    val host = hostOf(url)
    val cut = cut()
    val add = {
        problem = null
        run(checking) {
            try {
                account("add_anchored", "url" to url.trim(), "fingerprint" to print.trim().ifEmpty { null })
                url = ""
                print = ""
                added()
            } catch (e: Refused) {
                wrong = refusedField(e.id)
                problem = main.told(e).ifBlank { main.getString(R.string.failed_without_reason) }
            }
        }
    }
    TaskPage(foot = {
        Action(add, Modifier.weight(1f).testTag("repo_add"), enabled = url.isNotBlank() && !cut) { Text(stringResource(R.string.repo_add_url)) }
    }, above = { if (host.isNotEmpty()) Note(stringResource(R.string.repo_sent, host)) }) {
        Header(stringResource(R.string.repo_add_title), close)
        CutLine()
        Note(stringResource(R.string.repo_about))
        TextBox(stringResource(R.string.repo_url), url, { url = it; problem = null; server = null }, Modifier.testTag("repo_url"), stringResource(R.string.repo_url_example), problem.takeIf { wrong == URL_FIELD }) {
            if (blind == null) Square({ scanning = true }, Ph.Scan, stringResource(R.string.scan_code), Modifier.testTag("repo_scan"), boxed = false)
        }
        blind?.let { Text(stringResource(it), Modifier.padding(horizontal = Measure.margin).testTag("repo_typing"), tones.mist, style = Type.foot) }
        server?.let { code ->
            Row(Modifier.padding(horizontal = Measure.margin).testTag("repo_server"), verticalAlignment = Alignment.CenterVertically) {
                Text(stringResource(R.string.repo_server_code), Modifier.weight(1f), tones.mist, style = Type.foot)
                connect?.let { go -> WordAction({ go(JSONObject(code)) }, Modifier.testTag("repo_connect"), enabled = !cut) { Text(stringResource(R.string.connect)) } }
            }
        }
        TextBox(
            stringResource(R.string.repo_fingerprint), print, { print = it; problem = null },
            problem = problem.takeIf { wrong == PRINT_FIELD }, example = stringResource(R.string.repo_fingerprint_help), kind = KeyboardType.Ascii, done = add,
        )
        problem?.takeIf { wrong.isEmpty() }?.let { Text(it, Modifier.padding(horizontal = Measure.margin).testTag("repo_problem"), tones.coral, style = Type.foot) }
        Spacer(Modifier.height(Measure.e4))
    }
}

fun repoCode(text: String): JSONObject {
    val url = text.trim()
    if (url.startsWith("reiwand://")) runCatching { JSONObject(Native.connectCode(url)) }.getOrNull()?.takeIf { it.has("server") }?.let { return it }
    val uri = runCatching { URI(url) }.getOrNull()
    if (uri != null && uri.scheme?.lowercase() in setOf("https", "http") && !uri.host.isNullOrEmpty()) return JSONObject().put("url", url)
    return JSONObject().put("error", "a repository address starts with https://; http:// works only for an address on this same phone (localhost or 127.0.0.1)").put("id", "android.repo_https")
}

private const val URL_FIELD = "url"
private const val PRINT_FIELD = "print"
private val PRINT_REFUSALS = setOf("android.repo_fingerprint", "android.repo_needs_fingerprint", "android.repo_two_fingerprints")

fun refusedField(id: String) = when (id) {
    "android.repo_https" -> URL_FIELD
    in PRINT_REFUSALS -> PRINT_FIELD
    else -> ""
}
