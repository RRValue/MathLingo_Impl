#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simpple_addition() {
        let input = quote!(one plus two);
        let output = evaluate_math_lingo(input);
        let expected_output = quote!(3);

        assert_tokenstreams_eq!(&output, &expected_output);
    }
}
