package io.github.reiwand

import androidx.compose.runtime.Composable
import org.json.JSONObject

typealias Run = (String, suspend () -> Unit) -> Unit

fun JSONObject.text(key: String): String? = if (isNull(key)) null else getString(key)

fun host(url: String) = url.substringAfter("://").substringBefore('/').ifEmpty { url }

@Composable
fun Waiting(main: Main, text: String) = Progress(text, null) { main.act(false) { account("cancel") } }
