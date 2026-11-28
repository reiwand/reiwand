use super::*;

pub(super) fn accepted_shaders_run_the_helper_once(book: &Path) {
    let runs = helper_runs();
    let (d, first) =
        timed(|| Descriptor::packaged("h", &reiwand_fill::descriptors::completed(&json("h")), GOOD).unwrap());
    let (_, cached) =
        timed(|| Descriptor::packaged("h", &reiwand_fill::descriptors::completed(&json("h")), GOOD).unwrap());
    println!("an accepted shader took {first:?} with the helper and {cached:?} from the cache");
    assert_eq!(helper_runs(), runs + 1);
    guarded(&d).unwrap();
    let kept = lines(&book.with_file_name("checked-effects"));
    assert!(kept.len() == 1 && kept[0].ends_with(" ok") && kept[0].len() == 67, "{kept:?}");
    let forged = Descriptor { key: d.key ^ 2, ..d };
    assert_eq!(guarded(&forged).unwrap_err(), "its shader was not checked");
}

pub(super) fn refused_shaders_return_the_message_of_the_helper(book: &Path) {
    set_checker(None);
    let inside =
        Descriptor::packaged("h", &reiwand_fill::descriptors::completed(&json("h")), ENDLESS).unwrap_err();
    checker(|_| ());
    let runs = helper_runs();
    assert_eq!(
        Descriptor::packaged("h", &reiwand_fill::descriptors::completed(&json("h")), ENDLESS).unwrap_err(),
        inside
    );
    assert_eq!(
        Descriptor::packaged("h", &reiwand_fill::descriptors::completed(&json("h")), ENDLESS).unwrap_err(),
        inside
    );
    assert_eq!(helper_runs(), runs + 1);
    let kept = lines(&book.with_file_name("checked-effects"));
    let quoted = serde_json::Value::from(inside.as_str()).to_string();
    assert!(kept.len() == 2 && kept[1].ends_with(&format!(" refused {quoted}")), "{kept:?}");
}

pub(super) fn the_disk_cache_survives_a_new_process(book: &Path) {
    let me = std::env::current_exe().unwrap();
    let out = Command::new(me).arg("--again").arg(book).output().unwrap();
    let said = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success() && said.trim() == "runs 0",
        "{said}{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

pub(super) fn again(book: &str) {
    keep_strikes(Path::new(book));
    check_out_of_process(true);
    Descriptor::packaged("h", &reiwand_fill::descriptors::completed(&json("h")), GOOD).unwrap();
    assert!(Descriptor::packaged("h", &reiwand_fill::descriptors::completed(&json("h")), ENDLESS).is_err());
    println!("runs {}", helper_runs());
}

const TWO: &str = "@fragment fn fs_h(in: VertexOut) -> @location(0) vec4<f32> {
    return (tap(in.uv) + tap(in.uv + vec2<f32>(0.01, 0.0)) + tap(in.uv - vec2<f32>(0.01, 0.0))) / 3.0;
}
@fragment fn fs_l(in: VertexOut) -> @location(0) vec4<f32> { return tap(in.uv); }";

pub(super) fn a_lighter_variant_is_checked_with_its_effect() {
    let described = serde_json::json!({ "id": "l", "cost": { "taps": 3, "passes": 1, "scale": 1 },
        "passes": [{ "entry": "fs_h" }],
        "lighter": { "passes": [{ "entry": "fs_l" }], "cost": { "taps": 1 }, "up_to": "q1" } });
    let d = Descriptor::packaged("l", &reiwand_fill::descriptors::completed(&described.to_string()), TWO)
        .unwrap();
    guarded(&d).unwrap();
    let lite = d.lite.as_deref().expect("a lighter variant");
    guarded(lite).expect("the lighter variant of a checked package effect is drawn, not left out");
}
