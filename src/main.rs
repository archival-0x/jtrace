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
extern crate clap;
extern crate log;

mod ptrace;

use std::env;
use std::process::Command;
use std::os::unistd::Pid;
use std::os::unix::process::CommandExt;

use clap::{App, Arg, ArgMatches};

use ptrace::helpers;


fn main() {
    let matches = App::new("jtrace")
        .about("process tracer that outputs deserialized JSON")
        .author("Alan Cao")
        .arg(
            Arg::with_name("command")
                .raw(true)
                .help("command to analyze as child (including arguments and flags)")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("out_json")
                .short("o")
                .long("output")
                .help("name of json file to save output in")
                .takes_value(true)
                .required(false)
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .long("verbosity")
                .help("set verbosity for program logging output")
                .takes_value(false)
                .required(false)
        )
        .get_matches();


    // initialize logger
    let level_filter = match matches.occurrences_of("verbose") {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace
    };

    let mut args = matches.values_of("command")
                          .ok_or("unable to get child command")
                          .collect::<Vec<&str>>();

    // initialize a command with process builder
    let mut cmd = Command::new(args.next().expect(matches.print_help()));
    for arg in args {
        cmd.arg(arg);
    }

    // perform an initial TRACEME to determine current process
    cmd.before_exec(|| {
        helpers::traceme()
    });

    // initialize handle to child process
    let child_handle = cmd.spawn().expect("failed spawning child process");
    let pid = Pid::from_raw(child_handle.id() as libc::pid_t);
}
