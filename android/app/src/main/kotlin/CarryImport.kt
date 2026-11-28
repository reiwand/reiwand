package io.github.reiwand

import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts.OpenDocument
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import org.json.JSONArray
import org.json.JSONObject

private const val NEEDS_PASSWORD = "android.carry_needs_password"

@Composable
fun Import(main: Main, close: () -> Unit) {
    var preview by remember { mutableStateOf<JSONObject?>(null) }
    var password by rememberSaveable { mutableStateOf("") }
    var asked by rememberSaveable { mutableStateOf(false) }
    var problem by rememberSaveable { mutableStateOf<String?>(null) }
    var report by remember { mutableStateOf<JSONObject?>(null) }
    var both by rememberSaveable { mutableStateOf(false) }
    var off by rememberSaveable { mutableStateOf(setOf<String>()) }
    var task by rememberSaveable { mutableStateOf(false) }
    val working = stringResource(R.string.carry_working)
    val look = {
        main.act {
            task = true
            problem = null
            try {
                preview = account("carry_preview", "path" to main.carryFile().path, "password" to password) as JSONObject
                asked = false
            } catch (e: Refused) {
                asked = e.id == NEEDS_PASSWORD || (asked && e.id == "android.carry_wrong_password")
                problem = main.told(e)
            } finally {
                task = false
            }
        }
    }
    val open = rememberLauncherForActivityResult(OpenDocument()) { uri ->
        if (uri != null) main.act {
            report = null
            preview = null
            password = ""
            try {
                main.copyIn(uri)
                look()
            } catch (e: Exception) {
                problem = main.getString(R.string.carry_unreadable)
            }
        }
    }
    val shown = preview
    val cut = cut()
    TaskPage(foot = {
        when {
            report != null -> Action(close, Modifier.weight(1f)) { Text(stringResource(R.string.carry_done)) }
            shown != null -> Action({
                val take = JSONArray(shown.optJSONArray("rows").objects().map { it.getString("section") }.filter { it !in off })
                main.act {
                    task = true
                    try {
                        report = account("carry_import", "path" to main.carryFile().path, "password" to password, "take" to take, "lists" to if (both) "both" else "join") as JSONObject
                        main.revision.intValue++
                        main.dropCarry()
                    } catch (e: Refused) {
                        problem = main.told(e)
                    } finally {
                        task = false
                    }
                }
            }, Modifier.weight(1f).testTag("carry_apply"), enabled = !task && !cut) { Text(stringResource(R.string.carry_import)) }
            asked -> Action({ look() }, Modifier.weight(1f).testTag("carry_open"), enabled = password.isNotEmpty() && !task) { Text(stringResource(R.string.carry_open)) }
            else -> Action({ open.launch(arrayOf("*/*")) }, Modifier.weight(1f).testTag("carry_choose"), enabled = !task) { Text(stringResource(R.string.carry_choose)) }
        }
    }) {
        Header(stringResource(R.string.carry_import), close)
        Note(stringResource(R.string.carry_import_note))
        if (asked) TextBox(stringResource(R.string.carry_password), password, { password = it }, Modifier.testTag("carry_password"), hidden = true, problem = problem)
        else problem?.let { Note(it, tones.coral) }
        if (task) Progress(stringResource(R.string.carry_working), null)
        shown?.takeIf { report == null }?.let { Rows(it, off, { off = it }, both) { both = it } }
        report?.let { Report(it) }
    }
}

@Composable
private fun Rows(preview: JSONObject, off: Set<String>, set: (Set<String>) -> Unit, both: Boolean, mode: (Boolean) -> Unit) {
    preview.optJSONArray("rows").objects().forEach { row ->
        val name = row.getString("section")
        val sub = stringResource(R.string.carry_row, row.optInt("new"), row.optInt("held"))
        Toggle(carrySection(name) + " · " + sub, name !in off) { set(if (it) off - name else off + name) }
    }
    if (preview.optJSONArray("lists").let { it != null && it.length() > 0 }) {
        Toggle(stringResource(R.string.carry_lists_both), both, change = mode)
    }
    for (i in 0 until (preview.optJSONArray("notes")?.length() ?: 0)) Note(stringResource(R.string.carry_sessions_note))
}

@Composable
private fun Report(report: JSONObject) {
    Note(stringResource(R.string.carry_imported), tones.sprout)
    (report.optJSONObject("added") ?: JSONObject()).let { added ->
        added.keys().forEach { Setting(carrySection(it), value = added.optInt(it).toString()) }
    }
    report.optJSONArray("pending").objects().forEach { Note(it.optString("name") + ": " + it.optString("reason"), tones.mist) }
    report.optJSONArray("unresolved").let { list -> for (i in 0 until (list?.length() ?: 0)) Note(list!!.optString(i), tones.mist) }
}
