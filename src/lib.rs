use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::Result;

mod kw {
    syn::custom_keyword!(one);
    syn::custom_keyword!(two);
    syn::custom_keyword!(plus);
}

struct BasicSentence {
    pub one: kw::one,
    pub plus: kw::plus,
    pub two: kw::two,
}

impl Parse for BasicSentence {
    fn parse(input: ParseStream) -> Result<Self> {
        let one: kw::one = input.parse()?;
        let plus: kw::plus = input.parse()?;
        let two: kw::two = input.parse()?;
        Ok(BasicSentence{one, plus, two})
    }
}

pub fn evaluate_math_lingo(input: TokenStream) -> TokenStream{
    let mut input_def  = match syn::parse2::<BasicSentence>(input) {
        Ok(data) => data,
        Err(err) => {
            return TokenStream::from(err.to_compile_error());
        }
    };
    quote::quote!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use assert_tokenstreams_eq::assert_tokenstreams_eq;

    #[test]
    fn simple_addition() {
        let input = quote!(one plus two);
        let output = evaluate_math_lingo(input);
        let expected_output = quote!(1 + 2);

        assert_tokenstreams_eq!(&output, &expected_output);
    }
}
