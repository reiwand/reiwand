package io.github.reiwand

import android.os.Handler
import android.os.Looper
import androidx.webkit.ProxyConfig
import androidx.webkit.ProxyController
import androidx.webkit.WebViewFeature
import java.util.concurrent.Executor

object Reach {
    private const val NOWHERE = "127.0.0.1:9"
    private val main = Handler(Looper.getMainLooper())
    private val reaching = mutableMapOf<Any, List<String>>()

    fun open(page: Any, sites: List<String>, then: () -> Unit) {
        reaching[page] = sites
        apply(then)
    }

    fun close(page: Any) {
        if (reaching.remove(page) != null) apply {}
    }

    private fun apply(then: () -> Unit) {
        if (!WebViewFeature.isFeatureSupported(WebViewFeature.PROXY_OVERRIDE)) return then()
        val config = ProxyConfig.Builder().addProxyRule(NOWHERE).removeImplicitRules()
        reaching.values.flatten().distinct().forEach { config.addBypassRule(it) }
        ProxyController.getInstance().setProxyOverride(config.build(), Executor { main.post(it) }, then)
    }
}
