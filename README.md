# jtrace

system call tracer with JSON output capabilities in Rust

## intro

One of the things that are missing from common process tracing utilities (ie `strace`) is the ability to emit output in a serializable data format.

Besides being a security utility, __jtrace__ is also a PoC of a simple and native implementation of a process tracer.

## build

```
$ cargo install
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
