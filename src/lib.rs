#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
struct Quantifier {
    min: usize,
    max: Option<usize>,
}

#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
enum Matching {
    Character { value: char },
    Wildcard,
}

#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
struct State {
    matching: Matching,
    quantifier: Quantifier,
}

#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
pub struct Pattern {
    states: Vec<State>,
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum PatternParsingError {
    EmptyInput,
}

impl TryFrom<&str> for Pattern {
    type Error = PatternParsingError;

    fn try_from(input: &str) -> Result<Self, PatternParsingError> {
        Err(PatternParsingError::EmptyInput)
    }
}

#[cfg(test)]
mod tests {
    use super::{Matching::*, PatternParsingError::*, *};

    #[test]
    fn empty_regex_is_error() {
        assert_eq!(Pattern::try_from(""), Err(EmptyInput));
    }
}
