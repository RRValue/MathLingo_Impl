use crate::sentence::Sentence;

use proc_macro2::TokenStream;

pub fn evaluate_math_lingo(input: TokenStream) -> TokenStream{
    match syn::parse2::<Sentence>(input) {
        Ok(data) => return data.quote(),
        Err(err) => return TokenStream::from(err.to_compile_error())
    };
}