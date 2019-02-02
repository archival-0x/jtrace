/// ptrace.rs
///
///     Currently, `nix` support for `ptrace(2)` on the most recent
///     version of the crate (as of working, 0.12.0) is deprecated.
///     This is a re-implementation of `ptrace(2)` that allows safer 
///     usage through a specialized helper function.

mod ptrace {
    use libc::{c_int, c_long, c_void, pid_t};
    use nix::errno::Errno;

    /// while the parameters of `ptrace(2)` call for
    /// a `enum __ptrace_request`, we simplify it for FFI
    /// and instead define as an alias to a C integer.
    pub type PtraceRequest = c_int;

    /// these represent `PtraceRequest`s a tracer
    /// can send to tracee in order to perform actions
    /// on attached process.
    pub const PTRACE_TRACEME:     PtraceRequest = 0;
    pub const PTRACE_PEEKTEXT:    PtraceRequest = 1;
    pub const PTRACE_PEEKDATA:    PtraceRequest = 2;
    pub const PTRACE_PEEKUSER:    PtraceRequest = 3;
    pub const PTRACE_POKETEXT:    PtraceRequest = 4;
    pub const PTRACE_POKEDATA:    PtraceRequest = 5;
    pub const PTRACE_POKEUSER:    PtraceRequest = 6; 
    pub const PTRACE_CONT:        PtraceRequest = 7;
    pub const PTRACE_KILL:        PtraceRequest = 8;
    pub const PTRACE_SINGLESTEP:  PtraceRequest = 9;
    pub const PTRACE_GETREGS:     PtraceRequest = 12;
    pub const PTRACE_SETREGS:     PtraceRequest = 13;
    pub const PTRACE_GETFPREGS:   PtraceRequest = 14;
    pub const PTRACE_SETFPREGS:   PtraceRequest = 15;
    pub const PTRACE_ATTACH:      PtraceRequest = 16;
    pub const PTRACE_DETACH:      PtraceRequest = 17;
    pub const PTRACE_GETFPXREGS:  PtraceRequest = 18;
    pub const PTRACE_SETFPXREGS:  PtraceRequest = 19;
    pub const PTRACE_SYSCALL:     PtraceRequest = 24;
    pub const PTRACE_SETOPTIONS:  PtraceRequest = 0x4200;
    pub const PTRACE_GETEVENTMSG: PtraceRequest = 0x4201;
    pub const PTRACE_GETSIGINFO:  PtraceRequest = 0x4202;
    pub const PTRACE_SETSIGINFO:  PtraceRequest = 0x4203;
    pub const PTRACE_GETREGSET:   PtraceRequest = 0x4204;
    pub const PTRACE_SETREGSET:   PtraceRequest = 0x4205;
    pub const PTRACE_SEIZE:       PtraceRequest = 0x4206;
    pub const PTRACE_INTERRUPT:   PtraceRequest = 0x4207;
    pub const PTRACE_LISTEN:      PtraceRequest = 0x4208;
    pub const PTRACE_PEEKSIGINFO: PtraceRequest = 0x4209;


    /// defines an `unsafe` foreign function interface to the `ptrace(2)` system call.
    /// `ptrace(2)`'s original C function definition is as follows:
    /// 
    /// ```
    ///     long ptrace(enum __ptrace_request request, pid_t pid,
    ///                 void *addr, void *data);
    /// ```
    extern {
        fn ptrace(request: c_int, pid: pid_t, 
                  addr: * const c_void, data: * const c_void) -> c_long;
    }


    /// `exec_ptrace()` is the main and safest interface for calling the unsafe `ptrace` FFI.
    /// It does error-checking to ensure that the user receives errors through Result<T>, and
    pub fn exec_ptrace(request: PtraceRequest, pid: pid_t, addr: *mut c_void, data: *mut c_void) -> Result<i64, Errno> {

        // on PTRACE_PEEK* commands, a successful request might still return -1. As a result,
        // we need to clear errno and do some other error-checking.
        match request {
            PTRACE_PEEKTEXT | PTRACE_PEEKDATA | PTRACE_PEEKUSER => {
                // grab return value of ptrace call (notice no semicolon)
                let ret = unsafe {
                    Errno::clear();
                    ptrace(request, pid, addr, data)
                };

                // error-check and ensure that errno is actually not a false positive
                if ret == -1 && Errno::last() != Errno::UnknownErrno {
                    return Err(Errno::last());
                }

                return Ok(ret);
            },
            _ => {},
        }
                
        // for other conventional PTRACE_* commands
        match unsafe { ptrace(request, pid, addr, data) } {
            -1 => Err(Errno::last()),
            _ => Ok(0)
        }
    }
}

/// defines helper functions that interact with `exec_ptrace`
/// in order to perform process debugging.
pub mod helpers {
    use std::ptr;
    use std::ffi::CString;
    use libc::pid_t;
    use nix::errno::Errno; 
    use ptrace::ptrace::*;

    mod syscalls;
    use syscalls::wrappers;

    /// alias the pid_t for better clarification
    pub type InferiorType = pid_t;


    /// `traceme()` call with error-checking. PTRACE_TRACEME is used as a method
    /// used to check the process that the user is currently in, such as ensuring that
    /// a fork call actually spawned off a child process.
    pub fn traceme() -> () {
        if let Err(e) = exec_ptrace(PTRACE_TRACEME, 0, ptr::null_mut(), ptr::null_mut()) {
            panic!("Failed PTRACE_TRACEME: {:?}", e);
        }
    }


    /// `cont()` call with error-checking. PTRACE_CONTINUE is used to restart
    /// stopped tracee process.
    pub fn cont(pid: InferiorType) -> () {
        if let Err(e) = exec_ptrace(PTRCE_CONT, pid, ptr::null_mut(), ptr::null_mut()) {
            panic!("Failed PTRACE_CONTINUE: {:?}", e);
        }
    }


    /// TODO: description
    pub fn inferior_exec(filename: &str, args: &[&str]) -> InferiorType {
        let c_filename = &CString::new(filename).unwrap();
        traceme();
        wrappers::execve(c_filename, &[], &[]);
        unreachable!();
    }


    pub fn inferior_attach(pid: InferiorType) -> Result<InferiorType, Errno> {
        match waitpid(pid, None) {
            Ok(WaitStatus::Stopped(pid, signal::SIGTRAP)) => return Ok(pid),
            Ok(_)                                         => panic!("Unexpected stop in attach"),
            Err(e)                                        => return Err(e)
        }
    }

    /// `exec()` is the main helper method utilized during execution in order to
    /// fork off a child process and execute the specific program within that execution.
    ///
    /// This is equivalent to:
    ///
    /// ```
    /// pid_t pid = fork();
    /// if (pid == 0)
    ///     // child process
    ///     // execve
    /// else
    ///     // parent process
    ///     // debugger
    /// ```
    pub fn exec(file: &str, args: &[&str]) -> Result<InferiorType, nix::errno::Errno> {
        loop {
            match syscalls::fork() {
                Ok(Child)               => helpers::inferior_exec(file, args),
                Ok(Parent(pid))         => return helpers::inferior_attach(pid),
                Err(errno::EAGAIN)      => continue,
                Err(e)                  => return Err(e)
            };
        }
    }   

}
