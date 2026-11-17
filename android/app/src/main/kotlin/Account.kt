package io.github.reiwand

import androidx.compose.foundation.layout.Column
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import org.json.JSONObject

@Composable
fun Account(main: Main, open: (Aside) -> Unit, close: () -> Unit) {
    var changes by remember { mutableIntStateOf(0) }
    val book by loaded(JSONObject(), changes, main.revision.intValue) { account("accounts") as? JSONObject ?: JSONObject() }
    val found = book.optJSONArray("accounts").objects()
    TaskPage(foot = null) {
        Header(stringResource(R.string.account), close)
        CutLine()
        Group(stringResource(R.string.accounts_group))
        if (found.isEmpty()) Note(stringResource(R.string.accounts_none))
        found.forEach { rec -> Linked(main, rec) { changes++ } }
        Group(stringResource(R.string.carry_group))
        Setting(stringResource(R.string.carry_export), Modifier.testTag("carry_export"), sub = stringResource(R.string.carry_export_sub), onClick = { open(Aside.Export) })
        Setting(stringResource(R.string.carry_import), Modifier.testTag("carry_import"), sub = stringResource(R.string.carry_import_sub), onClick = { open(Aside.Import) })
    }
}

@Composable
private fun Linked(main: Main, rec: JSONObject, changed: () -> Unit) = Column(Modifier.testTag("linked_account")) {
    val name = rec.optString("label").ifEmpty { rec.optString("user") }
    Setting(name, sub = host(rec.getString("server")))
    rec.optJSONArray("repos").objects().forEach { repo ->
        Setting(host(repo.getString("url")), Modifier.testTag("unlink"), sub = stringResource(R.string.account_unlink), onClick = {
            main.act(false) { account("unlink_account", "url" to repo.getString("url")); changed() }
        })
    }
    WordAction({ main.act(false) { account("remove_account", "account" to rec.getString("id")); changed() } }, Modifier.testTag("remove_account")) {
        Text(stringResource(R.string.account_remove))
    }
}
