regex-automata-macro
====================

This crate provides a set of macros for precompiling regular expressions into
DFAs or full regex engines, so that the runtime does not need to do any
compilation. This can be useful for reducing the startup time of programs that
use regular expressions, or for ensuring that the codegen is small and only
contains code relevant to searching the regular expression, such as when
compiling regular expressions for embedded or WASM targets.