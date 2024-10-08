use crate::{
    sequences::{fibonacci::Fibonacci, models::Sequence},
    structs::{custom_error::CustomError, range::Range, sequences::SequenceSyntax},
};

/// Returns vector of `Fibonacci` sequence terms in the given `range`.
pub fn fibonacci(syn: &SequenceSyntax, range: &Range) -> Result<Vec<f64>, CustomError> {
    Ok(Fibonacci::new(syn.parameters[0], syn.parameters[1]).range(&range))
}
