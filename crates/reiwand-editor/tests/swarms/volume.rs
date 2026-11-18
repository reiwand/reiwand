use super::*;

fn ball() -> Value {
    json!({ "sphere": { "center": [100, 60], "depth": 30, "radius": [40, 20, 80], "inner": 0.5, "sign": [0, 0, 1] } })
}

#[test]
fn a_sphere_shape_is_stored_whole_and_shown_with_its_handles() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    edit(&mut p, "shape", json!({ "emitter": 0, "shape": ball() })).unwrap();
    assert_eq!(
        p.doc.pointer("/scene/particles/flakes/emitters/0/shape").unwrap(),
        &super::whole::same(&p, "/scene/particles/flakes/emitters/0/shape", ball())
    );
    let h = &handles(&p)["emitters"][0];
    assert_eq!(h["kind"], "sphere");
    let near = |a: &Value, x: f64, y: f64| {
        (a[0].as_f64().unwrap() - x).abs() < 0.05 && (a[1].as_f64().unwrap() - y).abs() < 0.05
    };
    assert!(near(&h["center"], 100.0, 60.0) && near(&h["east"], 140.0, 60.0), "{h}");
    assert!(near(&h["south"], 100.0, 80.0) && near(&h["inner_east"], 120.0, 60.0), "{h}");
    assert_eq!(
        (h["radius"][2].clone(), h["depth"].clone(), h["sign"].clone()),
        (json!(80.0), json!(30), json!([0, 0, 1]))
    );
}

#[test]
fn a_sphere_without_every_field_is_refused() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    for field in ["center", "depth", "radius", "inner"] {
        let mut shape = ball();
        shape["sphere"].as_object_mut().unwrap().remove(field);
        assert!(edit(&mut p, "shape", json!({ "emitter": 0, "shape": shape })).is_err(), "{field}");
    }
}
