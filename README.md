regex-automata-macro
====================

This crate provides a set of macros for precompiling regular expressions into
DFAs or full regex engines, so that the runtime does not need to do any
compilation. This can be useful for reducing the startup time of programs that
use regular expressions, or for ensuring that the codegen is small and only
contains code relevant to searching the regular expression, such as when
compiling regular expressions for embedded or WASM targets.

The compiled regular expressions are also minimized to take up less space,
although this could result in increased compilation times. It's worth noting
that despite this the resulting DFAs could be rather large, even with sparse
DFAs, so it's recommended to only use this crate for regular expressions that
are small or simple, that make excluding the regex compilation machinery is worth it.

This crate is marked as `#![no_std]`, as the precompiled
DFAs can be used without `alloc` or `std`.

# Crate Features

* `unicode` (default) - Enables unicode support for `regex-automata`.
* `perf` - Enables extra performance optimizations for `regex-automata`.