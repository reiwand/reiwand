use super::counted::allocations;
use super::{Net, SIZE, TIMES};
use reiwand_format::{Limits, build, open};
use reiwand_render::{Bench, Level, Renderer, SceneAssets, Stamping, WARMUP, scene_bench};
use reiwand_scene::{Quality, Runtime, Scene, Tier, Values, effective};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::sync::Arc;

const SHORT: usize = WARMUP + 20;
const LONG: usize = WARMUP + 60;

fn scenes() -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenes");
    let mut dirs: Vec<_> = std::fs::read_dir(root)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.join("scene.json").is_file())
        .collect();
    dirs.sort();
    dirs
}

fn load(source: &Path, out: &Path) -> (Scene, Values, SceneAssets) {
    build(source, out, &Limits::default()).unwrap();
    let mut package = open(out, &Limits::default()).unwrap();
    let scene = package.scene().unwrap();
    let mut read = |path: &str| package.read_file(path).map_err(|e| e.to_string());
    let assets =
        SceneAssets::read(&scene, &mut read).and_then(|a| a.with_scripts(&scene, &mut read)).unwrap();
    (scene, effective(&package.manifest.properties, &[]), assets)
}

fn bench(
    r: &Renderer,
    scene: &Arc<Scene>,
    props: &Values,
    assets: &SceneAssets,
    frames: usize,
) -> (Value, u64) {
    let bench = Bench {
        frames,
        size: SIZE,
        level: Level::Fixed(Quality::Q3),
        tier: Some(Tier::T2),
        stamps: Stamping::Off,
        ..Bench::default()
    };
    let runtime = Runtime::new(scene.clone(), props.clone());
    let (report, made) = allocations(|| scene_bench(r, runtime, assets.clone(), &bench));
    (report.unwrap(), made)
}

fn counters(report: &Value, per_frame: u64) -> Value {
    let n = |v: &Value| v.as_f64().unwrap_or(0.0);
    json!({
        "needed": n(&report["needed"]),
        "passes": n(&report["passes"]),
        "draws": n(&report["draws"]),
        "textures": n(&report["textures"]),
        "created": n(&report["created_after_warmup"]),
        "compiled": n(&report["compiled_after_warmup"]),
        "particles": n(&report["particles"]["slots"]),
        "steps": n(&report["particles"]["steps"]),
        "allocations": per_frame as f64,
        "ms": (n(&report["render_ms"]["mean"]) * 100.0).round() / 100.0,
    })
}

pub fn check(r: &Renderer, net: &mut Net) {
    let dir = tempfile::tempdir().unwrap();
    for source in scenes() {
        let name = source.file_name().unwrap().to_string_lossy().into_owned();
        let (scene, props, assets) = load(&source, &dir.path().join(format!("{name}.wpak")));
        let mut frames = Vec::new();
        let drawn = r.stills(&scene, &assets, &props, (0.0, "en"), (SIZE.0, &TIMES), |i| frames.push(i));
        if let Err(e) = drawn {
            net.fail(format!("{name}: not drawn: {e}"));
            continue;
        }
        let mut again = Vec::new();
        r.stills(&scene, &assets, &props, (0.0, "en"), (SIZE.0, &TIMES[..1]), |i| again.push(i)).unwrap();
        if again[0] != frames[0] {
            net.fail(format!("{name}: two drawings of the first frame differ"));
        }
        net.picture(&format!("scene-{name}"), &frames);
        let scene = Arc::new(scene);
        let (_, short) = bench(r, &scene, &props, &assets, SHORT);
        let (report, long) = bench(r, &scene, &props, &assets, LONG);
        let per_frame = long.saturating_sub(short).div_ceil((LONG - SHORT) as u64);
        net.counters(&format!("scene-{name}"), counters(&report, per_frame));
    }
}
