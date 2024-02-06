use super::*;

use proc_macro2::TokenStream;
use quote::quote;

pub fn evaluate_math_lingo(input: TokenStream) -> TokenStream{
    let sentence  = match syn::parse2::<Sentence>(input) {
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