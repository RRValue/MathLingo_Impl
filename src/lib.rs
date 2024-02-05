use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{LitFloat, LitInt, Result};
use quote::{quote, ToTokens};

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

enum NumberRep {
    NumberStr(Number),
    NumberLitInt(LitInt),
    NumberLitFloat(LitFloat)
}

impl Parse for NumberRep {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::zero) {
            input.parse::<kw::zero>()?;
            Ok(NumberRep::NumberStr(Number::Zero))
        } else if lookahead.peek(kw::one) {
            input.parse::<kw::one>()?;
            Ok(NumberRep::NumberStr(Number::One))
        } else if lookahead.peek(kw::two) {
            input.parse::<kw::two>()?;
            Ok(NumberRep::NumberStr(Number::Two))
        } else if lookahead.peek(kw::three) {
            input.parse::<kw::three>()?;
            Ok(NumberRep::NumberStr(Number::Three))
        } else if lookahead.peek(kw::four) {
            input.parse::<kw::four>()?;
            Ok(NumberRep::NumberStr(Number::Four))
        } else if lookahead.peek(kw::five) {
            input.parse::<kw::five>()?;
            Ok(NumberRep::NumberStr(Number::Five))
        } else if lookahead.peek(kw::six) {
            input.parse::<kw::six>()?;
            Ok(NumberRep::NumberStr(Number::Six))
        } else if lookahead.peek(kw::seven) {
            input.parse::<kw::seven>()?;
            Ok(NumberRep::NumberStr(Number::Seven))
        } else if lookahead.peek(kw::eight) {
            input.parse::<kw::eight>()?;
            Ok(NumberRep::NumberStr(Number::Eight))
        } else if lookahead.peek(kw::nine) {
            input.parse::<kw::nine>()?;
            Ok(NumberRep::NumberStr(Number::Nine))
        } else if lookahead.peek(LitInt) {
            Ok(NumberRep::NumberLitInt(input.parse::<LitInt>()?))
        } else if lookahead.peek(LitFloat) {
            Ok(NumberRep::NumberLitFloat(input.parse::<LitFloat>()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl NumberRep {
    fn quote(&self) -> TokenStream{
        match self {
            NumberRep::NumberStr(value) => value.quote(),
            NumberRep::NumberLitInt(lit) => lit.to_token_stream(),
            NumberRep::NumberLitFloat(lit) => lit.to_token_stream(),
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
    pub number_left: NumberRep,
    pub operation: Operation,
    pub number_right: NumberRep,
}

impl Parse for BasicSentence {
    fn parse(input: ParseStream) -> Result<Self> {
        let number_left: NumberRep = input.parse()?;
        let operation: Operation = input.parse()?;
        let number_right: NumberRep = input.parse()?;
        Ok(BasicSentence{number_left, operation, number_right})
    }
}

pub fn evaluate_math_lingo(input: TokenStream) -> TokenStream{
    let sentence  = match syn::parse2::<BasicSentence>(input) {
        Ok(data) => data,
        Err(err) => {
            return TokenStream::from(err.to_compile_error());
        }
    };

    let left_ts = sentence.number_left.quote();
    let operation_ts = sentence.operation.quote();
    let right_ts = sentence.number_right.quote();

    quote!(#left_ts #operation_ts #right_ts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_addition_int_number() {        
        let output = evaluate_math_lingo(quote!(1 plus 2));
        let expected_output = quote!(1 + 2);
        
        assert_eq!(output.to_string(), expected_output.to_string())
    }

    #[test]
    fn simple_addition_float_number() {        
        let output = evaluate_math_lingo(quote!(1.11 plus 2.22));
        let expected_output = quote!(1.11 + 2.22);
        
        assert_eq!(output.to_string(), expected_output.to_string())
    }

    #[test]
    fn simple_addition_named_number() {        
        let output = evaluate_math_lingo(quote!(one plus two));
        let expected_output = quote!(1 + 2);
        
        assert_eq!(output.to_string(), expected_output.to_string())
    }
}
