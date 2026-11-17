use super::report::Report;
use crate::daemon::Daemon;
use crate::error::{Error, Result};
use reiwand_carry::{Change, Item};
use reiwand_library::Rules;
use std::collections::HashMap;

fn resolved<'a>(uids: &'a HashMap<Item, String>, items: &[Item], report: &mut Report) -> Vec<&'a str> {
    let mut found = Vec::new();
    for item in items {
        match uids.get(item) {
            Some(uid) => found.push(uid.as_str()),
            None => report.unresolved += 1,
        }
    }
    found
}

impl Daemon {
    pub fn carry_library(&self, changes: &[Change], report: &mut Report) -> Result<()> {
        let uids = self.carry_state()?.uids;
        self.store.write(|lib| {
            for change in changes {
                match change {
                    Change::AddList(list) => {
                        let found = resolved(&uids, &list.items, report);
                        match lib.create_playlist(&list.name, None, &Rules::default()) {
                            Ok(uid) => {
                                lib.add_to_playlist(&uid, &found)?;
                                report.add("lists");
                            }
                            Err(e) => report.fail("lists", &list.name, &Error::from(e)),
                        }
                    }
                    Change::JoinList { name, items } => {
                        let found = resolved(&uids, items, report);
                        match lib.playlists()?.into_iter().find(|p| &p.name == name) {
                            Some(p) => {
                                lib.add_to_playlist(&p.uid, &found)?;
                                report.add("lists");
                            }
                            None => report.pend("lists", name, "the list is gone"),
                        }
                    }
                    Change::AddFavourite(item) => match uids.get(item) {
                        Some(uid) => {
                            lib.set_favorite(uid, true)?;
                            report.add("favourites");
                        }
                        None => report.unresolved += 1,
                    },
                    Change::SetRating(r) => match uids.get(&r.item) {
                        Some(uid) => {
                            lib.set_rating(uid, r.rating)?;
                            report.add("ratings");
                        }
                        None => report.unresolved += 1,
                    },
                    Change::AddTags { item, tags } => match uids.get(item) {
                        Some(uid) => {
                            let tags: Vec<&str> = tags.iter().map(String::as_str).collect();
                            lib.retag(&[uid], &tags, &[])?;
                            report.add("tags");
                        }
                        None => report.unresolved += 1,
                    },
                    _ => {}
                }
            }
            Ok::<_, Error>(())
        })
    }
}
