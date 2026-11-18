use anyhow::Result;
use std::fs;
use std::path::Path;

pub const DESCRIPTOR: &str = "transitions/opening.json";
const WGSL: &str = "transitions/opening.wgsl";

const JSON_TEXT: &str = r##"{
  "audio": [],
  "bounded": [],
  "category": null,
  "compose": false,
  "cost": {
    "passes": 1,
    "scale": 1,
    "taps": 1
  },
  "description": null,
  "fuse": null,
  "id": "opening",
  "lighter": null,
  "mask_mode": "mix",
  "min_fps": null,
  "name": {
    "en": "Soft reveal",
    "es": "Apertura suave"
  },
  "options": [],
  "pace": null,
  "params": [
    {"id": "center", "type": "vec2", "channels": null, "help": {}, "label": {"en": "Center", "es": "Centro"}, "max": 1, "min": 0, "paint": null, "prefill": [0.5, 0.5], "tool": null, "tools": [], "unit": null, "uses": null, "values": [], "waits_for": null, "yields_to": null},
    {"id": "edge", "type": "float", "channels": null, "help": {}, "label": {"en": "Edge", "es": "Borde"}, "max": 1, "min": 0.01, "paint": null, "prefill": 0.2, "tool": null, "tools": [], "unit": null, "uses": null, "values": [], "waits_for": null, "yields_to": null}
  ],
  "passes": [],
  "quality": {},
  "reach": null,
  "solve": null,
  "stage": "transition",
  "state": null,
  "targets": {},
  "uses": [],
  "velocity": null
}
"##;

const WGSL_TEXT: &str = "fn transition(c: Crossing) -> Crossing {
    var o = c;
    let center = param(0u).xy;
    let edge = max(param(1u).x, 0.01);
    let aspect = vec2<f32>(c.resolution.x / max(c.resolution.y, 1.0), 1.0);
    let far = length(max(center, 1.0 - center) * aspect);
    let d = length((c.uv - center) * aspect) / max(far, 0.001);
    let r = c.progress * (1.0 + edge);
    let k = smoothstep(r - edge, r, d);
    o.color = vec4<f32>(mix(after(c.uv).rgb, before(c.uv).rgb, k), 1.0);
    return o;
}
";

pub fn write(dir: &Path) -> Result<String> {
    for (name, text) in [(DESCRIPTOR, JSON_TEXT), (WGSL, WGSL_TEXT)] {
        let file = dir.join(name);
        if !file.exists() {
            fs::create_dir_all(file.parent().unwrap_or(dir))?;
            fs::write(&file, text)?;
        }
    }
    Ok(DESCRIPTOR.to_owned())
}
