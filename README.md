# jtrace

process trace to json utility written in Rust

## intro

One of the things that are missing from common process tracing utilities (ie `strace`) is the ability to emit output in a serializable data format.

Besides being a security utility, __jtrace__ is also a PoC of a simple and native implementation of a process tracer.

## features

* safe and performant calls to `ptrace(2)`

## build

```
$ cargo install
```
