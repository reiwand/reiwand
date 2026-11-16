use crate::decode::{Picture, Problem, decode, package, video};
use image::ImageReader;
use reiwand_library::{Kind, Meta};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};
use std::os::fd::FromRawFd;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub path: PathBuf,
    pub kind: Kind,
    pub thumbs: Vec<(u32, PathBuf)>,
    #[serde(default)]
    pub draw: u32,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Response {
    pub meta: Meta,
    pub written: Vec<u32>,
    pub failure: Option<String>,
    #[serde(default)]
    pub undrawn: Option<String>,
    pub missing: bool,
    #[serde(skip)]
    pub retry: bool,
}

pub fn run() -> anyhow::Result<()> {
    reiwand_scene::launch_checks_from_here();
    let replies = unsafe {
        libc::setpriority(libc::PRIO_PROCESS, 0, 19);
        libc::syscall(libc::SYS_ioprio_set, 1, 0, 3 << 13);
        let replies = libc::fcntl(1, libc::F_DUPFD_CLOEXEC, 3);
        anyhow::ensure!(replies >= 0, "cannot set up the reply channel");
        libc::dup2(2, 1);
        std::fs::File::from_raw_fd(replies)
    };
    crate::compositor::stops::restore();
    let mut out = std::io::BufWriter::new(replies);
    for line in std::io::stdin().lock().lines() {
        let request: Request = serde_json::from_str(&line?)?;
        serde_json::to_writer(&mut out, &handle(&request))?;
        out.write_all(b"\n")?;
        out.flush()?;
    }
    reiwand_scene::end_trials();
    reiwand_render::finish_builds(0);
    Ok(())
}

fn handle(request: &Request) -> Response {
    let edge = request.thumbs.iter().map(|(size, _)| *size).max().unwrap_or(256);
    let picture = match request.kind {
        Kind::Image => match reiwand_render::decode_reduced(&request.path, edge) {
            Some((image, (width, height))) => {
                let meta = Meta { width: Some(width), height: Some(height), ..Meta::default() };
                Ok(Picture { meta, image: Some(image), failure: None, undrawn: None })
            }
            None => ImageReader::open(&request.path).map_err(Problem::from).and_then(decode).map(Picture::of),
        },
        Kind::Video => video(&request.path),
        Kind::Package => {
            let root = request.thumbs.first().and_then(|(_, p)| p.parent()?.parent());
            package(&request.path, request.draw, root.map(|r| r.join(".drawing")).as_deref())
        }
    };
    match picture {
        Err(Problem::Missing(e)) => Response { missing: true, failure: Some(e), ..Response::default() },
        Err(Problem::Decode(e)) => {
            Response { meta: Meta { error: Some(e), ..Meta::default() }, ..Response::default() }
        }
        Ok(Picture { mut meta, image, failure, undrawn }) => {
            let mut written = Vec::new();
            if let Some(image) = image {
                meta.color =
                    Some(reiwand_render::palette(&image).colors.first().copied().unwrap_or_default());
                if let Err(e) = reiwand_render::thumbnails(image, &request.thumbs, &mut written) {
                    return Response {
                        meta,
                        written,
                        failure: Some(e.to_string()),
                        undrawn,
                        ..Response::default()
                    };
                }
            }
            Response { meta, written, failure, undrawn, ..Response::default() }
        }
    }
}

#[cfg(test)]
mod tests;
