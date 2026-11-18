mod common;
#[test]
#[ignore]
fn convert_listed() {
    let list = std::fs::read_to_string(common::repo().join("../../../../.wf/s/e17s2-nulls.txt")).unwrap();
    for line in list.lines() {
        if line.contains("newer.json") || line.contains("features.json") { continue; }
        let p = common::repo().join(line);
        let done = reiwand_upgrade::scenes::scene(&p).unwrap();
        println!("{done} {line}");
    }
}
