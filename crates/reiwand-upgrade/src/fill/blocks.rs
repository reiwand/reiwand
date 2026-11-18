use std::collections::BTreeSet;
use std::sync::LazyLock;

static BLOCKS: LazyLock<BTreeSet<String>> = LazyLock::new(|| {
    [include_str!("../../frozen/blocks.tsv"), include_str!("../../frozen/blocks-formats.tsv")]
        .into_iter()
        .flat_map(str::lines)
        .filter_map(|l| {
            l.split_once('\t').filter(|(_, class)| *class == "block").map(|(row, _)| row.to_owned())
        })
        .collect()
});

const VARIANTS: [(&str, &str); 5] = [
    ("text", "TextLayer"),
    ("model", "ModelLayer"),
    ("mesh", "MeshLayer"),
    ("light", "Light"),
    ("sound", "Sound"),
];

fn owners<'a>(def: &'a str, ctx: &str, variant: Option<&str>) -> Vec<&'a str> {
    let owner = def.split("_for_").next().unwrap_or(def);
    let mut owners = vec![owner];
    match owner {
        "Layer" => {
            owners.push("Kind");
            owners.extend(VARIANTS.iter().filter(|(kind, _)| Some(*kind) == variant).map(|(_, own)| *own));
        }
        "Operator" => owners.push("Force"),
        "" if ctx == "vortex" => owners.push("Force"),
        _ => {}
    }
    owners
}

pub(super) fn is_block(
    def: &str,
    ctx: &str,
    props: Option<&serde_json::Map<String, serde_json::Value>>,
    name: &str,
) -> bool {
    let variant = props.and_then(|p| p.get("type")).and_then(|t| t.get("const")).and_then(|c| c.as_str());
    owners(def, ctx, variant).into_iter().any(|owner| BLOCKS.contains(&format!("{owner}.{name}")))
}
