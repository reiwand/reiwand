mod apply;
mod choices;
mod export;
mod gather;
mod libapply;
mod local;
mod preview;
mod report;
mod stage;

pub use export::Options;

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
