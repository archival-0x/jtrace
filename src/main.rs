//! main.rs
//!
//!     CLI entry point for ptrace wrapper and helper
//!     functions for performing process tracing
//!     and se/deserialization

#[cfg(all(target_os = "linux",
          any(target_arch = "x86",
              target_arch = "x86_64")),
)]
extern crate libc;
extern crate clap;
extern crate nix;
extern crate regex;
extern crate serde;

#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

use std::io;
use std::process::Command;
use std::ffi::CString;

use libc::{pid_t, c_int};

use nix::unistd;
use nix::sys::signal;

use clap::{App, Arg};
use log::LevelFilter;

mod logger;
use logger::JtraceLogger;

mod ptrace;
use ptrace::consts::{options, regs};
use ptrace::helpers;

mod syscall;
use syscall::SyscallManager;

static LOGGER: JtraceLogger = JtraceLogger;

/// `Parent` provides an interface for initializing
/// and interacting with a specified PID. It implements
/// internal controls and establishes helpers for syscalls
/// that are needed for tracer/tracee interactions.
struct Parent { pid: pid_t }


impl Parent {
    fn new(pid: pid_t) -> Self {
        Self { pid }
    }


    /// `run()` instantiates the loop that loops
    /// through program execution, waiting and stepping
    /// through each syscall
    fn run(&mut self) -> io::Result<()> {
        loop {
            match self.step() {
                Err(e) => panic!("Unable to run tracer. Reason: {:?}", e),
                Ok(Some(status)) => {
                    if status == 0 {
                        break;
                    }
                    // TODO
                },
                other => { other?; }
            }
        }
        Ok(())
    }

    /// `step()` defines the main instrospection
    /// performed ontop of the traced process, using
    /// ptrace to parse syscall registers for output
    fn step(&mut self) -> io::Result<Option<c_int>> {
        helpers::syscall(self.pid)?;
        if let Some(status) = self.wait().unwrap() {
            return Ok(Some(status));
        }

        // determine syscall number and initialize
        let syscall_num = match self.get_syscall_num() {
            Ok(num) => num,
            Err(e) => panic!("Cannot retrieve syscall number. Reason {:?}", e),
        };

        // TODO: proper parsing of args
        let args = vec![self.get_arg(0).unwrap(),
                        self.get_arg(1).unwrap(),
                        self.get_arg(2).unwrap()];

        // initialize new syscall manager
        let mut manager = SyscallManager::new();
        manager.add_syscall(syscall_num, args);

        helpers::syscall(self.pid)?;
        if let Some(status) = self.wait().unwrap() {
            return Ok(Some(status));
        }

        Ok(None)
    }


    /// `wait()` wrapper to waitpid/wait4, with error-checking in order
    /// to return proper type back to developer.
    fn wait(&self) -> io::Result<Option<c_int>> {
        let mut status = 0;
        unsafe {
            libc::waitpid(self.pid, &mut status, 0);

            // error-check status set
            if libc::WIFEXITED(status) {
                Ok(Some(libc::WEXITSTATUS(status)))
            } else {
                Ok(None)
            }
        }
    }


    /// `get_arg()` is called to introspect current process
    /// states register values in order to determine syscall
    /// and arguments passed.
    fn get_arg(&mut self, reg: u8) -> io::Result<u64> {

        // TODO: use different regs for different archs
        let offset = match reg {
            0 => regs::RDI,
            1 => regs::RSI,
            2 => regs::RDX,
            3 => regs::R10,
            4 => regs::R8,
            5 => regs::R9,
            _ => panic!("Unmatched argument offset")
        };
        helpers::peek_user(self.pid, offset).map(|x| x as u64)
    }


    /// `get_syscall_num()` uses ptrace with PEEK_USER to return the
    /// syscall num from ORIG_RAX.
    fn get_syscall_num(&mut self) -> io::Result<u64> {
        helpers::peek_user(self.pid, regs::ORIG_RAX).map(|x| x as u64)
    }


}


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
    if args.len() > 1 {
        for arg in args.iter().skip(1) {
            debug!("Adding arg: {}", arg);
            cmd.arg(arg);
        }
    }

    // fork child process
    info!("Forking child process from parent");
    let result = unistd::fork().expect("unable to call fork(2)");
    match result {
        unistd::ForkResult::Parent { child } => {

            // initialize wrapper for interactions
            let mut pid = Parent::new(child.as_raw());

            info!("Tracing parent process");

            // in parent, wait for process event from child
            info!("Waiting for child process to send SIGSTOP");
            if let Err(e) = pid.wait() {
                panic!("Error: {:?}", e);
            }

            // set trace options
            info!("Setting trace options with PTRACE_SETOPTIONS");
            helpers::set_options(child.as_raw(), options::PTRACE_O_TRACESYSGOOD.into());

            // execute loop that examines through syscalls
            info!("Executing parent with tracing");
            pid.run();
        },
        unistd::ForkResult::Child => {
            info!("Tracing child process");

            // start tracing process, notifying parent through wait(2)
            info!("Child process executing PTRACE_TRACEME");
            helpers::traceme();

            // send a SIGSTOP in order to stop child process for parent introspection
            info!("Sending SIGTRAP, going back to parent process");
            signal::kill(unistd::getpid(), signal::Signal::SIGSTOP);

            // execute child process with tracing until termination
            info!("Executing rest of child execution until termination");
            let c_cmd = CString::new(args[0]).expect("failed to initialize CString command");
            let c_args: Vec<CString> = args.iter()
                .map(|&arg| CString::new(arg).expect("CString::new() failed"))
                .collect();
            unistd::execvp(&c_cmd, &c_args).ok().expect("failed to call execvp(2) in child process");
        }
    }
}
