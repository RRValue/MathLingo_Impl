use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::Result;
use quote::quote;

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
    syn::custom_keyword!(minus);
    syn::custom_keyword!(time);
    syn::custom_keyword!(through);
}

enum Number {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Parse for Number {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::zero) {
            input.parse::<kw::zero>()?;
            Ok(Number::Zero)
        } else if lookahead.peek(kw::one) {
            input.parse::<kw::one>()?;
            Ok(Number::One)
        } else if lookahead.peek(kw::two) {
            input.parse::<kw::two>()?;
            Ok(Number::Two)
        } else if lookahead.peek(kw::three) {
            input.parse::<kw::three>()?;
            Ok(Number::Three)
        } else if lookahead.peek(kw::four) {
            input.parse::<kw::four>()?;
            Ok(Number::Four)
        } else if lookahead.peek(kw::five) {
            input.parse::<kw::five>()?;
            Ok(Number::Five)
        } else if lookahead.peek(kw::six) {
            input.parse::<kw::six>()?;
            Ok(Number::Six)
        } else if lookahead.peek(kw::seven) {
            input.parse::<kw::seven>()?;
            Ok(Number::Seven)
        } else if lookahead.peek(kw::eight) {
            input.parse::<kw::eight>()?;
            Ok(Number::Eight)
        } else if lookahead.peek(kw::nine) {
            input.parse::<kw::nine>()?;
            Ok(Number::Nine)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Number {
    fn quote(&self) -> TokenStream{
        match self {
            Number::Zero => quote!(0),
            Number::One => quote!(1),
            Number::Two => quote!(2),
            Number::Three => quote!(3),
            Number::Four => quote!(4),
            Number::Five => quote!(5),
            Number::Six => quote!(6),
            Number::Seven => quote!(7),
            Number::Eight => quote!(8),
            Number::Nine => quote!(9),
        }
    }
}

enum Operation {
    Plus,
    Minus,
    Time,
    Through,
}

impl Parse for Operation {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::plus) {
            input.parse::<kw::plus>()?;
            Ok(Operation::Plus)
        } else if lookahead.peek(kw::minus) {
            input.parse::<kw::minus>()?;
            Ok(Operation::Minus)
        } else if lookahead.peek(kw::time) {
            input.parse::<kw::time>()?;
            Ok(Operation::Time)
        } else if lookahead.peek(kw::through) {
            input.parse::<kw::through>()?;
            Ok(Operation::Through)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Operation {
    fn quote(&self) -> TokenStream{
        match self {
            Operation::Plus => quote!(+),
            Operation::Minus => quote!(-),
            Operation::Time => quote!(*),
            Operation::Through => quote!(/),
        }
    }
}

struct BasicSentence {
    pub numer_left: Number,
    pub operation: Operation,
    pub numer_right: Number,
}

impl Parse for BasicSentence {
    fn parse(input: ParseStream) -> Result<Self> {
        let numer_left: Number = input.parse()?;
        let operation: Operation = input.parse()?;
        let numer_right: Number = input.parse()?;
        Ok(BasicSentence{numer_left, operation, numer_right})
    }
}

pub fn evaluate_math_lingo(input: TokenStream) -> TokenStream{
    let sentence  = match syn::parse2::<BasicSentence>(input) {
        Ok(data) => data,
        Err(err) => {
            return TokenStream::from(err.to_compile_error());
        }
    };

    let left_ts = sentence.numer_left.quote();
    let operation_ts = sentence.operation.quote();
    let right_ts = sentence.numer_right.quote();

    quote!(#left_ts #operation_ts #right_ts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_addition() {
        let input = quote!(one plus two);
        let input_str = format!("{}", input);
        
        let output = evaluate_math_lingo(input);
        let output_str = format!("{}", output);

        let expected_output = quote!(1 + 2);
        let expected_output_str = format!("{}", expected_output);

        assert_eq!(output_str, expected_output_str)
    }
}
