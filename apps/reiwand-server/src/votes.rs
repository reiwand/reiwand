use crate::auth::Auth;
use crate::db::{HOUR, now};
use crate::error::{Input, Problem, Res};
use crate::lang::t;
use crate::scope::{Scope, at};
use crate::{Reply, Router, St};
use axum::Json;
use axum::extract::{Query, State};
use axum::routing::get;
use rusqlite::{Connection, params};
use serde::Deserialize;
use serde_json::{Map, Value, json};

const PER_HOUR: i64 = 60;
const COUNTS: &str = "SELECT COALESCE(SUM(value = 1), 0), COALESCE(SUM(value = -1), 0) FROM votes \
                      WHERE repo = ?2 AND package_id = ?1";

pub fn routes() -> Router {
    let router = at(Router::new(), "/api/v1/packages/{id}/vote", get(mine).post(vote));
    at(router, "/api/v1/votes", get(counts))
}

#[derive(Deserialize)]
struct Vote {
    vote: String,
}

#[derive(Deserialize)]
struct Ids {
    #[serde(default)]
    ids: String,
}

fn published(c: &Connection, repo: &str, id: &str) -> Res<()> {
    let sql = "SELECT COUNT(*) FROM versions v JOIN packages p ON p.repo = v.repo AND p.id = v.package_id \
               WHERE v.repo = ?2 AND v.package_id = ?1 AND v.state = 'published' AND p.hidden IS NULL";
    match c.query_row(sql, [id, repo], |r| r.get::<_, i64>(0))? {
        0 => Err(Problem::missing("published package")),
        _ => Ok(()),
    }
}

fn tally(c: &Connection, repo: &str, id: &str, user: Option<i64>) -> Res<Value> {
    let (up, down): (i64, i64) = c.query_row(COUNTS, [id, repo], |r| Ok((r.get(0)?, r.get(1)?)))?;
    let mut found = json!({"id": id, "repo": repo, "up": up, "down": down});
    if let Some(user) = user {
        let sql = "SELECT COALESCE((SELECT value FROM votes WHERE repo = ?3 AND package_id = ?1 AND user_id = ?2), 0)";
        let mine: i64 = c.query_row(sql, params![id, user, repo], |r| r.get(0))?;
        found["mine"] = json!(match mine {
            1 => "up",
            -1 => "down",
            _ => "none",
        });
    }
    Ok(found)
}

async fn mine(State(app): St, auth: Auth, sc: Scope) -> Reply {
    let found = app.db.run(move |c| {
        published(c, &sc.repo, &sc.id).and_then(|()| tally(c, &sc.repo, &sc.id, Some(auth.user)))
    });
    Ok(Json(found.await?))
}

async fn vote(State(app): St, auth: Auth, sc: Scope, Input(v): Input<Vote>) -> Reply {
    let (repo, id) = (sc.repo, sc.id);
    let value = match v.vote.as_str() {
        "up" => 1,
        "down" => -1,
        "none" => 0,
        _ => return Err(Problem::bad("vote: up, down or none")),
    };
    let user = auth.user;
    let found = app.db.run(move |c| {
        published(c, &repo, &id)?;
        let t = now();
        c.execute("DELETE FROM vote_changes WHERE at < ?1", [t - HOUR])?;
        let sql = "SELECT COUNT(*) FROM vote_changes WHERE user_id = ?1";
        if c.query_row(sql, [user], |r| r.get::<_, i64>(0))? >= PER_HOUR {
            return Err(Problem::busy(600));
        }
        c.execute("INSERT INTO vote_changes (user_id, at) VALUES (?1, ?2)", params![user, t])?;
        match value {
            0 => c.execute(
                "DELETE FROM votes WHERE repo = ?3 AND package_id = ?1 AND user_id = ?2",
                params![id, user, repo],
            )?,
            _ => c.execute(
                "INSERT INTO votes (package_id, user_id, value, at, repo) VALUES (?1, ?2, ?3, ?4, ?5) \
                 ON CONFLICT (repo, package_id, user_id) DO UPDATE SET value = ?3, at = ?4",
                params![id, user, value, t, repo],
            )?,
        };
        tally(c, &repo, &id, Some(user))
    });
    Ok(Json(found.await?))
}

async fn counts(State(app): St, auth: Option<Auth>, sc: Scope, Query(q): Query<Ids>) -> Reply {
    let ids: Vec<String> = q.ids.split(',').filter(|i| !i.is_empty()).map(String::from).collect();
    if ids.len() > 100 || !ids.iter().all(|i| reiwand_format::valid_id(i)) {
        return Err(Problem::bad("ids: at most 100 package ids separated by commas"));
    }
    let items = app.db.run(move |c| {
        let mut items = Map::new();
        for id in ids {
            let mut found = tally(c, &sc.repo, &id, auth.as_ref().map(|a| a.user))?;
            found.as_object_mut().map(|o| (o.remove("id"), o.remove("repo")));
            items.insert(id, found);
        }
        Ok(items)
    });
    Ok(Json(json!({"items": items.await?})))
}

pub fn ballot(base: &str, id: &str, latest: &Value, signed: bool) -> maud::Markup {
    let (up, down) = (&latest["votes"]["up"], &latest["votes"]["down"]);
    maud::html! {
        p {
            @if latest["featured"] == true { strong { (t("Featured by this server.")) " " } }
            "▲ " (up) " · ▼ " (down) " " (t("votes, counted without any list of who voted."))
        }
        @if signed { @for (vote, label) in [("up", t("Vote up")), ("down", t("Vote down")), ("none", t("Withdraw my vote"))] {
            form.inline method="post" action={ "/api/v1" (base) "/packages/" (id) "/vote?back=" (base) "/packages/" (id) } {
                input type="hidden" name="vote" value=(vote);
                (crate::ui::button(crate::ui::Kind::Plain, label))
            }
        } }
    }
}
