use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::Result;
use quote::quote;

mod kw {
    syn::custom_keyword!(plus);
    syn::custom_keyword!(minus);
    syn::custom_keyword!(time);
    syn::custom_keyword!(through);
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
        } else if lookahead.peek(kw::through) {
            input.parse::<kw::through>()?;
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
        assert_eq!(syn::parse2::<Operation>(quote!(through)).unwrap().quote().to_string(), quote!(/).to_string());
    }
}