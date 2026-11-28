package io.github.reiwand

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.test.isDialog
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import com.github.takahirom.roborazzi.ExperimentalRoborazziApi
import com.github.takahirom.roborazzi.captureRoboImage
import com.github.takahirom.roborazzi.captureScreenRoboImage
import org.json.JSONArray
import org.json.JSONObject
import org.junit.Assume.assumeTrue
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.Robolectric
import org.robolectric.RobolectricTestRunner
import org.robolectric.RuntimeEnvironment
import org.robolectric.annotation.Config
import org.robolectric.annotation.GraphicsMode

/**
 * Phone screens drawn on the JVM, for captures without an emulator:
 * `./gradlew -p android testDebugUnitTest --tests '*PhoneShots*' -Pshots=DIR -x :app:cargo`.
 * Without `-Pshots` the tests are skipped.
 */
@OptIn(ExperimentalRoborazziApi::class)
@RunWith(RobolectricTestRunner::class)
@GraphicsMode(GraphicsMode.Mode.NATIVE)
@Config(sdk = [36], qualifiers = "w411dp-h891dp-night-xxhdpi", application = android.app.Application::class)
class PhoneShots {
    @get:Rule val compose = createComposeRule()
    private val dir: String? = System.getProperty("reiwand.shots")
    private val idle: Run = { _, _ -> }

    @Before
    fun only() = assumeTrue("set -Pshots=DIR to draw the screens", dir != null)

    private fun shot(name: String, content: @Composable () -> Unit) {
        // A text field's cursor blinks for ever: the clock is moved by hand instead of waiting for idle.
        compose.mainClock.autoAdvance = false
        compose.setContent { Theme { Column(Modifier.fillMaxSize().background(tones.night)) { content() } } }
        compose.mainClock.advanceTimeBy(2_000)
        captureScreenRoboImage("$dir/$name.png")
    }

    private val list get() = JSONArray(listOf(weather, broken))

    private val weather = JSONObject(
        """{"id": "skies", "title": "Open skies", "version": "1.2.0", "license": "MIT", "hosts": ["api.openskies.example"],
           "secret": {"name": "API key", "required": true, "hosts": ["api.openskies.example"]}, "sources": ["Morning sky"], "error": null}""",
    )
    private val broken = JSONObject(
        """{"id": "lens", "title": "Street lens", "version": "0.3.1", "license": "GPL-3.0-or-later", "hosts": [], "sources": [],
           "error": "the plugin did not start: a call it needs is missing"}""",
    )

    @Test
    fun pluginList() = shot("plugins-list") { PluginRows(list, {}) {} }

    @Test
    fun pluginConsent() = shot("plugins-consent") {
        val offer = JSONObject().put("plugin", weather).put("installed", JSONObject.NULL).put("added_hosts", JSONArray()).put("token", "t")
        PluginRows(list, {}) {}
        Consent(offer, "plugin.zip", idle, {}) {}
    }

    @Test
    fun pluginOpened() = shot("plugins-opened") {
        PluginRows(list, {}) {}
        Opened(broken, idle) {}
    }

    @Test
    fun pluginRemove() {
        compose.setContent {
            Theme {
                Column(Modifier.fillMaxSize().background(tones.night)) {
                    PluginRows(list, {}) {}
                    Opened(broken, idle) {}
                }
            }
        }
        compose.onNodeWithText(RuntimeEnvironment.getApplication().getString(R.string.plugin_remove)).performClick()
        compose.waitForIdle()
        captureScreenRoboImage("$dir/plugins-remove.png")
    }

    @Test
    fun pluginSource() {
        compose.mainClock.autoAdvance = false
        val kind = JSONObject(
            """{"kind": "${PLUGIN}skies", "title": "Open skies", "hosts": ["api.openskies.example"],
               "options": [{"key": "units", "type": "pick", "label": "Units", "values": ["metric", "imperial"], "default": "metric"},
                           {"key": "night", "type": "bool", "label": "Night pictures too", "default": "false"}]}""",
        )
        val main = Robolectric.buildActivity(Main::class.java).get()
        compose.setContent { Theme { Column(Modifier.fillMaxSize().background(tones.night)) { ProviderForm(main, kind, null, idle) {} } } }
        compose.mainClock.advanceTimeBy(2_000)
        // Under Robolectric a text field never lets the main loop go idle, so this form is drawn
        // with choices and switches only, and its dialog on its own.
        compose.onNode(isDialog()).captureRoboImage("$dir/plugins-source.png")
    }
}
