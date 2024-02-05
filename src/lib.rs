use proc_macro2::TokenStream;

mod kw {
    syn::custom_keyword!(one);
    syn::custom_keyword!(two);
    syn::custom_keyword!(plus);
}

pub fn evaluate_math_lingo(input: TokenStream) -> TokenStream{
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use assert_tokenstreams_eq::assert_tokenstreams_eq;

    #[test]
    fn simpple_addition() {
        let input = quote!(one plus two);
        let output = evaluate_math_lingo(input);
        let expected_output = quote!(3);

        assert_tokenstreams_eq!(&output, &expected_output);
    }
}
