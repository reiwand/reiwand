package io.github.reiwand

import android.content.Context
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import java.io.IOException
import kotlin.concurrent.thread

val ABOUT = listOf(
    Line(R.string.app_version_name, Block.About) { AppVersion() },
    Line(R.string.app_update_auto, Block.About) { AppAuto() },
)

@Composable
private fun AppVersion() {
    val context = LocalContext.current
    val found by AppUpdate.found
    LaunchedEffect(Unit) {
        if (found == Found.Idle) AppUpdate.ready(context)?.let { AppUpdate.found.value = Found.Ready(it) }
    }
    val version = remember { AppUpdate.installed(context).orEmpty().substringBefore('+') }
    val sub = when (val f = found) {
        Found.Idle -> null
        Found.Checking -> stringResource(R.string.app_update_checking)
        Found.Current -> stringResource(R.string.app_update_current)
        is Found.Ready -> stringResource(R.string.app_update_ready, f.version)
        is Found.Failed -> stringResource(f.why)
    }
    Setting(stringResource(R.string.app_version, version), Modifier.testTag("app_version"), sub = sub, trailing = {
        when (val f = found) {
            is Found.Ready -> WordAction({ AppInstall.start(context) }, Modifier.testTag("app_install")) {
                Text(stringResource(R.string.app_update_install, f.version))
            }
            else -> WordAction({ look(context) }, Modifier.testTag("app_look"), enabled = f != Found.Checking) {
                Text(stringResource(R.string.app_update_look))
            }
        }
    })
}

@Composable
private fun AppAuto() {
    val context = LocalContext.current
    var on by remember { mutableStateOf(AppUpdate.auto(context)) }
    Switch(stringResource(R.string.app_update_auto), on, stringResource(R.string.app_update_auto_sub)) {
        on = it
        thread(isDaemon = true) { AppUpdate.setAuto(context, it) }
    }
}

private fun look(context: Context) {
    AppUpdate.found.value = Found.Checking
    thread(isDaemon = true) {
        AppUpdate.found.value = try {
            AppUpdate.check(context)
        } catch (refused: UpdateRefused) {
            Found.Failed(refused.why)
        } catch (_: IOException) {
            Found.Failed(R.string.app_update_offline)
        }
    }
}
