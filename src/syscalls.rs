/// syscalls.rs
///     
///     wrappers:
///         Defines wrappers to commonly used system calls
///         during process tracing and other interactions.
///
///     syscalls:
///         De/serializable system call representations for
///         output reasoning.
///

pub mod wrappers {

    pub fn fork();

    pub fn clone();

    pub fn execve();

    pub fn waitpid();
}

pub mod syscalls {

    #[derive(Serialize, Deserialize, Syscall)]
    pub enum Syscall {
        /// TODO

    }

}
