# braindump

A minimal dynamic analysis tool in Rust.

## Introduction

__braindump__ is a minimal dynamic analysis tool written in Rust in order to debug Linux ELF binaries during runtime.
It provides and simplifies support for various features, as mentioned below.

## Features

* __Binary Instrumentation__: Breakpoints with callback support
* __Symbol Table Loading__: Function address resolving with symbol tables
* __Trace record-replay__: Record and then replay a syscall trace of target binary
