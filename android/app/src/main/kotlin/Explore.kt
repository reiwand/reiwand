package io.github.reiwand

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import org.json.JSONArray
import org.json.JSONObject

private val COVER = 150.dp

@Composable
fun Explore(main: Main, back: () -> Unit, manage: () -> Unit) {
    var query by rememberSaveable { mutableStateOf("") }
    var installed by remember { mutableIntStateOf(0) }
    var only by rememberSaveable { mutableStateOf("") }
    var installing by remember { mutableStateOf(setOf<String>()) }
    val revision = main.revision.intValue
    val repos by loaded(JSONObject(), revision, installed) { account("repos") as? JSONObject ?: JSONObject() }
    val found by loaded(JSONArray(), query, installed, revision) { account("browse", "query" to query) as? JSONArray ?: JSONArray() }
    val cut = cut()
    val listed = repos.optJSONArray("repos").objects()
    val chosen = only.takeIf { picked -> listed.any { it.getString("repo_id") == picked } }.orEmpty()
    val shelves = listed.filter { chosen.isEmpty() || it.getString("repo_id") == chosen }.map { repo -> repo to found.objects().filter { it.getString("repo_id") == repo.getString("repo_id") } }
    val install = { item: JSONObject ->
        val key = item.getString("repo_id") + "/" + item.getString("id")
        installing += key
        main.act {
            try {
                main.complete(account("install", "repo_id" to item.getString("repo_id"), "id" to item.getString("id")))
            } finally {
                installing -= key
                installed++
            }
        }
        Unit
    }
    val named = listed.firstOrNull { it.getString("repo_id") == chosen }?.let { repoName(it.optBoolean("bundled"), it.getString("name")) }
    LazyColumn(Modifier.testTag("explore_page")) {
        item(key = "header") { Header(stringResource(R.string.repo_browse), back) }
        item(key = "cut") { CutLine() }
        item(key = "search") { Seek(query, { query = it }, named?.let { stringResource(R.string.repo_search_in, it) } ?: stringResource(R.string.repo_search_all), Modifier.padding(horizontal = Measure.margin, vertical = Measure.e2)) }
        if (listed.size > 1) item(key = "only") {
            val options = listOf("" to stringResource(R.string.all)) + listed.map { it.getString("repo_id") to repoName(it.optBoolean("bundled"), it.getString("name")) }
            Choices(null, options, chosen) { only = it }
        }
        shelves.filter { it.second.isNotEmpty() }.forEach { (repo, packages) ->
            item(key = repo.getString("repo_id")) { Packages(repo, packages, installing, manage, install.takeIf { !cut || repo.optBoolean("bundled") }) }
        }
        if (shelves.all { it.second.isEmpty() }) item(key = "nothing") { Note(stringResource(R.string.repo_nothing)) }
        item(key = "manage") {
            Setting(stringResource(R.string.sources), Modifier.testTag("manage_repos"), stringResource(R.string.repo_manage_note), onClick = manage) {
                Glyph(Ph.CaretRight, null, tint = tones.mist, size = Measure.iconSmall)
            }
        }
        item(key = "end") { Spacer(Modifier.height(Measure.e8)) }
    }
}

@Composable
private fun Packages(repo: JSONObject, packages: List<JSONObject>, installing: Set<String>, manage: () -> Unit, install: ((JSONObject) -> Unit)?) = Column(Modifier.padding(top = Measure.e3)) {
    val bundled = repo.optBoolean("bundled")
    Row(
        Modifier.fillMaxWidth().clickable(role = Role.Button, onClick = manage).padding(start = Measure.margin, end = Measure.e3, top = Measure.e2, bottom = Measure.e2),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Column(Modifier.weight(1f)) {
            Text(repoName(bundled, repo.getString("name")), style = Type.row)
            Text(if (bundled) stringResource(R.string.repo_on_phone) else stringResource(R.string.repo_packages, repo.optInt("packages")), color = tones.mist, style = Type.foot)
        }
        Glyph(Ph.CaretRight, null, tint = tones.mist, size = Measure.iconSmall)
    }
    LazyRow(contentPadding = PaddingValues(horizontal = Measure.margin, vertical = Measure.e2), horizontalArrangement = Arrangement.spacedBy(Measure.e3)) {
        items(packages, key = { it.getString("id") }) { item -> Offered(item, item.getString("repo_id") + "/" + item.getString("id") in installing, install) }
    }
}

@Composable
private fun Offered(item: JSONObject, busy: Boolean, install: ((JSONObject) -> Unit)?) = Column(Modifier.width(COVER).testTag("offered")) {
    val picture by preview(item.getString("repo_id") + "/" + item.getString("id") + "/" + item.getString("version"))
    val frame = Modifier.fillMaxWidth().aspectRatio(16f / 10f).clip(RoundedCornerShape(Measure.rControl))
    picture?.let { Image(it, null, frame, contentScale = ContentScale.Crop) } ?: Column(frame.background(tones.nightHigh)) {}
    Text(item.optString("title").ifEmpty { item.getString("id") }, Modifier.padding(top = Measure.e1), style = Type.foot, maxLines = 1, overflow = TextOverflow.Ellipsis)
    if (item.optBoolean("mature")) Text(stringResource(R.string.mature), color = tones.amber, style = Type.foot)
    if (item.optBoolean("installed")) Text(stringResource(R.string.repo_installed), color = tones.mist, style = Type.foot)
    else if (busy) Column(Modifier.testTag("installing")) {
        Text(stringResource(R.string.repo_installing), color = tones.mist, style = Type.foot)
        LinearProgressIndicator(Modifier.fillMaxWidth().padding(top = Measure.e1), color = tones.frost, trackColor = tones.nightLine)
    }
    else WordAction({ install?.invoke(item) }, enabled = install != null) { Text(stringResource(R.string.repo_install)) }
}
