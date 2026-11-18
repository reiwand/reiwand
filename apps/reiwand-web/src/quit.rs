use std::time::{Duration, Instant};

const LINGER: Duration = Duration::from_secs(1);

fn others() -> Vec<u32> {
    let me = std::process::id();
    match unsafe { libc::getpgrp() } as u32 == me {
        true => reiwand_web::members(me).into_iter().filter(|p| *p != me).collect(),
        false => Vec::new(),
    }
}

pub fn finish(code: i32) -> ! {
    let until = Instant::now() + LINGER;
    while !others().is_empty() && Instant::now() < until {
        std::thread::sleep(Duration::from_millis(5));
    }
    for pid in others() {
        unsafe { libc::kill(pid as i32, libc::SIGKILL) };
    }
    std::process::exit(code)
}
