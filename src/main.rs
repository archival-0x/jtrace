/// main.rs
///
///     CLI entry point for ptrace wrapper and helper
///     functions for performing process tracing
///     and se/deserialization

#[cfg(all(target_os = "linux",
          any(target_arch = "x86",
              target_arch = "x86_64")),
)]

extern crate libc;
extern crate nix;

mod ptrace;

use std::env;
use std::process::Command;
use std::os::unix::process::CommandExt;

use ptrace::helpers;


fn main() {

    // collect arguments (expect for this prog name)
    let mut args = env::args_os().skip(1);

    // initialize a command with process builder
    let mut cmd = Command::new(args.next()
                           .expect("usage: jtrace cmd [args...]"));
    for arg in args {
        cmd.arg(arg);
    }

    // perform an initial TRACEME to determine current process
    cmd.before_exec(|| {
        helpers::traceme();
    });

    // initialize handle to child process
    let child_handle = cmd.spawn().expect("failed spawning child process");
}
