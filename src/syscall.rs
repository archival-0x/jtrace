//! syscall.rs
//!
//!     Defines struct interface for system calls.
//!
//!     Implements a parser for unistd.h's syscall table
//!     in order to generate system calls with correct names.

use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use regex::Regex;
use serde::{Deserialize, Serialize};

// path to unistd file with syscall number definitions
static SYSCALL_TABLE: &str = "/usr/include/asm/unistd_64.h";

// regex for parsing macro definitions of syscall numbers
static SYSCALL_REGEX: &str = r"^#define\s*__NR_(\w+)\s*(\d+)";

type SyscallTable = HashMap<u64, String>;

/// Defines an arbitrary syscall, with support for de/serialization
/// with serde_json.
#[derive(Serialize, Deserialize)]
pub struct Syscall {
    number: u64,
    name: String,
    args: Vec<String>,
}


/// SyscallManager stores a vector of Syscalls and manages a HashMap
/// that stores syscall num and name mappings.
#[derive(Serialize, Deserialize)]
pub struct SyscallManager {
    syscalls: Vec<Syscall>,
    _syscall_table: SyscallTable
}


impl Default for SyscallManager {
    fn default() -> Self {
        let syscall_table = SyscallManager::_parse_syscall_table()
            .expect("cannot parse syscall table.");

        Self {
            syscalls: Vec::new(),
            _syscall_table: syscall_table
        }
    }
}

impl SyscallManager {

    pub fn new() -> Self {
        Self::default()
    }

    /// `_parse_syscall_table()` is a helper method that parses a "syscall table"
    /// and instantiates a HashMap that stores the syscall num as a key and the name
    /// as the value.
    #[inline]
    fn _parse_syscall_table() -> io::Result<SyscallTable> {

        // read unistd.h for macro definitions
        let mut tbl_file = File::open(SYSCALL_TABLE)?;
        let mut contents = String::new();
        tbl_file.read_to_string(&mut contents)?;

        lazy_static! {
            static ref REF: Regex = Regex::new(SYSCALL_REGEX).expect("cannot initialize regex object");
        }

        let syscall_table = HashMap::new();
        Ok(syscall_table)
    }


    /// `add_syscall()` finds a corresponding syscall name from
    /// a parsed syscall table and instantiates and stores a new Syscall.
    pub fn add_syscall(&mut self, syscall_num: u64, args: Vec<String>) -> () {

        // retrieve syscall name from HashMap by syscall_num key
        let syscall_name = match self._syscall_table.get(&syscall_num) {
            Some(s) => s,
            None => {
                panic!("unable to determine corresponding syscall for number {}", syscall_num);
            }
        };

        // initialize Syscall definition and store
        let syscall = Syscall {
            number: syscall_num,
            name: syscall_name.to_string(),
            args: args
        };
        self.syscalls.push(syscall);
    }

}
