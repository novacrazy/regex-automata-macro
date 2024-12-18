#![doc = include_str!("../README.md")]

pub extern crate regex_automata;

/// Create a [dense DFA](regex_automata::dfa::dense::DFA) from a regex pattern
/// and return a reference to it to be used inline.
///
/// The pattern is compiled at Rust compile time, so this has no runtime overhead.
#[macro_export]
macro_rules! dfa {
    ($input:literal) => {{
        $crate::decl_dfa!(_DFA = $input);
        &*_DFA // NOTE: DFA has an `as_ref` method, but I chose this for consistency with Regex.
    }};
}

/// Create a [sparse DFA](regex_automata::dfa::sparse::DFA) from a regex pattern
/// and return a reference to it to be used inline.
///
/// The pattern is compiled at Rust compile time, so this has no runtime overhead.
#[macro_export]
macro_rules! dfa_sparse {
    ($input:literal) => {{
        $crate::decl_dfa_sparse!(_DFA = $input);
        &*_DFA
    }};
}

/// Create a dense [DFA Regex](regex_automata::dfa::regex::Regex) from a regex pattern
/// and return a reference to it to be used inline.
///
/// The pattern is compiled at Rust compile time, so this has no runtime overhead.
#[macro_export]
macro_rules! regex {
    ($input:literal) => {{
        $crate::decl_regex!(_REGEX = $input);
        &*_REGEX
    }};
}

/// Create a sparse [DFA Regex](regex_automata::dfa::regex::Regex) from a regex pattern
/// and return a reference to it to be used inline.
///
/// The pattern is compiled at Rust compile time, so this has no runtime overhead.
#[macro_export]
macro_rules! regex_sparse {
    ($input:literal) => {{
        $crate::decl_regex_sparse!(_REGEX = $input);
        &*_REGEX
    }};
}

/// Declare a [dense DFA](regex_automata::dfa::dense::DFA) from a regex pattern.
///
/// This is useful when you want to declare a DFA but not use it immediately.
/// The pattern is compiled at Rust compile time, so this has no runtime overhead.
#[macro_export]
macro_rules! decl_dfa {
    ($vis:vis $name:ident = $input:literal) => {
        $vis static $name: $crate::regex_automata::util::lazy::Lazy<
            $crate::regex_automata::dfa::dense::DFA<&'static [u32]>
        > = $crate::regex_automata::util::lazy::Lazy::new(|| {
            regex_automata_proc_macro::dfa!($crate, false, $input)
        });
    };
}

/// Declare a [sparse DFA](regex_automata::dfa::sparse::DFA) from a regex pattern.
///
/// This is useful when you want to declare a DFA but not use it immediately.
/// The pattern is compiled at Rust compile time, so this has no runtime overhead.
#[macro_export]
macro_rules! decl_dfa_sparse {
    ($vis:vis $name:ident = $input:literal) => {
        $vis static $name: $crate::regex_automata::util::lazy::Lazy<
            $crate::regex_automata::dfa::sparse::DFA<&'static [u8]>
        > = $crate::regex_automata::util::lazy::Lazy::new(|| {
            regex_automata_proc_macro::dfa!($crate, true, $input)
        });
    };
}

/// Declare a dense [DFA Regex](regex_automata::dfa::regex::Regex) from a regex pattern.
///
/// This is useful when you want to declare a Regex but not use it immediately.
/// The pattern is compiled at Rust compile time, so this has no runtime overhead.
#[macro_export]
macro_rules! decl_regex {
    ($vis:vis $name:ident = $input:literal) => {
        $vis static $name: $crate::regex_automata::util::lazy::Lazy<
            $crate::regex_automata::dfa::regex::Regex<
                $crate::regex_automata::dfa::dense::DFA<&'static [u32]>
            >
        > = $crate::regex_automata::util::lazy::Lazy::new(|| {
            regex_automata_proc_macro::regex!($crate, false, $input)
        });
    }
}

/// Declare a sparse [DFA Regex](regex_automata::dfa::regex::Regex) from a regex pattern.
///
/// This is useful when you want to declare a Regex but not use it immediately.
/// The pattern is compiled at Rust compile time, so this has no runtime overhead.
#[macro_export]
macro_rules! decl_regex_sparse {
    ($vis:vis $name:ident = $input:literal) => {
        $vis static $name: $crate::regex_automata::util::lazy::Lazy<
            $crate::regex_automata::dfa::regex::Regex<
                $crate::regex_automata::dfa::sparse::DFA<&'static [u8]>
            >
        > = $crate::regex_automata::util::lazy::Lazy::new(|| {
            regex_automata_proc_macro::regex!($crate, true, $input)
        });
    }
}
