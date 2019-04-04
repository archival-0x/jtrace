# jtrace 

process trace to json utility written in Rust

## intro

One of the things that are missing from common process tracing utilities (ie `strace`) is the ability to emit output in a serializable data format.

## features

* safe and performant calls to `ptrace(2)`

## build

```
$ cargo install
```
