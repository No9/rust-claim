#[cfg(not(target_os = "wasi"))]
extern crate autocfg;
fn main() {
    let cfg = autocfg::new();
    cfg.emit_path_cfg("core::task::Poll", "has_task_poll");
    cfg.emit_path_cfg("std::task::Poll", "has_task_poll");

    // Needed to enable `#![no_std]` only on rustc versions that support it (rustc 1.6.0 and up).
    cfg.emit_rustc_version(1, 6);

    // Needed for `assert_matches!`' minimum rust version.
    cfg.emit_rustc_version(1, 26);

    if cfg.probe_rustc_version(1, 15) && !cfg.probe_rustc_version(1, 16) {
        autocfg::emit("has_private_in_public_issue");
    }
}
