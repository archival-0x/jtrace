/// braindump-bin
///     This is the primary debugger tool used for instrumentation and introspection.

extern crate clap;
extern crate libc;
extern crate goblin;

mod ptrace; // spawning process and injecting breakpoints
mod elf;    // parsing and storing information about binary

use std::env;
use std::path::Path;

fn main() {

}
