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

#[macro_use] extern crate log;

use std::process::Command;
use std::os::unix::process::CommandExt;

use clap::{App, Arg};
use log::LevelFilter;

mod logger; use logger::JtraceLogger;
mod ptrace; use ptrace::helpers;


static LOGGER: JtraceLogger = JtraceLogger;

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
        ).get_matches();


    // initialize logger
    let level_filter = match matches.occurrences_of("verbose") {
        2       => LevelFilter::Debug,
        1       => LevelFilter::Warn,
        0 | _   => LevelFilter::Error,
    };
    log::set_logger(&LOGGER).expect("unable to initialize logger");
    log::set_max_level(level_filter);

    // collect args into vec
    let mut args = matches.values_of("command")
                          .unwrap() 
                          .collect::<Vec<&str>>();
    debug!("cmd and args: {:?}", args);
}
