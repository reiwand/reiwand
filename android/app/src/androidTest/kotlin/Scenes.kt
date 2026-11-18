package io.github.reiwand

val TAPS_SCENE: String get() = sample("taps", "scene")
const val TAPS_SCRIPT = """import { scene } from "reiwand";
export function init() { scene.layer("button").on("click", () => { scene.layer("lamp").color = "#ff0000"; }); }
"""
val TAPS_MANIFEST: String get() = sample("taps", "manifest")
val CLIP_SCENE: String get() = sample("clip", "scene")
val CLIP_MANIFEST: String get() = sample("clip", "manifest")
val KNOBS_SCENE: String get() = sample("knobs", "scene")
val KNOBS_MANIFEST: String get() = sample("knobs", "manifest")
val PAGE_MANIFEST: String get() = sample("page", "manifest")
val LAMP_SCENE: String get() = sample("lamp", "scene")
val LAMP_MANIFEST: String get() = sample("lamp", "manifest")
val FOG_SCENE: String get() = sample("fog", "scene")
val FOG_MANIFEST: String get() = sample("fog", "manifest")
val SHADOW_SCENE: String get() = sample("shadow", "scene")
val SHADOW_MANIFEST: String get() = sample("shadow", "manifest")
val STAGE_SCENE: String get() = sample("stage", "scene")
val STAGE_MANIFEST: String get() = sample("stage", "manifest")
val FLOWN_SCENE: String get() = sample("flown", "scene")
val FLOWN_MANIFEST: String get() = sample("flown", "manifest")
val SHAFTS_SCENE: String get() = sample("shafts", "scene")
val SHAFTS_MANIFEST: String get() = sample("shafts", "manifest")
val MIXES_SCENE: String get() = sample("mixes", "scene")
val MIXES_MANIFEST: String get() = sample("mixes", "manifest")
val RAIN_SCENE: String get() = sample("rain", "scene")
val RAIN_MANIFEST: String get() = sample("rain", "manifest")
val FLASH_SCENE: String get() = sample("flash", "scene")
val FLASH_MANIFEST: String get() = sample("flash", "manifest")
val LEANS_SCENE: String get() = sample("leans", "scene")
val LEANS_MANIFEST: String get() = sample("leans", "manifest")
val PART_SCENE: String get() = sample("part", "scene")
val PART_MANIFEST: String get() = sample("part", "manifest")
val RIG_SCENE: String get() = sample("rig", "scene")
val RIG_MANIFEST: String get() = sample("rig", "manifest")
val SWING_SCENE: String get() = sample("swing", "scene")
val SWING_MANIFEST: String get() = sample("swing", "manifest")
const val RIG_GLB = "Z2xURgIAAABsBgAAmAQAAEpTT057ImFjY2Vzc29ycyI6W3siYnVmZmVyVmlldyI6MCwiY29tcG9uZW50VHlwZSI6NTEyNiwiY291" +
    "bnQiOjYsIm1heCI6WzMyLjAsMTYuMCwwLjBdLCJtaW4iOlstMzIuMCwtMTYuMCwwLjBdLCJ0eXBlIjoiVkVDMyJ9LHsiYnVmZmVy" +
    "VmlldyI6MSwiY29tcG9uZW50VHlwZSI6NTEyNiwiY291bnQiOjYsInR5cGUiOiJWRUMyIn0seyJidWZmZXJWaWV3IjoyLCJjb21w" +
    "b25lbnRUeXBlIjo1MTI1LCJjb3VudCI6MTIsInR5cGUiOiJTQ0FMQVIifSx7ImJ1ZmZlclZpZXciOjMsImNvbXBvbmVudFR5cGUi" +
    "OjUxMjMsImNvdW50Ijo2LCJ0eXBlIjoiVkVDNCJ9LHsiYnVmZmVyVmlldyI6NCwiY29tcG9uZW50VHlwZSI6NTEyNiwiY291bnQi" +
    "OjYsInR5cGUiOiJWRUM0In0seyJidWZmZXJWaWV3Ijo1LCJjb21wb25lbnRUeXBlIjo1MTI2LCJjb3VudCI6MiwidHlwZSI6Ik1B" +
    "VDQifV0sImFzc2V0Ijp7InZlcnNpb24iOiIyLjAifSwiYnVmZmVyVmlld3MiOlt7ImJ1ZmZlciI6MCwiYnl0ZUxlbmd0aCI6NzIs" +
    "ImJ5dGVPZmZzZXQiOjB9LHsiYnVmZmVyIjowLCJieXRlTGVuZ3RoIjo0OCwiYnl0ZU9mZnNldCI6NzJ9LHsiYnVmZmVyIjowLCJi" +
    "eXRlTGVuZ3RoIjo0OCwiYnl0ZU9mZnNldCI6MTIwfSx7ImJ1ZmZlciI6MCwiYnl0ZUxlbmd0aCI6NDgsImJ5dGVPZmZzZXQiOjE2" +
    "OH0seyJidWZmZXIiOjAsImJ5dGVMZW5ndGgiOjk2LCJieXRlT2Zmc2V0IjoyMTZ9LHsiYnVmZmVyIjowLCJieXRlTGVuZ3RoIjox" +
    "MjgsImJ5dGVPZmZzZXQiOjMxMn1dLCJidWZmZXJzIjpbeyJieXRlTGVuZ3RoIjo0NDB9XSwibWVzaGVzIjpbeyJleHRyYXMiOnsi" +
    "dGFyZ2V0TmFtZXMiOltdfSwibmFtZSI6ImJvZHkiLCJwcmltaXRpdmVzIjpbeyJhdHRyaWJ1dGVzIjp7IkpPSU5UU18wIjozLCJQ" +
    "T1NJVElPTiI6MCwiVEVYQ09PUkRfMCI6MSwiV0VJR0hUU18wIjo0fSwiaW5kaWNlcyI6MiwidGFyZ2V0cyI6W119XX1dLCJub2Rl" +
    "cyI6W3sibWVzaCI6MCwic2tpbiI6MH0seyJjaGlsZHJlbiI6WzJdLCJuYW1lIjoicm9vdCIsInRyYW5zbGF0aW9uIjpbLTE2LjAs" +
    "MC4wLDAuMF19LHsibmFtZSI6ImFybSIsInRyYW5zbGF0aW9uIjpbMTYuMCwwLjAsMC4wXX1dLCJzY2VuZSI6MCwic2NlbmVzIjpb" +
    "eyJub2RlcyI6WzAsMV19XSwic2tpbnMiOlt7ImludmVyc2VCaW5kTWF0cmljZXMiOjUsImpvaW50cyI6WzEsMl19XX0gICC4AQAA" +
    "QklOAAAAAMIAAIBBAAAAAAAAAAAAAIBBAAAAAAAAAAAAAIDBAAAAAAAAAMIAAIDBAAAAAAAAAEIAAIBBAAAAAAAAAEIAAIDBAAAA" +
    "AAAAAAAAAAAAAAAAPwAAAAAAAAA/AACAPwAAAAAAAIA/AACAPwAAAAAAAIA/AACAPwAAAAABAAAAAgAAAAAAAAACAAAAAwAAAAEA" +
    "AAAEAAAABQAAAAEAAAAFAAAAAgAAAAAAAQAAAAAAAAABAAAAAAAAAAEAAAAAAAAAAQAAAAAAAAABAAAAAAAAAAEAAAAAAAAAgD83" +
    "vnQsAAAAAAAAAAAAAAA/AAAAPwAAAAAAAAAAAAAAPwAAAD8AAAAAAAAAAAAAgD83vnQsAAAAAAAAAAAAAAA/AAAAPwAAAAAAAAAA" +
    "AAAAPwAAAD8AAAAAAAAAAAAAgD8AAAAAAAAAAAAAAAAAAAAAAACAPwAAAAAAAAAAAAAAAAAAAAAAAIA/AAAAAAAAgEEAAACAAAAA" +
    "gAAAgD8AAIA/AAAAAAAAAAAAAAAAAAAAAAAAgD8AAAAAAAAAAAAAAAAAAAAAAACAPwAAAAAAAACAAAAAgAAAAIAAAIA/"
