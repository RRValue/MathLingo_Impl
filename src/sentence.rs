use super::*;

use syn::parse::{Parse, ParseStream};
use syn::Result;

pub struct Sentence {
    pub number_left: NumberRep,
    pub operation: Operation,
    pub number_right: NumberRep,
}

impl Parse for Sentence {
    fn parse(input: ParseStream) -> Result<Self> {
        let number_left: NumberRep = input.parse()?;
        let operation: Operation = input.parse()?;
        let number_right: NumberRep = input.parse()?;
        Ok(Sentence{number_left, operation, number_right})
    }
}