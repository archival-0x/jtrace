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
extern crate clap;
extern crate nix;

#[macro_use] extern crate log;

use std::process::Command;
use std::os::unix::process::CommandExt;
use std::ffi::CString;

use nix::unistd;
use nix::sys::{signal, wait};

use clap::{App, Arg};
use log::LevelFilter;

mod logger; use logger::JtraceLogger;
mod ptrace; use ptrace::helpers;


static LOGGER: JtraceLogger = JtraceLogger;

#[allow(unused_must_use)]
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
                .multiple(true)
                .takes_value(false)
                .required(false)
        ).get_matches();


    // initialize logger
    let level_filter = match matches.occurrences_of("verbosity") {
        3       => LevelFilter::Warn,
        2       => LevelFilter::Debug,
        1       => LevelFilter::Info,
        0 | _   => LevelFilter::Off,
    };
    log::set_logger(&LOGGER).expect("unable to initialize logger");
    log::set_max_level(level_filter);
    info!("Initialized logger");

    // collect args into vec
    let args = matches.values_of("command")
                      .unwrap()
                      .collect::<Vec<&str>>();
    debug!("Command and args: {:?}", args);

    // initialize command
    let mut cmd = Command::new(args[0]);
    for arg in args.iter().skip(1) {
        cmd.arg(arg);
    }

    // fork child process
    info!("Forking child process from parent");
    let result = unistd::fork().expect("unable to call fork(2)");
    match result {
        unistd::ForkResult::Parent { child } => {
            info!("Tracing parent process");

            // in parent, wait for state change in child
            info!("Waiting for child process to send SIGSTOP");

            // TODO
        },
        unistd::ForkResult::Child => {
            info!("Tracing child process");

            // start tracing process, notifying parent through wait(2)
            info!("Child process executing PTRACE_TRACEME");
            helpers::traceme();

            // send a SIGSTOP in order to stop child process for parent introspection
            info!("Sending SIGSTOP, going back to parent process");
            signal::kill(unistd::getpid(), signal::Signal::SIGSTOP);

            // execute child process with tracing until termination
            info!("Executing rest of child execution until termination");
            let c_cmd = CString::new(args[0]).expect("failed to initialize CString command");
            let c_args: Vec<CString> = args.iter()
                .skip(1)
                .map(|&arg| CString::new(arg).expect("CString::new() failed"))
                .collect();
            unistd::execvp(&c_cmd, &c_args).ok().expect("failed to call execve(2) in child process");
        }
    }
}
