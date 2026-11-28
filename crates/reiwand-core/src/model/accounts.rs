model! {
    pub struct ServerAccountRepo {
        pub url: String,
        #[serde(deserialize_with = "crate::explicit::null")]
        pub repo_id: Option<String>,
    }

    pub struct ServerAccount {
        pub id: String,
        pub server: String,
        pub user: String,
        pub label: String,
        pub pending: bool,
        pub repos: Vec<ServerAccountRepo>,
    }

    pub struct AccountsReply { pub model: u32, pub serial: u64, pub accounts: Vec<ServerAccount> }

    pub struct SignIn { pub user_code: String, pub verification_uri: String, pub job: u64 }
}
