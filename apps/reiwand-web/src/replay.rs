use crate::state::Live;
use serde_json::{Value, json};

pub fn missed(live: &Live, view: u32) -> Vec<Value> {
    let media = live.heard.clone().filter(|_| live.media).map(|last| json!({ "media": last }));
    let lights = live.lights.contains(&view).then(|| json!({ "lights": true }));
    let values = live.views.get(&view).filter(|v| !v.is_empty()).map(|v| json!({ "properties": v }));
    values.into_iter().chain(media).chain(lights).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_page_loading_after_the_lights_were_granted_hears_it() {
        let mut live = Live::default();
        assert!(missed(&live, 3).is_empty());
        live.views.entry(3).or_default().insert("speed".into(), json!(3));
        assert_eq!(missed(&live, 3), vec![json!({ "properties": { "speed": 3 } })]);
        live.views.remove(&3);
        live.lights.insert(3);
        assert_eq!(missed(&live, 3), vec![json!({ "lights": true })]);
        assert!(missed(&live, 4).is_empty(), "another view was lit");
        live.media = true;
        live.heard = Some(json!({ "title": "a" }));
        assert_eq!(missed(&live, 3).len(), 2);
    }
}
