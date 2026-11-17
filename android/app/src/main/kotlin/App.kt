package io.github.reiwand

import androidx.activity.compose.BackHandler
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.res.stringResource

enum class Aside { Sources, Explore, Account, Export, Import }

private fun Aside.back(): Aside? = when (this) {
    Aside.Export, Aside.Import -> Aside.Account
    else -> null
}

@Composable
fun App(main: Main) = Theme {
    var tab by rememberSaveable { mutableStateOf(Tab.Library) }
    var item by rememberSaveable { mutableStateOf<String?>(null) }
    var playlist by rememberSaveable { mutableStateOf<String?>(null) }
    var aside by rememberSaveable { mutableStateOf<Aside?>(null) }
    var sourcesFrom by rememberSaveable { mutableStateOf<Aside?>(null) }
    var every by rememberSaveable { mutableStateOf(false) }
    var address by rememberSaveable { mutableStateOf("") }
    BackHandler(item != null || playlist != null || aside != null || tab != Tab.Library) {
        when {
            item != null -> item = null
            aside != null -> aside = when (aside) {
                Aside.Sources -> sourcesFrom
                else -> aside?.back()
            }
            playlist != null -> playlist = null
            else -> tab = Tab.Library
        }
    }
    val jump = Jump.to.value
    LaunchedEffect(jump) {
        jump?.let {
            tab = it
            playlist = null
            item = null
            Jump.to.value = null
        }
    }
    val pending = main.pendingAddress.value
    LaunchedEffect(pending) {
        if (pending.isNotBlank()) {
            address = pending
            sourcesFrom = null
            aside = Aside.Sources
            item = null
            main.pendingAddress.value = ""
        }
    }
    LaunchedEffect(aside) { if (aside != Aside.Sources) address = "" }
    if (main.placing.value) Placing(main)
    FirstRun(main)
    EmptyFolder(main)
    val (shown, list, away) = Triple(item, playlist, aside)
    val sources = {
        sourcesFrom = null
        aside = Aside.Sources
    }
    val veil = when {
        shown == null && away == Aside.Account -> Veil.Strip
        shown != null || away != null -> Veil.Panel
        tab == Tab.Library && !every -> Veil.Title
        else -> Veil.Shelf
    }
    val bar: (@Composable () -> Unit)? = if (shown == null && away == null) {
        {
            Tabs(tab, Updates.count.intValue) {
                tab = it
                playlist = null
            }
        }
    } else {
        null
    }
    Shell(main, veil, bar) {
        when {
            shown != null -> Detail(main, shown, { item = it }) { item = null }
            away == Aside.Sources -> Sources(main, { aside = sourcesFrom }, address) {
                main.search.value = it
                aside = null
                every = true
                tab = Tab.Library
            }
            away == Aside.Explore -> Explore(main, { aside = null }) { sourcesFrom = Aside.Explore; aside = Aside.Sources }
            away == Aside.Account -> Account(main, { aside = it }) { aside = null }
            away == Aside.Export -> Export(main) { aside = Aside.Account }
            away == Aside.Import -> Import(main) { aside = Aside.Account }
            tab == Tab.Library && !every -> Shelf(main, sources, { aside = Aside.Explore }, { search -> if (search) main.search.value = ""; every = true }) { item = it }
            tab == Tab.Library -> Library(main, sources, { aside = Aside.Explore }, { every = false }) { item = it }
            tab == Tab.Lists && list != null -> Entries(main, list, { item = it }) { playlist = null }
            tab == Tab.Lists -> Playlists(main) { playlist = it }
            else -> Settings(main) { aside = Aside.Account }
        }
    }
}
