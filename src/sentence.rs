use crate::number_rep::NumberRep;
use crate::operation::Operation;

use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::Result;
use quote::quote;

pub struct Sentence {
    number_left: NumberRep,
    operation: Operation,
    number_right: NumberRep,
}

impl Parse for Sentence {
    fn parse(input: ParseStream) -> Result<Self> {
        let number_left: NumberRep = input.parse()?;
        let operation: Operation = input.parse()?;
        let number_right: NumberRep = input.parse()?;
        Ok(Sentence{number_left, operation, number_right})
    }
}

impl Sentence {
    pub fn quote(&self) -> TokenStream{
        let left_ts = self.number_left.quote();
        let operation_ts = self.operation.quote();
        let right_ts = self.number_right.quote();

        quote!(#left_ts #operation_ts #right_ts)
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
}