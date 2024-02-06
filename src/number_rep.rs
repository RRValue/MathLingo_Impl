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
}

pub enum Number {
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

pub enum NumberRep {
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
    pub fn quote(&self) -> TokenStream{
        match self {
            NumberRep::NumberStr(value) => value.quote(),
            NumberRep::NumberLitInt(lit) => lit.to_token_stream(),
            NumberRep::NumberLitFloat(lit) => lit.to_token_stream(),
        }
    }
}