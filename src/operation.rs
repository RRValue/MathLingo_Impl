use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};
use quote::quote;

mod kw {
    syn::custom_keyword!(plus);    // -> '+'
    syn::custom_keyword!(minus);   // -> '-'
    syn::custom_keyword!(time);    // -> '*'
    syn::custom_keyword!(times);   // -> '*'
    syn::custom_keyword!(through); // -> '/'
    syn::custom_keyword!(devided); // in conjunction with "by" -> '/'
    syn::custom_keyword!(by);      // in conjunction with "devided" -> '/'
}

pub enum Operation {
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
        } else if lookahead.peek(kw::times) {
            input.parse::<kw::times>()?;
            Ok(Operation::Time)
        } else if lookahead.peek(kw::through) {
            input.parse::<kw::through>()?;
            Ok(Operation::Through)
        } else if lookahead.peek(kw::devided) {
            input.parse::<kw::devided>()?;
            input.parse::<kw::by>()?;
            Ok(Operation::Through)
        } else if lookahead.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            Ok(Operation::Plus)
        } else if lookahead.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            Ok(Operation::Minus)
        } else if lookahead.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            Ok(Operation::Time)
        } else if lookahead.peek(Token![/]) {
            input.parse::<Token![/]>()?;
            Ok(Operation::Through)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Operation {
    pub fn quote(&self) -> TokenStream{
        match self {
            Operation::Plus => quote!(+),
            Operation::Minus => quote!(-),
            Operation::Time => quote!(*),
            Operation::Through => quote!(/),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_operation_token() {        
        assert_eq!(syn::parse2::<Operation>(quote!(plus)).unwrap().quote().to_string(), quote!(+).to_string());
        assert_eq!(syn::parse2::<Operation>(quote!(minus)).unwrap().quote().to_string(), quote!(-).to_string());
        assert_eq!(syn::parse2::<Operation>(quote!(time)).unwrap().quote().to_string(), quote!(*).to_string());
        assert_eq!(syn::parse2::<Operation>(quote!(times)).unwrap().quote().to_string(), quote!(*).to_string());
        assert_eq!(syn::parse2::<Operation>(quote!(through)).unwrap().quote().to_string(), quote!(/).to_string());
        assert_eq!(syn::parse2::<Operation>(quote!(devided by)).unwrap().quote().to_string(), quote!(/).to_string());
    }

    #[test]
    fn parse_operation_literals() {        
        assert_eq!(syn::parse2::<Operation>(quote!(+)).unwrap().quote().to_string(), quote!(+).to_string());
        assert_eq!(syn::parse2::<Operation>(quote!(-)).unwrap().quote().to_string(), quote!(-).to_string());
        assert_eq!(syn::parse2::<Operation>(quote!(*)).unwrap().quote().to_string(), quote!(*).to_string());
        assert_eq!(syn::parse2::<Operation>(quote!(/)).unwrap().quote().to_string(), quote!(/).to_string());
    }
}