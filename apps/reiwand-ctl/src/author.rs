pub(crate) use crate::author_words::{DIR, POINTER};
use clap::Subcommand;
use std::path::PathBuf;

fn cells(text: &str) -> Result<[u32; 2], String> {
    let parsed =
        text.split_once(',').and_then(|(a, b)| Some([a.trim().parse().ok()?, b.trim().parse().ok()?]));
    parsed.ok_or_else(|| crate::told::fill(crate::told::told("%1 is not ROWS,COLUMNS"), &[&text]))
}

#[derive(Subcommand, Debug)]
pub enum Author {
    #[command(about = crate::steps::NEW)]
    New {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(long, default_value = "empty", help = "Starting point")]
        from: String,
        #[arg(long, help = "Package id; org.example.<folder name> when omitted")]
        id: Option<String>,
        #[arg(long, help = "Title; the folder name when omitted")]
        title: Option<String>,
        #[arg(long, help = "Language of the title, such as es or pt-BR; the desktop language when omitted")]
        locale: Option<String>,
        #[command(flatten)]
        preset: crate::presetting::Starting,
    },
    #[command(about = crate::steps::OPEN)]
    Open {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(long, value_name = "FILE", help = "Scene package to extract into the new folder")]
        from: Option<PathBuf>,
        #[command(flatten)]
        copying: crate::copying::Copying,
        #[arg(long, help = "Print the description as JSON")]
        json: bool,
    },
    #[command(about = "Print the document, or the value at a JSON Pointer, as JSON")]
    Show {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(help = POINTER)]
        pointer: Option<String>,
    },
    #[command(about = "Print the fields the inspector shows for a JSON Pointer, as JSON")]
    Fields {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(help = POINTER)]
        pointer: String,
    },
    #[command(about = "Copy a picture, video, sound, font or .cube table into the folder as an asset")]
    Import {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(help = "The file to copy")]
        file: PathBuf,
        #[arg(long, help = "Asset id; made from the file name when omitted")]
        id: Option<String>,
        #[arg(long, help = "Read the picture as data, such as a normal map or a mask, not as colors")]
        linear: bool,
        #[arg(long, value_name = "ROWS,COLUMNS", value_parser = cells, help = "Split the picture into a sprite sheet of equal cells")]
        atlas: Option<[u32; 2]>,
    },
    #[command(
        about = "Copy a picture into assets/ as the thumbnail of the package, made smaller when it is over 1024 pixels or 1 MiB, or with --animated as its animated preview; with --capture draw the scene instead of giving a picture"
    )]
    Thumbnail {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(long, help = "A gif or webp of at most 640 pixels a side, 15 s and 8 MiB")]
        animated: bool,
        #[command(flatten)]
        shot: crate::capturing::Capture,
    },
    #[command(flatten)]
    Texturing(crate::texturing::Texturing),
    #[command(flatten)]
    Folder(crate::folder::Folder),
    #[command(flatten)]
    Acting(crate::acting::Acting),
    #[command(flatten)]
    Rendering(crate::rendering::Rendering),
}
