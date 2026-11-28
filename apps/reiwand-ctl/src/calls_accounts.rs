pub const ACCOUNT_CALLS: &[(&str, &str, &str, &[&str])] = &[
    ("Accounts1", "List", "account ls", &[]),
    ("Accounts1", "SignIn", "account sign-in", &["URL"]),
    ("Accounts1", "Use", "account use", &["REPO", "ACCOUNT"]),
    ("Accounts1", "Unlink", "account unlink", &["REPO"]),
    ("Accounts1", "SignOut", "account sign-out", &["ACCOUNT"]),
    ("Accounts1", "Remove", "account rm", &["ACCOUNT"]),
];
