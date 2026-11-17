use crate::cli::{JSON, Wait};
use crate::daemon::job;
use crate::show::{self, prefix, table, text, time};
use crate::told::{fill, told};
use anyhow::Result;
use clap::Subcommand;
use futures_util::TryFutureExt;
use reiwand_client::Client;
use reiwand_core::model::ReposReply;

#[derive(Subcommand, Debug)]
pub enum Remote {
    #[command(about = "List the repositories added to this computer")]
    Ls {
        #[arg(long, help = JSON)]
        json: bool,
    },
    #[command(about = "Add a repository by pasting its address (or a folder)")]
    Add {
        #[arg(help = "Address of the repository, or a folder")]
        url: String,
        #[arg(
            long,
            value_name = "FINGERPRINT",
            help = "Trust the certificate of a home server that has its own, by the fingerprint the first attempt printed"
        )]
        trust_certificate: Option<String>,
        #[command(flatten)]
        wait: Wait,
    },
    #[command(about = "Add the example wallpapers that come with the program, after removing them")]
    Examples {
        #[command(flatten)]
        wait: Wait,
    },
    #[command(
        about = "Check a repository, or every repository when omitted, for updates and revocations",
        long_about = "Check a repository, or every repository when omitted, for updates and revocations. \
            A broken repository, whose stored copy cannot be read, is fetched again from its address; \
            when that address is lost, add it again with its URL or remove it."
    )]
    Refresh {
        #[arg(help = "Repository id, or at least its first 8 hex digits")]
        repo: Option<String>,
        #[command(flatten)]
        wait: Wait,
    },
    #[command(about = "Remove a repository")]
    Rm {
        #[arg(help = "Repository id, or at least its first 8 hex digits")]
        repo: String,
        #[arg(long, help = "Also uninstall its packages")]
        uninstall: bool,
    },
    #[command(
        about = "Vote for a package on the linked server; the server keeps only the counts and your own vote"
    )]
    Vote {
        #[arg(help = "A package id of the server's repository, or REPO/ID; other repositories are refused")]
        id: String,
        #[arg(value_parser = ["up", "down", "none"], help = "Vote up, down or take your vote back with none")]
        vote: String,
    },
    #[command(
        about = "Print the vote counts of packages on the linked server, unless repos.vote_counts hides them"
    )]
    Votes {
        #[arg(required = true, help = "Package ids, or REPO/ID; ids of other repositories are never sent")]
        ids: Vec<String>,
    },
}

pub async fn run(client: &Client, command: Remote) -> Result<()> {
    let repos = &client.repos;
    match command {
        Remote::Ls { json } => show::reply(repos.list().await?, json, list)?,
        Remote::Add { url, trust_certificate, wait } => {
            let url = crate::address::repository(&url)?;
            match trust_certificate {
                Some(print) => {
                    job(client, repos.add_trusting(&url, &print).map_ok(|id| ((), id)), &wait).await?
                }
                None => job(client, repos.add(&url).map_ok(|id| ((), id)), &wait).await?,
            }
        }
        Remote::Examples { wait } => job(client, repos.add_examples().map_ok(|id| ((), id)), &wait).await?,
        Remote::Refresh { repo, wait } => {
            job(client, repos.refresh(repo.as_deref().unwrap_or("")).map_ok(|id| ((), id)), &wait).await?
        }
        Remote::Rm { repo, uninstall } => repos.remove(&repo, uninstall).await?,
        Remote::Vote { id, vote } => {
            println!("{}", crate::providers::patient(client.publish.vote(&id, &vote)).await?)
        }
        Remote::Votes { ids } => {
            let ids: Vec<&str> = ids.iter().map(String::as_str).collect();
            println!("{}", crate::providers::patient(client.publish.get_votes(&ids)).await?)
        }
    }
    Ok(())
}

pub fn list(reply: ReposReply) -> String {
    let rows = reply.repos.into_iter().map(|r| {
        let broken = r.broken.map(|why| fill(told("broken, refresh or remove it: %1"), &[&why]));
        let status = broken.into_iter().chain(r.warnings.iter().map(text)).chain(r.last_error);
        let status = status.collect::<Vec<_>>().join(", ");
        let (packages, checked, expires) = (r.packages.to_string(), time(r.checked), time(r.entry_expires));
        vec![prefix(&r.repo_id), r.name, packages, checked, expires, status]
    });
    table(&["REPO", "NAME", "PACKAGES", "CHECKED", "EXPIRES", "STATUS"], rows)
}
