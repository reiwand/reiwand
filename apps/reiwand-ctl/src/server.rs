use crate::cli::{JSON, Wait};
use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct At {
    #[arg(
        long,
        value_name = "ADDRESS",
        help = "Repository whose account is used (address or id); required when several repositories have an account"
    )]
    pub repo: Option<String>,
}

impl At {
    pub fn text(&self) -> &str {
        self.repo.as_deref().unwrap_or("")
    }
}

#[derive(Subcommand, Debug)]
pub enum Server {
    #[command(about = "Upload a .wpak package to the repository of your account")]
    Upload {
        #[arg(help = "File to upload")]
        file: String,
        #[arg(
            long,
            help = "Keep it out of the catalog and search; whoever has its link can still install it, and \
                    the public repository files still list it, so it is not private"
        )]
        unlisted: bool,
        #[arg(
            long,
            value_name = "ADDRESS",
            help = "Repository to publish to; required when several repositories have an account"
        )]
        repo: Option<String>,
        #[command(flatten)]
        wait: Wait,
    },
    #[command(about = "List the packages this account uploaded, with their state and visibility")]
    Packages {
        #[arg(long, help = "Print the result as JSON")]
        json: bool,
        #[command(flatten)]
        at: At,
    },
    #[command(about = "List the upload sessions of this account, or cancel an unfinished one")]
    Uploads {
        #[arg(long, help = JSON)]
        json: bool,
        #[arg(long, value_name = "ID", help = "Cancel this unfinished upload")]
        cancel: Option<String>,
        #[command(flatten)]
        at: At,
    },
    #[command(subcommand, about = "The keys with which you sign your packages online")]
    AuthorKey(crate::keyed::AuthorKey),
    #[command(about = "Make a package, or one version, public or unlisted")]
    Visibility {
        #[arg(help = "Package id")]
        id: String,
        #[arg(
            value_parser = ["public", "unlisted"],
            help = "Public shows the package in browsing and search; unlisted only by its id or link"
        )]
        visibility: String,
        #[arg(long, help = "Version to change; every version when left out")]
        version: Option<String>,
        #[command(flatten)]
        at: At,
    },
    #[command(
        about = "Withdraw a package of yours from the repository for good, after asking",
        long_about = "Withdraw a package of yours from the repository for good. Every published version \
                      is revoked, so devices that installed it are told it was withdrawn by its author, \
                      and its files are deleted from the server; this cannot be undone. The id stays \
                      yours: you may upload a newer version later."
    )]
    Withdraw {
        #[arg(help = "Package id")]
        id: String,
        #[arg(long, help = "Withdraw without asking, for scripts")]
        yes: bool,
        #[command(flatten)]
        at: At,
    },
}
