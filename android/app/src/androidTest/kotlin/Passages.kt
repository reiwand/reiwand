package io.github.reiwand

import android.graphics.Color
import java.io.File
import org.json.JSONArray
import org.json.JSONObject
import org.junit.Assert.assertTrue

object Passages {
    const val RED = 0xffff0000.toInt()
    const val BLUE = 0xff0000ff.toInt()
    const val GREEN = 0xff00ff00.toInt()

    fun install() {
        val root = File(storage(), "passages").apply { deleteRecursively() }
        SCENES.forEach { (name, scene) -> write(File(root, name), name, scene) }
        val reply = Native.library(JSONObject().put("op", "examples").put("dir", root.path).toString())
        assertTrue(reply, reply.startsWith("["))
    }

    fun pick(title: String) {
        val items = JSONArray(Native.library(JSONObject().put("op", "items").put("query", "kind:scene").toString()))
        val uid = (0 until items.length()).map(items::getJSONObject).first { it.getString("title") == title }.getString("uid")
        Native.library(JSONObject(mapOf("op" to "pick", "choice" to choice("item", uid))).toString())
    }

    fun marked(drawn: Int) = Color.red(drawn) > 0xc0 && Color.green(drawn) > 0xc0 && Color.blue(drawn) < 0x40

    fun mixed(drawn: Int) = Color.red(drawn) in 0x30..0xe0 && Color.blue(drawn) in 0x30..0xe0 && Color.green(drawn) < 0x30

    private fun write(dir: File, name: String, scene: String) {
        dir.mkdirs()
        File(dir, "scene.json").writeText(scene)
        File(dir, "manifest.json").writeText(
            sample("passage", "manifest").replace("org.example.passage", "org.example.passage.${name.lowercase()}").replace("TITLE", name),
        )
        File(dir, "t").mkdirs()
        File(dir, "t/mark.json").writeText(MARK)
        File(dir, "t/mark.wgsl").writeText(if (name == "Broken") BROKEN else WGSL)
    }

    private val MARK: String get() = sample("passage", "descriptor")
    private const val WGSL = """fn transition(c: Crossing) -> Crossing {
    var o = c;
    o.color = select(after(c.uv), vec4<f32>(1.0, 1.0, 0.0, 1.0), c.progress < 0.999);
    return o;
}
"""
    private const val BROKEN = """fn transition(c: Crossing) -> Crossing {
    return 1.0;
}
"""
    private fun scene(color: String, ms: Int?) =
        if (ms == null) sample("passplain", "scene").replace("#ff0000", color)
        else sample("passmark", "scene").replace("#0000ff", color).replace("\"duration_ms\": 1500", "\"duration_ms\": $ms")
    private val SCENES = mapOf(
        "Red" to scene("#ff0000", null),
        "Mark" to scene("#0000ff", 1500),
        "Long" to scene("#0000ff", 6000),
        "Broken" to scene("#0000ff", 1500),
        "Green" to scene("#00ff00", null),
    )
}
