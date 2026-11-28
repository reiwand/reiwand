use super::*;

const COLOR: &str = r##"{"id": "c", "type": "color", "prefill": "#ffffff", "min": null, "max": null, "uses": null,
    "unit": null, "tools": [], "label": {}, "help": {}, "values": [], "channels": null, "paint": null,
    "yields_to": null, "waits_for": null, "tool": null}"##;

fn complete(params: &str) -> Json {
    let mut d = json!({ "id": "t", "params": serde_json::from_str::<Json>(params).unwrap() });
    reiwand_fill::descriptors::complete(&mut d);
    d
}

fn parse(d: &Json) -> Result<Descriptor, String> {
    Descriptor::parse(&d.to_string(), "").map_err(|e| e.to_string())
}

#[test]
fn every_bundled_descriptor_and_the_catalog_load_complete() {
    assert!(builtin().len() > 50);
    for d in builtin() {
        let text = serde_json::to_value(d).unwrap();
        let again = parse(&text).unwrap_or_else(|e| panic!("effects/{}.json: {e}", d.id));
        assert_eq!(again.params, d.params, "{}", d.id);
        for p in &d.params {
            assert_eq!(p.fits_kind(), Ok(()), "{}", d.id);
        }
    }
    for d in undrawn() {
        assert!(described(&d.id).is_some());
    }
    for d in builtin() {
        newer_params(&d.id).for_each(|p| assert_eq!(p.fits_kind(), Ok(()), "{} {}", d.id, p.id));
    }
}

#[test]
fn a_descriptor_with_a_missing_field_is_refused_naming_it() {
    let mut d = complete("[]");
    assert!(parse(&d).is_ok());
    d.as_object_mut().unwrap().remove("cost");
    assert!(parse(&d).unwrap_err().contains("missing field `cost`"));
    let mut d = complete(&format!("[{COLOR}]"));
    d["params"][0].as_object_mut().unwrap().remove("tools");
    assert!(parse(&d).unwrap_err().contains("missing field `tools`"));
    let mut d = complete("[]");
    d["passes"] = json!([{ "entry": "fs_t" }]);
    assert!(parse(&d).unwrap_err().contains("missing field"));
}

#[test]
fn a_parameter_carries_exactly_the_fields_of_its_kind() {
    let mut color: Json = serde_json::from_str(COLOR).unwrap();
    assert!(parse(&complete(&format!("[{COLOR}]"))).is_ok());
    color["min"] = json!(0);
    assert!(parse(&complete(&format!("[{color}]"))).unwrap_err().contains("has no \"min\""));
    let float = json!({"id": "f", "type": "float", "prefill": 0.5});
    let mut bare = complete(&format!("[{float}]"));
    bare["params"][0]["min"] = Json::Null;
    bare["params"][0]["max"] = Json::Null;
    let found = parse(&bare).unwrap_err();
    assert!(found.contains("needs \"min\""), "{found}");
    let list = json!({"id": "e", "type": "enum", "prefill": 0});
    assert!(parse(&complete(&format!("[{list}]"))).unwrap_err().contains("needs \"values\""));
    let full = json!({"id": "f", "type": "float", "prefill": 0.5, "min": 0, "max": 1});
    assert!(parse(&complete(&format!("[{full}]"))).is_ok());
}
