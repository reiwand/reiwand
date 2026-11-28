package io.github.reiwand

import androidx.compose.runtime.Composable
import org.json.JSONObject

typealias Run = (String, suspend () -> Unit) -> Unit

fun JSONObject.text(key: String): String? = if (isNull(key)) null else getString(key)

fun host(url: String) = url.substringAfter("://").substringBefore('/').ifEmpty { url }

@Composable
fun Waiting(main: Main, text: String) = Progress(text, null) { main.act { account("cancel") } }

fun size(bytes: Long) = if (bytes >= 1_000_000) "%.1f MB".format(bytes / 1e6) else "%d KB".format((bytes + 999) / 1000)
