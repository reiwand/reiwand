use crate::Facts;
use reiwand_scene::{Quality, Scene};

pub fn weigh(facts: &mut Facts, scene: &Scene) {
    facts.heavy = scene.heavy_cost().map(|(cost, budget)| {
        let layers: Vec<String> =
            scene.heaviest(Quality::Q2, 3).iter().map(|(id, units)| format!("{id} ({units:.1})")).collect();
        (format!("{cost:.1}"), budget.to_string(), layers.join(", "))
    });
}
