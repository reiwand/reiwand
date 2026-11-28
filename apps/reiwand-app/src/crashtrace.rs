use std::sync::atomic::{AtomicBool, Ordering};

const SIGNALS: [libc::c_int; 4] = [libc::SIGSEGV, libc::SIGBUS, libc::SIGABRT, libc::SIGILL];
const STACK: usize = 1 << 20;
const FRAMES: usize = 128;

unsafe extern "C" {
    fn backtrace(buffer: *mut *mut libc::c_void, size: libc::c_int) -> libc::c_int;
    fn backtrace_symbols_fd(buffer: *const *mut libc::c_void, size: libc::c_int, fd: libc::c_int);
}

static TRACING: AtomicBool = AtomicBool::new(false);

fn say(text: &str) {
    let mut rest = text.as_bytes();
    while !rest.is_empty() {
        let written = unsafe { libc::write(2, rest.as_ptr().cast(), rest.len()) };
        if written <= 0 {
            return;
        }
        rest = &rest[written as usize..];
    }
}

extern "C" fn caught(signal: libc::c_int) {
    if !TRACING.swap(true, Ordering::SeqCst) {
        let thread = std::thread::current();
        let name = thread.name().unwrap_or("unnamed");
        say(&format!("reiwand-app: caught signal {signal} in thread {name}; its frames:\n"));
        let mut frames = [std::ptr::null_mut(); FRAMES];
        unsafe {
            let count = backtrace(frames.as_mut_ptr(), FRAMES as libc::c_int);
            backtrace_symbols_fd(frames.as_ptr(), count, 2);
        }
        say("reiwand-app: the same stack, resolved:\n");
        say(&format!("{}\n", std::backtrace::Backtrace::force_capture()));
    }
    unsafe {
        libc::signal(signal, libc::SIG_DFL);
        libc::raise(signal);
    }
}

pub fn install() {
    unsafe {
        let stack = libc::mmap(
            std::ptr::null_mut(),
            STACK,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );
        if stack != libc::MAP_FAILED {
            let alternate = libc::stack_t { ss_sp: stack, ss_flags: 0, ss_size: STACK };
            libc::sigaltstack(&alternate, std::ptr::null_mut());
        }
        let mut action: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = caught as extern "C" fn(libc::c_int) as libc::sighandler_t;
        action.sa_flags = libc::SA_ONSTACK | libc::SA_RESETHAND | libc::SA_NODEFER;
        libc::sigemptyset(&mut action.sa_mask);
        for signal in SIGNALS {
            libc::sigaction(signal, &action, std::ptr::null_mut());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::process::{CommandExt, ExitStatusExt};
    use std::process::Command;

    const CHILD: &str = "REIWAND_APP_CRASH_CHILD";

    #[inline(never)]
    fn fall_over() {
        unsafe { libc::raise(libc::SIGSEGV) };
    }

    #[test]
    fn a_crash_prints_its_stack_and_still_dies_by_the_signal() {
        if std::env::var_os(CHILD).is_some() {
            super::install();
            fall_over();
            return;
        }
        let name = "crashtrace::tests::a_crash_prints_its_stack_and_still_dies_by_the_signal";
        let mut child = Command::new(std::env::current_exe().unwrap());
        unsafe {
            child.pre_exec(|| {
                let none = libc::rlimit { rlim_cur: 0, rlim_max: 0 };
                libc::setrlimit(libc::RLIMIT_CORE, &none);
                Ok(())
            });
        }
        let out = child
            .args(["--exact", name, "--nocapture", "--test-threads", "1"])
            .env(CHILD, "1")
            .output()
            .unwrap();
        let text = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.signal(), Some(libc::SIGSEGV), "{text}");
        assert!(text.contains("caught signal 11"), "{text}");
        assert!(text.contains("fall_over"), "{text}");
    }
}
