package io.github.reiwand

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.LiveRegionMode
import androidx.compose.ui.semantics.liveRegion
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

@Composable
fun cut(): Boolean {
    LaunchedEffect(Unit) { withContext(Dispatchers.IO) { Net.private() } }
    return Net.cut.value
}

@Composable
fun CutLine(modifier: Modifier = Modifier) {
    if (!cut()) return
    val context = LocalContext.current
    val scope = rememberCoroutineScope()
    Row(
        modifier.fillMaxWidth().padding(horizontal = Measure.margin, vertical = Measure.e1).testTag("private_line"),
        horizontalArrangement = Arrangement.spacedBy(Measure.e2),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Glyph(Ph.WifiSlash, null, tint = tones.mist, size = 16.dp)
        Text(stringResource(R.string.private_mode_line), Modifier.weight(1f).semantics { liveRegion = LiveRegionMode.Polite }, tones.mist, style = Type.foot)
        WordAction({
            scope.launch {
                withContext(Dispatchers.IO) {
                    Net.choose(false)
                    Network.changed(context)
                }
                Render.post { Wallpaper.reload(false) }
            }
        }, Modifier.testTag("private_off")) { Text(stringResource(R.string.private_mode_off)) }
    }
}
