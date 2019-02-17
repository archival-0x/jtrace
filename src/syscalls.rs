/// syscalls.rs
///
///     De/serializable system call representations for
///     output reasoning.
///
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Syscall)]
pub enum Syscall {
    Read,
    Write,
    Open,
    Close,
    Stat,
    Fstat,
    Lstat,
    Other
}
