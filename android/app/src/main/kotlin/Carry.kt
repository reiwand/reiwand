package io.github.reiwand

import android.net.Uri
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import java.io.File
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

val CARRY_SECTIONS = listOf("repositories", "providers", "lists", "favourites", "ratings", "tags", "installed", "settings", "own", "secrets")

const val CARRY_NAME = "reiwand-config.reiwand"
const val CARRY_TYPE = "application/octet-stream"

@Composable
fun carrySection(name: String) = stringResource(
    when (name) {
        "repositories" -> R.string.carry_repositories
        "providers" -> R.string.carry_providers
        "lists" -> R.string.carry_lists
        "favourites" -> R.string.carry_favourites
        "ratings" -> R.string.carry_ratings
        "tags" -> R.string.carry_tags
        "installed" -> R.string.carry_installed
        "settings" -> R.string.carry_settings
        "own" -> R.string.carry_own
        else -> R.string.carry_secrets
    },
)

fun Main.carryFile(): File = File(cacheDir, "carry").apply { mkdirs() }.resolve("file.reiwand")

suspend fun Main.copyIn(from: Uri): File = withContext(Dispatchers.IO) {
    val to = carryFile().also { it.delete() }
    contentResolver.openInputStream(from)!!.use { input -> to.outputStream().use { input.copyTo(it) } }
    to
}

suspend fun Main.copyOut(file: File, to: Uri) = withContext(Dispatchers.IO) {
    try {
        contentResolver.openOutputStream(to, "wt")!!.use { out -> file.inputStream().use { it.copyTo(out) } }
    } finally {
        file.delete()
    }
}

suspend fun Main.dropCarry() = withContext(Dispatchers.IO) { carryFile().delete() }
