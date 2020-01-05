# jtrace

system call tracer implementation in Rust

## intro

__jtrace__ is a syscall tracer implemented natively in Rust. At its core, it is simply a `strace` clone that can be used for learning about the intrinsics of
tracing on Linux.

## build

Just like other process tracers, `jtrace` works out of the box without any auxiliary tools.

```
$ cargo install
$ jtrace -h
```

## usage

```
# basic usage
$ jtrace -- ls .

# emit a JSON trace, and print debug information
$ jtrace -vv --json -- ls .
```

## license

[mit](https://codemuch.tech/license.txt)
