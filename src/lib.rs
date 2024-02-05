use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::Result;

mod kw {
    syn::custom_keyword!(zero);
    syn::custom_keyword!(one);
    syn::custom_keyword!(two);
    syn::custom_keyword!(three);
    syn::custom_keyword!(four);
    syn::custom_keyword!(five);
    syn::custom_keyword!(six);
    syn::custom_keyword!(seven);
    syn::custom_keyword!(eight);
    syn::custom_keyword!(nine);
    
    syn::custom_keyword!(plus);
}

enum Number {
    Zero(kw::zero),
    One(kw::one),
    Two(kw::two),
    Three(kw::three),
    Four(kw::four),
    Five(kw::five),
    Six(kw::six),
    Seven(kw::seven),
    Eight(kw::eight),
    Nine(kw::nine),
}

struct BasicSentence {
    pub numer_left: Number,
    pub plus: kw::plus,
    pub numer_right: Number,
}

impl Parse for BasicSentence {
    fn parse(input: ParseStream) -> Result<Self> {
        let numer_left: Number = input.parse()?;
        let plus: kw::plus = input.parse()?;
        let numer_right: Number = input.parse()?;
        Ok(BasicSentence{numer_left, plus, numer_right})
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
