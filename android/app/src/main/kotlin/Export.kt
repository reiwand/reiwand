package io.github.reiwand

import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts.CreateDocument
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource

@Composable
fun Export(main: Main, close: () -> Unit) {
    var own by rememberSaveable { mutableStateOf(false) }
    var secrets by rememberSaveable { mutableStateOf(false) }
    var password by rememberSaveable { mutableStateOf("") }
    var problem by rememberSaveable { mutableStateOf<String?>(null) }
    var done by rememberSaveable { mutableStateOf(false) }
    val working = stringResource(R.string.carry_working)
    var task by rememberSaveable { mutableStateOf(false) }
    val save = rememberLauncherForActivityResult(CreateDocument(CARRY_TYPE)) { uri ->
        if (uri != null) main.act {
            task = true
            problem = null
            try {
                val file = main.carryFile().also { it.delete() }
                account("carry_export", "path" to file.path, "own" to own, "secrets" to secrets, "password" to password)
                main.copyOut(file, uri)
                done = true
            } catch (e: Refused) {
                problem = main.told(e)
            } finally {
                task = false
            }
        }
    }
    val cut = cut()
    TaskPage(foot = {
        Action({ save.launch(CARRY_NAME) }, Modifier.weight(1f).testTag("carry_save"), enabled = !task && !cut && (!secrets || password.isNotEmpty())) {
            Text(stringResource(R.string.carry_export))
        }
    }) {
        Header(stringResource(R.string.carry_export), close)
        Note(stringResource(R.string.carry_export_note))
        CARRY_SECTIONS.filter { it != "own" && it != "secrets" }.forEach { Setting(carrySection(it), sub = null) }
        Toggle(carrySection("own"), own) { own = it }
        Toggle(carrySection("secrets"), secrets) { secrets = it; done = false }
        if (secrets) {
            Note(stringResource(R.string.carry_secrets_note))
            TextBox(stringResource(R.string.carry_password), password, { password = it; problem = null }, Modifier.testTag("carry_password"), hidden = true, problem = problem.takeIf { secrets })
        } else {
            problem?.let { Note(it, tones.coral) }
        }
        if (task) Progress(working, null)
        if (done) Note(stringResource(R.string.carry_exported), tones.sprout)
    }
}
