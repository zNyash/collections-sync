// src/ui.rs
use indicatif::MultiProgress;

#[allow(dead_code)]
pub fn info(mp: &MultiProgress, msg: &str) {
    mp.println(format!("\x1b[32m[Info]\x1b[0m {}", msg))
        .unwrap();
}

#[allow(dead_code)]
pub fn warn(mp: &MultiProgress, msg: &str) {
    mp.println(format!("\x1b[33m[Warn]\x1b[0m {}", msg))
        .unwrap();
}

#[allow(dead_code)]
pub fn error(mp: &MultiProgress, msg: &str) {
    mp.println(format!("\x1b[31m[Error]\x1b[0m {}", msg))
        .unwrap();
}
