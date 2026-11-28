package io.github.reiwand

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalUriHandler
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import org.json.JSONObject

private const val NEEDS_ACCOUNT = "android.account_needed"
private const val UNKNOWN_CERTIFICATE = "android.cert_unknown"
private const val IS_SERVER = "android.repo_is_server"

@Composable
fun RepoAdd(main: Main, run: Run, close: (() -> Unit)? = null, start: String = "", added: () -> Unit) {
    var url by rememberSaveable { mutableStateOf(start) }
    var problem by rememberSaveable { mutableStateOf<String?>(null) }
    var print by rememberSaveable { mutableStateOf<String?>(null) }
    var trust by rememberSaveable { mutableStateOf<String?>(null) }
    var accounts by remember { mutableStateOf<List<JSONObject>?>(null) }
    var code by remember { mutableStateOf<JSONObject?>(null) }
    var repos by remember { mutableStateOf<List<JSONObject>?>(null) }
    val checking = stringResource(R.string.repo_checking)
    val signing = stringResource(R.string.sign_in_waiting)
    val browser = LocalUriHandler.current
    val cut = cut()
    val finish: suspend () -> Unit = {
        url = ""
        trust = null
        added()
    }
    val fail: (Refused) -> Unit = { e -> problem = main.told(e).ifBlank { main.getString(R.string.failed_without_reason) } }
    val add = { address: String ->
        problem = null
        run(checking) {
            try {
                account("add_repo", "url" to address.trim(), "trust" to trust)
                finish()
            } catch (e: Refused) {
                when (e.id) {
                    NEEDS_ACCOUNT -> accounts = (account("accounts") as JSONObject).optJSONArray("accounts").objects().filter { it.getString("server") == origin(address) }
                    UNKNOWN_CERTIFICATE -> print = e.args.getOrNull(1)
                    IS_SERVER -> repos = (account("server_repos", "url" to address.trim(), "trust" to trust) as JSONObject).optJSONArray("repos").objects()
                    else -> fail(e)
                }
            }
        }
    }
    val signIn = {
        problem = null
        run(signing) {
            try {
                val reply = account("sign_in", "url" to url.trim()) as JSONObject
                code = reply
                browser.openUri(reply.getString("uri"))
                account("sign_in_wait")
                code = null
                accounts = null
                account("add_repo", "url" to url.trim(), "trust" to trust)
                finish()
            } catch (e: Refused) {
                code = null
                fail(e)
            }
        }
    }
    val use = { rec: String ->
        run(checking) {
            try {
                account("use_account", "url" to url.trim(), "account" to rec)
                accounts = null
                account("add_repo", "url" to url.trim(), "trust" to trust)
                finish()
            } catch (e: Refused) {
                fail(e)
            }
        }
    }
    val choose = repos
    if (choose != null) {
        BackHandler { repos = null }
        return ServerRepos(choose, { repos = null }) { pick -> url = pick; repos = null; add(pick) }
    }
    TaskPage(foot = {
        Action({ add(url) }, Modifier.weight(1f).testTag("repo_add"), enabled = url.isNotBlank() && !cut) { Text(stringResource(R.string.repo_add_url)) }
    }, above = { if (host(url).isNotEmpty()) Note(stringResource(R.string.repo_sent, host(url))) }) {
        Header(stringResource(R.string.repo_add_title), close)
        CutLine()
        Note(stringResource(R.string.repo_about))
        TextBox(stringResource(R.string.repo_url), url, { url = it; problem = null; print = null; trust = null; accounts = null }, Modifier.testTag("repo_url"), stringResource(R.string.repo_url_example), problem, done = { add(url) })
        print?.let { seen ->
            Note(stringResource(R.string.repo_trust_ask, host(url), seen))
            Action({ trust = seen; print = null; add(url) }, Modifier.padding(horizontal = Measure.margin).testTag("repo_trust"), enabled = !cut) { Text(stringResource(R.string.repo_trust)) }
        }
        accounts?.let { found ->
            Note(stringResource(R.string.repo_private, host(url)))
            found.forEach { rec ->
                val name = rec.optString("label").ifEmpty { rec.optString("user") }
                Setting(stringResource(R.string.use_account, name), Modifier.testTag("use_account"), sub = host(rec.getString("server")), enabled = !cut, onClick = { use(rec.getString("id")) })
            }
            Setting(stringResource(if (found.isEmpty()) R.string.sign_in_browser else R.string.use_other_account), Modifier.testTag("sign_in"), enabled = !cut, onClick = signIn)
        }
        code?.optString("code")?.takeIf { it.isNotEmpty() }?.let { Text(stringResource(R.string.sign_in_code, it), Modifier.padding(horizontal = Measure.margin).testTag("sign_in_code"), tones.frost, style = Type.text) }
        Spacer(Modifier.height(Measure.e4))
    }
}

private fun origin(url: String): String {
    val trimmed = url.trim()
    val scheme = trimmed.substringBefore("://", "")
    return if (scheme.isEmpty()) trimmed else "$scheme://${host(trimmed)}"
}

@Composable
private fun ServerRepos(repos: List<JSONObject>, close: () -> Unit, pick: (String) -> Unit) = TaskPage(foot = null) {
    Header(stringResource(R.string.repo_server_pick), close)
    Note(stringResource(R.string.repo_server_note))
    repos.forEach { repo ->
        Setting(repo.getString("name"), Modifier.testTag("server_repo"), sub = host(repo.getString("url")), onClick = { pick(repo.getString("url")) })
    }
}
