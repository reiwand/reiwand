package io.github.reiwand

import android.content.Context
import android.content.pm.PackageInfo
import android.content.pm.PackageManager
import android.os.Build
import androidx.compose.runtime.mutableStateOf
import java.io.File
import java.io.IOException
import java.security.MessageDigest

private val RELEASE = Regex("""^v(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$""")
private val INSTALLED = Regex("""^(\d+)\.(\d+)\.(\d+)""")

/** The release tag `vX.Y.Z` is newer than the installed version name (build data after `+` ignored). */
fun newerRelease(installed: String?, tag: String): Boolean {
    val new = RELEASE.find(tag)?.groupValues?.drop(1)?.map(String::toLong) ?: return false
    val old = installed?.let(INSTALLED::find)?.groupValues?.drop(1)?.map(String::toLong) ?: return true
    return new.zip(old).firstOrNull { (a, b) -> a != b }?.let { (a, b) -> a > b } ?: false
}

/** The version name is the release of that tag (build data after `+` ignored). */
fun sameRelease(name: String?, tag: String): Boolean =
    name?.let(INSTALLED::find)?.groupValues?.drop(1) == RELEASE.find(tag)?.groupValues?.drop(1) && RELEASE.matches(tag)

/** The APK of the release made for the first ABI of the device that has one of its own. */
fun apkFor(abis: List<String>): String = when (val abi = abis.firstOrNull { it == "arm64-v8a" || it == "x86_64" }) {
    null -> "reiwand-android.apk"
    else -> "reiwand-android-$abi.apk"
}

/** The checksum that SHA256SUMS lists for exactly that file name, if any. */
fun listedSum(sums: String, name: String): String? =
    sums.lineSequence().firstNotNullOfOrNull { Regex("^([0-9a-f]{64})  (.+)$").find(it)?.takeIf { m -> m.groupValues[2] == name }?.groupValues?.get(1) }

sealed interface Found {
    data object Idle : Found
    data object Checking : Found
    data object Current : Found
    data class Ready(val version: String) : Found
    data class Failed(val why: Int) : Found
}

class UpdateRefused(val why: Int) : IOException()

object AppUpdate {
    const val RELEASES = "https://github.com/reiwand/reiwand/releases"
    private const val KEPT = "app_update"
    private const val AUTO = "auto"
    private const val READY = "ready"
    private const val TIMEOUT = 30_000
    val found = mutableStateOf<Found>(Found.Idle)

    private fun kept(context: Context) = context.applicationContext.getSharedPreferences(KEPT, Context.MODE_PRIVATE)

    fun auto(context: Context) = kept(context).getBoolean(AUTO, true)

    fun setAuto(context: Context, on: Boolean) {
        kept(context).edit().putBoolean(AUTO, on).apply()
        AppCheck.schedule(context)
    }

    fun installed(context: Context): String? = context.packageManager.getPackageInfo(context.packageName, 0).versionName

    fun apk(context: Context) = File(context.noBackupFilesDir, "update/reiwand.apk")

    /** The version downloaded and checked earlier, while it is still newer than the installed one. */
    fun ready(context: Context): String? {
        val tag = kept(context).getString(READY, null)?.takeIf { apk(context).isFile && newerRelease(installed(context), it) }
        if (tag == null) forget(context)
        return tag?.removePrefix("v")
    }

    private fun forget(context: Context) {
        kept(context).edit().remove(READY).apply()
        apk(context).parentFile?.deleteRecursively()
    }

    /** Looks for a newer release and, if there is one, downloads its APK and checks it. Blocking. */
    fun check(context: Context): Found {
        if (!Net.open()) return Found.Failed(R.string.app_update_private)
        val tag = latest()
        if (!newerRelease(installed(context), tag)) return Found.Current.also { forget(context) }
        if (kept(context).getString(READY, null) == tag && ready(context) != null) return Found.Ready(tag.removePrefix("v"))
        forget(context)
        val name = apkFor(Build.SUPPORTED_ABIS.toList())
        val want = listedSum(text("$RELEASES/download/$tag/SHA256SUMS"), name) ?: throw UpdateRefused(R.string.app_update_unlisted)
        val file = apk(context).apply { parentFile?.mkdirs() }
        val part = File(file.path + ".part")
        download("$RELEASES/download/$tag/$name", part)
        if (sha256(part) != want) throw UpdateRefused(R.string.app_update_broken).also { part.delete() }
        val archive = archive(context, part) ?: throw UpdateRefused(R.string.app_update_broken).also { part.delete() }
        if (archive.packageName != context.packageName || !sameRelease(archive.versionName, tag)) throw UpdateRefused(R.string.app_update_broken).also { part.delete() }
        if (!sameSigner(context, archive)) throw UpdateRefused(R.string.app_update_signer).also { part.delete() }
        if (!part.renameTo(file)) throw IOException("could not keep the download")
        kept(context).edit().putString(READY, tag).apply()
        return Found.Ready(tag.removePrefix("v"))
    }

    private fun latest(): String {
        val c = Net.https("$RELEASES/latest")
        c.instanceFollowRedirects = false
        c.requestMethod = "HEAD"
        c.connectTimeout = TIMEOUT
        c.readTimeout = TIMEOUT
        try {
            val to = c.getHeaderField("Location") ?: throw IOException("no latest release (${c.responseCode})")
            return to.substringAfterLast("/tag/", "")
        } finally {
            c.disconnect()
        }
    }

    private fun text(url: String): String {
        val c = Net.https(url).apply { connectTimeout = TIMEOUT; readTimeout = TIMEOUT }
        try {
            if (c.responseCode != 200) throw IOException("$url: ${c.responseCode}")
            return c.inputStream.bufferedReader().use { it.readText() }
        } finally {
            c.disconnect()
        }
    }

    private fun download(url: String, to: File) {
        val c = Net.https(url).apply { connectTimeout = TIMEOUT; readTimeout = TIMEOUT }
        try {
            if (c.responseCode != 200) throw IOException("$url: ${c.responseCode}")
            c.inputStream.use { input -> to.outputStream().use { input.copyTo(it) } }
        } finally {
            c.disconnect()
        }
    }

    private fun sha256(file: File): String {
        val digest = MessageDigest.getInstance("SHA-256")
        file.inputStream().use { input ->
            val buffer = ByteArray(1 shl 16)
            while (true) {
                val n = input.read(buffer).takeIf { it > 0 } ?: break
                digest.update(buffer, 0, n)
            }
        }
        return digest.digest().joinToString("") { "%02x".format(it) }
    }

    @Suppress("DEPRECATION")
    private fun archive(context: Context, file: File): PackageInfo? =
        context.packageManager.getPackageArchiveInfo(file.path, PackageManager.GET_SIGNING_CERTIFICATES or PackageManager.GET_SIGNATURES)

    /** The download is signed with the very certificates of the installed app (Android refuses anything else). */
    @Suppress("DEPRECATION")
    private fun sameSigner(context: Context, archive: PackageInfo): Boolean {
        val own = context.packageManager.getPackageInfo(context.packageName, PackageManager.GET_SIGNING_CERTIFICATES).signingInfo
        if (own == null || own.hasMultipleSigners()) return false
        val ours = own.signingCertificateHistory.map { it.toByteArray().toList() }.toSet()
        val theirs = archive.signingInfo?.takeUnless { it.hasMultipleSigners() }?.signingCertificateHistory ?: archive.signatures?.takeIf { it.size == 1 }
        return theirs?.any { it.toByteArray().toList() in ours } == true
    }
}
