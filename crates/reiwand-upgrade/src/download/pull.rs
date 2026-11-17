use anyhow::bail;
use reiwand_sync::{Doc, Remote, Session, View};
use std::collections::BTreeMap;

pub async fn all(session: &Session) -> anyhow::Result<BTreeMap<String, View>> {
    let (mut since, mut docs) = (0, BTreeMap::<String, Doc>::new());
    loop {
        let page = session.pull(since).await?;
        for change in page.changes {
            docs.insert(change.key, change.doc);
        }
        if !page.more {
            break;
        }
        if page.last <= since {
            bail!("the server sent the same page of changes again");
        }
        since = page.last;
    }
    Ok(docs.into_iter().filter_map(|(key, doc)| Some((key, doc.view()?))).collect())
}
