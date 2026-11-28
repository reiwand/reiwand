package io.github.reiwand

import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts.OpenDocument
import androidx.annotation.StringRes
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import java.io.File
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONArray
import org.json.JSONObject

const val PLUGIN = "plugin:"

/** One line of what a plugin's consent says, as a string and its arguments. */
data class Said(@StringRes val id: Int, val args: List<String> = emptyList())

/** What the phone tells before a plugin is installed or updated: the same facts as the desktop's consent. */
fun consent(version: String, license: String, hosts: List<String>, installed: String?, added: List<String>, key: Triple<String, Boolean, List<String>>?, keyChanged: Boolean, step: String = "new"): List<Said> = buildList {
    add(Said(R.string.plugin_version, listOf(version, license)))
    add(Said(R.string.plugin_contacts, listOf(hosts.joinToString(", "))))
    installed?.let { add(if (added.isEmpty()) Said(R.string.plugin_replaces, listOf(it)) else Said(R.string.plugin_replaces_more, listOf(it, added.joinToString(", ")))) }
    installed?.takeIf { step == "same" }?.let { add(Said(R.string.plugin_same)) }
    installed?.takeIf { step == "older" }?.let { add(Said(R.string.plugin_older, listOf(it))) }
    add(
        when {
            key == null -> Said(R.string.plugin_no_key)
            key.second -> Said(R.string.plugin_key, listOf(key.first, key.third.joinToString(", ")))
            else -> Said(R.string.plugin_key_maybe, listOf(key.first, key.third.joinToString(", ")))
        },
    )
    if (keyChanged) add(Said(R.string.plugin_key_changed))
}

private fun JSONObject.key() = optJSONObject("secret")?.let { Triple(it.getString("name"), it.getBoolean("required"), it.optJSONArray("hosts").strings()) }

private fun JSONObject.nullable(name: String) = if (isNull(name)) null else optString(name)

private fun JSONObject.uses() = optJSONArray("sources").strings()

@Composable
private fun Said.text() = stringResource(id, *args.toTypedArray())

/** The installed provider plugins, how to install one from a zip file and how to remove one. */
@Composable
fun Plugins(main: Main, changes: Int, run: Run) {
    val context = LocalContext.current
    val plugins by loaded(JSONArray(), changes, main.revision.intValue) { account("plugins") as? JSONArray ?: JSONArray() }
    var offer by remember { mutableStateOf<JSONObject?>(null) }
    var open by remember { mutableStateOf<JSONObject?>(null) }
    val zip = File(context.cacheDir, "plugin").resolve("plugin.zip")
    val pick = rememberLauncherForActivityResult(OpenDocument()) { uri ->
        if (uri != null) run("") {
            withContext(Dispatchers.IO) {
                zip.parentFile?.mkdirs()
                context.contentResolver.openInputStream(uri)!!.use { input -> zip.outputStream().use { input.copyTo(it) } }
            }
            offer = account("plugin_inspect", "path" to zip.path) as JSONObject
        }
    }
    PluginRows(plugins, { pick.launch(arrayOf("application/zip")) }) { open = it }
    offer?.let { shown -> Consent(shown, zip.path, run, { offer = null; zip.delete() }) { title -> main.toast(main.getString(R.string.plugin_installed, title)) } }
    open?.let { Opened(it, run) { open = null } }
}

/** The installed plugins with what uses each, and the button that installs one (no state of its own). */
@Composable
internal fun PluginRows(plugins: JSONArray, install: () -> Unit, open: (JSONObject) -> Unit) {
    Group(stringResource(R.string.plugins))
    Note(stringResource(R.string.plugins_intro))
    plugins.objects().forEach { plugin ->
        val used = plugin.uses()
        val state = plugin.nullable("error")?.let { stringResource(R.string.plugin_broken) }
        val sub = if (used.isEmpty()) stringResource(R.string.plugin_unused) else stringResource(R.string.plugin_used_by, used.joinToString(", "))
        Setting(plugin.optString("title").ifEmpty { plugin.getString("id") }, sub = listOfNotNull(state, sub).joinToString(" · "), value = plugin.optString("version").ifEmpty { null }, onClick = { open(plugin) })
    }
    Column(Modifier.fillMaxWidth().padding(horizontal = Measure.margin, vertical = Measure.e2)) {
        QuietAction(install, Modifier.fillMaxWidth()) { Text(stringResource(R.string.plugin_install)) }
    }
}

@Composable
internal fun Consent(offer: JSONObject, path: String, run: Run, close: () -> Unit, done: (String) -> Unit) {
    val plugin = offer.getJSONObject("plugin")
    val title = plugin.getString("title")
    val installed = offer.nullable("installed")
    val said = consent(plugin.getString("version"), plugin.getString("license"), plugin.optJSONArray("hosts").strings(), installed, offer.optJSONArray("added_hosts").strings(), plugin.key(), offer.optBoolean("secret_changed"), offer.optString("step"))
    Question(
        stringResource(if (installed == null) R.string.plugin_install_question else R.string.plugin_update_question, title),
        said.map { it.text() }.joinToString("\n"),
        stringResource(if (installed == null) R.string.plugin_install_yes else R.string.plugin_update_yes),
        stringResource(R.string.cancel),
        false,
        close,
    ) {
        val token = offer.getString("token")
        run("") {
            try {
                account("plugin_install", "path" to path, "token" to token)
                done(title)
            } finally {
                close()
            }
        }
    }
}

@Composable
internal fun Opened(plugin: JSONObject, run: Run, close: () -> Unit) {
    val used = plugin.uses()
    val title = plugin.optString("title").ifEmpty { plugin.getString("id") }
    val key = plugin.key()
    val lines = listOfNotNull(
        stringResource(R.string.plugin_contacts, plugin.optJSONArray("hosts").strings().joinToString(", ")).takeIf { plugin.optJSONArray("hosts").strings().isNotEmpty() },
        key?.let { stringResource(R.string.plugin_asks_key, it.first) } ?: stringResource(R.string.plugin_no_key),
        plugin.nullable("error")?.let(::sentence),
        if (used.isEmpty()) null else stringResource(R.string.plugin_remove_sources_first, title, used.joinToString(", ")),
    )
    var removing by remember { mutableStateOf(false) }
    if (removing) {
        Question(stringResource(R.string.plugin_remove_question), stringResource(R.string.plugin_remove_note, title), stringResource(R.string.remove), stringResource(R.string.cancel), true, close) {
            run("") {
                try {
                    account("plugin_remove", "id" to plugin.getString("id"))
                } finally {
                    close()
                }
            }
        }
        return
    }
    Sheet(close, title, lines.joinToString("\n")) {
        if (used.isEmpty()) SheetOption(stringResource(R.string.plugin_remove), ph = Ph.Trash, removes = true) { removing = true }
        SheetOption(stringResource(R.string.close)) { close() }
    }
}
