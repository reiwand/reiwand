use crate::Error;
use std::sync::{Arc, Mutex, PoisonError};
use swc_common::errors::{DiagnosticBuilder, Emitter, Handler};
use swc_common::sync::Lrc;
use swc_common::{BytePos, GLOBALS, Globals, SourceMap};
use swc_ecma_parser::TsSyntax;
use swc_ts_fast_strip::{Mode, Options, operate};

type Found = Arc<Mutex<Vec<(String, Option<BytePos>)>>>;

struct Collect(Found);

impl Emitter for Collect {
    fn emit(&mut self, db: &mut DiagnosticBuilder<'_>) {
        let at = db.span.primary_span().map(|span| span.lo);
        self.0.lock().unwrap_or_else(PoisonError::into_inner).push((db.message(), at));
    }
}

pub fn strip(file: &str, source: &str) -> Result<String, Error> {
    let found = Found::default();
    let handler = Handler::with_emitter(false, false, Box::new(Collect(found.clone())));
    let map: Lrc<SourceMap> = Lrc::default();
    let parser = TsSyntax { tsx: false, decorators: false, ..TsSyntax::default() };
    let options = Options {
        module: Some(true),
        filename: Some(file.into()),
        parser,
        mode: Mode::StripOnly,
        ..Options::default()
    };
    let done = GLOBALS.set(&Globals::new(), || operate(&map, &handler, source.to_owned(), options));
    let first = found.lock().unwrap_or_else(PoisonError::into_inner).first().cloned();
    match (done, first) {
        (Ok(out), None) => Ok(out.code),
        (done, first) => {
            let (message, at) =
                first.unwrap_or_else(|| (done.err().map_or_else(String::new, |e| e.message), None));
            let (line, column) = at.map_or((0, 0), |at| {
                let loc = map.lookup_char_pos(at);
                (loc.line, loc.col_display + 1)
            });
            Err(Error::Strip { file: file.into(), line, column, message: plain(&message) })
        }
    }
}

fn plain(message: &str) -> String {
    message
        .replace("in strip-only mode", "in a plugin: Reiwand only removes types")
        .replace("type strip mode", "a plugin")
}
