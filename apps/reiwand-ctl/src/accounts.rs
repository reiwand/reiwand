use crate::args::Wait;
use crate::cli::JSON;
use crate::daemon::job;
use crate::show::table;
use crate::told::{fill, told, tr};
use anyhow::Result;
use clap::Subcommand;
use reiwand_client::Client;
use reiwand_core::model::AccountsReply;

#[derive(Subcommand, Debug)]
pub enum Accounts {
    #[command(about = "List the accounts known on this computer and the repositories each one is used for")]
    Ls {
        #[arg(long, help = JSON)]
        json: bool,
    },
    #[command(
        name = "sign-in",
        about = "Sign in to the server of a repository: approve the code in the browser; the account is used for that repository"
    )]
    SignIn {
        #[arg(value_name = "URL", help = "Address of the repository whose server you sign in to")]
        url: String,
    },
    #[command(about = "Use an account for a repository")]
    Use {
        #[arg(value_name = "REPO", help = "Address of the repository")]
        repo: String,
        #[arg(value_name = "ACCOUNT", help = "Account id from reiwand-ctl account ls")]
        account: String,
    },
    #[command(about = "Stop using any account for a repository; the account stays")]
    Unlink {
        #[arg(value_name = "REPO", help = "Address of the repository")]
        repo: String,
    },
    #[command(name = "rm", about = "Forget an account on this computer, with its saved access")]
    Remove {
        #[arg(value_name = "ACCOUNT", help = "Account id from reiwand-ctl account ls")]
        account: String,
    },
}

pub async fn run(client: &Client, command: Accounts) -> Result<()> {
    let accounts = &client.accounts;
    match command {
        Accounts::Ls { json } => {
            let text = accounts.list().await?;
            if json {
                println!("{text}");
                return Ok(());
            }
            let reply: AccountsReply = serde_json::from_str(&text)?;
            if reply.accounts.is_empty() {
                println!("{}", tr(told("no accounts")));
            }
            let rows = reply.accounts.iter().map(|a| {
                let state = if a.pending { tr(told("sign in again")) } else { String::new() };
                let repos = a.repos.iter().map(|r| r.url.as_str()).collect::<Vec<_>>().join(", ");
                vec![a.id.clone(), a.user.clone(), a.server.clone(), state, repos]
            });
            print!("{}", table(&["ID", "USER", "SERVER", "STATE", "REPOSITORIES"], rows));
        }
        Accounts::SignIn { url } => {
            let (code, page, id) = accounts.sign_in(&url).await?;
            println!("{}", fill(told("Open %1 and enter the code %2"), &[&page, &code]));
            let wait = Wait { wait: true, json: false };
            job(client, async { Ok(((), id)) }, &wait).await?;
        }
        Accounts::Use { repo, account } => accounts.use_account(&repo, &account).await?,
        Accounts::Unlink { repo } => accounts.unlink(&repo).await?,
        Accounts::Remove { account } => accounts.remove(&account).await?,
    }
    Ok(())
}
