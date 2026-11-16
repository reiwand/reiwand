use crate::App;
use crate::auth::{Auth, Role};
use crate::error::{Problem, Res};
use axum::http::StatusCode;
use rusqlite::{Connection, OptionalExtension, params};

pub struct Rights {
    pub private: bool,
    pub members_publish: bool,
    pub may_read: bool,
    pub may_publish: bool,
}

pub fn needed() -> Problem {
    Problem::new(
        StatusCode::UNAUTHORIZED,
        "account_needed",
        "this repository is private: sign in with an account that is a member of it",
    )
}

pub fn rights(c: &Connection, repo: &str, auth: Option<&Auth>) -> Res<Rights> {
    let sql = "SELECT r.private, r.publishing = 'members', COALESCE(m.may_read, 0), COALESCE(m.may_publish, 0) \
               FROM repos r LEFT JOIN repo_members m ON m.repo = r.name AND m.user_id = ?2 WHERE r.name = ?1";
    let user = auth.map_or(0, |a| a.user);
    let found = c
        .query_row(sql, params![repo, user], |r| {
            Ok((r.get::<_, bool>(0)?, r.get::<_, bool>(1)?, r.get::<_, bool>(2)?, r.get::<_, bool>(3)?))
        })
        .optional()?;
    let (private, members_publish, read, publish) = found.unwrap_or((false, false, false, false));
    let staff = auth.is_some_and(|a| a.role >= Role::Moderator);
    let admin = auth.is_some_and(|a| a.role >= Role::Admin);
    let open = !private && !members_publish;
    Ok(Rights {
        private,
        members_publish,
        may_read: !private || staff || read || publish,
        may_publish: auth.is_some() && (admin || publish || open),
    })
}

pub async fn read(app: &App, repo: &str, auth: Option<&Auth>) -> Res<()> {
    let (name, who) = (repo.to_string(), auth.cloned());
    let found = app.db.run(move |c| rights(c, &name, who.as_ref())).await?;
    if found.may_read { Ok(()) } else { Err(needed()) }
}

pub async fn publish(app: &App, repo: &str, auth: &Auth) -> Res<()> {
    let (name, who) = (repo.to_string(), auth.clone());
    let found = app.db.run(move |c| rights(c, &name, Some(&who))).await?;
    if found.may_publish {
        Ok(())
    } else {
        Err(Problem::new(StatusCode::FORBIDDEN, "not_allowed", "you may not publish into this repository"))
    }
}

pub async fn is_private(app: &App, repo: &str) -> bool {
    let name = repo.to_string();
    app.db.run(move |c| Ok(rights(c, &name, None)?.private)).await.unwrap_or(false)
}

pub async fn visible(app: &App, auth: Option<&Auth>, names: Vec<String>) -> Vec<(String, Rights)> {
    let who = auth.cloned();
    let found = app.db.run(move |c| {
        names.into_iter().map(|n| rights(c, &n, who.as_ref()).map(|r| (n, r))).collect::<Res<Vec<_>>>()
    });
    found.await.unwrap_or_default()
}

pub fn mode(r: &Rights) -> &'static str {
    if r.private || r.members_publish { "members" } else { "open" }
}
