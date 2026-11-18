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

pub(super) fn is_block(def: &str, name: &str) -> bool {
    let owner = def.split("_for_").next().unwrap_or(def);
    BLOCKS.contains(&format!("{owner}.{name}"))
}
