package io.github.reiwand

import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.stringResource

@Composable
fun Fault(main: Main) {
    val told = main.fault.value ?: return
    val close = { main.fault.value = null }
    AlertDialog(
        onDismissRequest = close,
        confirmButton = { WordAction(close, Modifier.testTag("fault_close")) { Text(stringResource(R.string.close)) } },
        text = { Text(told, Modifier.testTag("fault")) },
    )
}
