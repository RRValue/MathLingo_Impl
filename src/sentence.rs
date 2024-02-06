use crate::number_rep::NumberRep;
use crate::operation::Operation;

use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::{Result, parenthesized};
use quote::quote;

enum Operant {
    Number(NumberRep),
    Sentence(Box<Sentence>),
}

impl Parse for Operant {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Paren) {
            let content;
            parenthesized!(content in input);
            let sentence = content.parse()?;
            
            Ok(Operant::Sentence(Box::new(sentence)))
        }
        else {
            let number_rep: NumberRep = input.parse()?;

            Ok(Operant::Number(number_rep))
        }
    }
}

impl Operant {
    pub fn quote(&self) -> TokenStream{
        match self {
            Operant::Number(number_rep) => number_rep.quote(),
            Operant::Sentence(sentence) => sentence.quote_parenthesized(),
        }
    }
}

pub struct Sentence {
    left_operant: Operant,
    operation: Operation,
    right_operant: Operant,
}

impl Parse for Sentence {
    fn parse(input: ParseStream) -> Result<Self> {
        let left_operant: Operant = input.parse()?;
        let operation: Operation = input.parse()?;
        let right_operant: Operant = input.parse()?;
        Ok(Sentence{left_operant, operation, right_operant})
    }
}

impl Sentence {
    pub fn quote(&self) -> TokenStream{
        self.quote_impl(false)
    }

    pub fn quote_parenthesized(&self) -> TokenStream{
        self.quote_impl(true)
    }

    fn quote_impl(&self, parenthesized: bool) -> TokenStream{
        let left_ts = self.left_operant.quote();
        let operation_ts = self.operation.quote();
        let right_ts = self.right_operant.quote();

        if parenthesized {
            quote!((#left_ts #operation_ts #right_ts))
        }
        else {
            quote!(#left_ts #operation_ts #right_ts)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentences() {        
        assert_eq!(syn::parse2::<Sentence>(quote!(1 plus 2)).unwrap().quote().to_string(), quote!(1.0 + 2.0).to_string());
        assert_eq!(syn::parse2::<Sentence>(quote!(1.11 plus 2.22)).unwrap().quote().to_string(), quote!(1.11 + 2.22).to_string());
        assert_eq!(syn::parse2::<Sentence>(quote!(one through three)).unwrap().quote().to_string(), quote!(1.0 / 3.0).to_string());
        assert_eq!(syn::parse2::<Sentence>(quote!(1 through 3.0)).unwrap().quote().to_string(), quote!(1.0 / 3.0).to_string());
    }

    #[test]
    fn test_sentences_parenthese() {        
        assert_eq!(syn::parse2::<Sentence>(quote!((1 + 1) + 2)).unwrap().quote().to_string(), quote!((1.0 + 1.0) + 2.0).to_string());
        assert_eq!(syn::parse2::<Sentence>(quote!(1 + (1 + 2))).unwrap().quote().to_string(), quote!(1.0 + (1.0 + 2.0)).to_string());
        assert_eq!(syn::parse2::<Sentence>(quote!((1 + 1) + (1 + 2))).unwrap().quote().to_string(), quote!((1.0 + 1.0) + (1.0 + 2.0)).to_string());
    }
}