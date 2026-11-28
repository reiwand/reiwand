use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Limits {
    pub memory: usize,
    pub stack: usize,
    pub cpu: Duration,
    pub wall: Duration,
    pub fetches: u32,
    pub response: usize,
    pub items: usize,
}

impl Limits {
    pub const STANDARD: Limits = Limits {
        memory: 32 << 20,
        stack: 512 << 10,
        cpu: Duration::from_secs(2),
        wall: Duration::from_secs(60),
        fetches: 32,
        response: 4 << 20,
        items: 500,
    };
}

pub const FILES: usize = 64;
pub const FILE_BYTES: u64 = 256 << 10;
pub const TOTAL_BYTES: u64 = 1 << 20;
