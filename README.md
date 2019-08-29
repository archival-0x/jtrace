# jtrace

system call tracer with capabilities

## intro

__jtrace__ is a syscall tracer that has elevated capabilities implemented natively in Rust.
At its core, it is simply a `strace` clone that can be used for learning about instrinics,
but it also supports a variety of features not found in modern process tracers.

## features

* Supports eBPF (TODO) and `ptrace` modes of operation
* Supports emitting output in a serializable data format (JSON)
* (TODO) Library calls

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

# with json output and verbosity
$ jtrace -vv --out_json -- ls .
```

More (compelling) features coming soon!

## license

[mit](https://codemuch.tech/license.txt)
