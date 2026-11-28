pub const CARRY_CALLS: &[(&str, &str, &str, &[&str])] = &[
    ("Carry1", "Export", "export", &["FILE", "=options of --own, --secrets and the password"]),
    ("Carry1", "PreviewImport", "import", &["FILE", "=password"]),
    ("Carry1", "ImportChosen", "import", &["FILE", "=password", "=choices of --take, --lists and --suffix"]),
];
