package io.github.reiwand

import androidx.compose.runtime.mutableStateOf
import java.io.IOException
import java.net.URI
import javax.net.ssl.HttpsURLConnection

object Net {
    const val PRIVATE = "private_mode"
    val cut = mutableStateOf(false)

    fun open() = !private()

    fun private() = (runCatching { Native.setting(PRIVATE, null) }.getOrNull() == "on").also { if (cut.value != it) cut.value = it }

    fun choose(on: Boolean) {
        Native.setting(PRIVATE, if (on) "on" else "off")
        cut.value = on
    }

    fun https(url: String): HttpsURLConnection {
        if (private()) throw IOException(CUT)
        return URI(url).toURL().openConnection() as HttpsURLConnection
    }

    private const val CUT = "private mode: the network is cut"
}
