extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::Ident;

struct Input {
    krate: Ident,
    sparse: bool,
    input: String,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let krate = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let sparse = input.parse::<syn::LitBool>()?.value;
        input.parse::<syn::Token![,]>()?;
        let input = input.parse::<syn::LitStr>()?.value();

        Ok(Self { krate, sparse, input })
    }
}

#[proc_macro]
pub fn dfa(input: TokenStream) -> TokenStream {
    impl_dfa(syn::parse_macro_input!(input as Input))
}

#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    impl_regex(syn::parse_macro_input!(input as Input))
}

fn compile_err(e: impl std::fmt::Debug) -> TokenStream {
    let e = format!("{:?}", e);
    TokenStream::from(quote::quote!(compile_error!(#e)))
}

use regex_automata::dfa::dense::{BuildError, Builder as DenseBuilder, Config as DenseConfig, DFA};
use regex_automata::dfa::regex::Regex;

fn dfa_apply_sparse(sparse: bool, dfa: &DFA<Vec<u32>>) -> Result<(Vec<u8>, usize), BuildError> {
    Ok(match sparse {
        true => (dfa.to_sparse()?.to_bytes_native_endian(), 0),
        false => dfa.to_bytes_native_endian(),
    })
}

fn ty_import(sparse: bool) -> (TokenStream2, TokenStream2) {
    match sparse {
        true => (quote::quote!(u8), quote::quote!(sparse)),
        false => (quote::quote!(u32), quote::quote!(dense)),
    }
}

fn impl_dfa(Input { krate, sparse, input }: Input) -> TokenStream {
    let (bytes, pad) = match DenseBuilder::new()
        .configure(DenseConfig::new().minimize(true))
        .build(&input)
        .and_then(|dfa| dfa_apply_sparse(sparse, &dfa))
    {
        Ok(dfa) => dfa,
        Err(e) => return compile_err(e),
    };

    let bytes = &bytes[pad..];

    let (ty, imports) = ty_import(sparse);

    TokenStream::from(quote::quote! {{
        use #krate::regex_automata::{
            dfa::#imports::DFA,
            util::wire::AlignAs,
        };

        static ALIGNED: &'static AlignAs<[u8], #ty> = &AlignAs {
            _align: [],
            bytes: [#(#bytes),*],
        };

        let (dfa, _) = DFA::from_bytes(&ALIGNED.bytes).expect("invalid DFA bytes");

        dfa
    }})
}

fn impl_regex(Input { krate, sparse, input }: Input) -> TokenStream {
    let regex = match Regex::builder().dense(DenseConfig::new().minimize(true)).build(&input) {
        Ok(regex) => regex,
        Err(e) => return compile_err(e),
    };

    let ((forward_bytes, forward_pad), (reverse_bytes, reverse_pad)) = (
        match dfa_apply_sparse(sparse, regex.forward()) {
            Ok(dfa) => dfa,
            Err(e) => return compile_err(e),
        },
        match dfa_apply_sparse(sparse, regex.reverse()) {
            Ok(dfa) => dfa,
            Err(e) => return compile_err(e),
        },
    );

    let forward_bytes = &forward_bytes[forward_pad..];
    let reverse_bytes = &reverse_bytes[reverse_pad..];

    let (ty, imports) = ty_import(sparse);

    TokenStream::from(quote::quote! {{
        use #krate::regex_automata::{
            dfa::{#imports::DFA, regex::Regex},
            util::wire::AlignAs,
        };

        static FORWARD: &'static AlignAs<[u8], #ty> = &AlignAs {
            _align: [],
            bytes: [#(#forward_bytes),*],
        };

        static REVERSE: &'static AlignAs<[u8], #ty> = &AlignAs {
            _align: [],
            bytes: [#(#reverse_bytes),*],
        };

        let (forward, _) = DFA::from_bytes(&FORWARD.bytes).expect("invalid Forward DFA bytes");
        let (reverse, _) = DFA::from_bytes(&REVERSE.bytes).expect("invalid Reverse DFA bytes");

        Regex::builder().build_from_dfas(forward, reverse)
    }})
}
