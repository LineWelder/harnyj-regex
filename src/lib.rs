use std::str::Chars;

#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
struct Quantifier {
    min: usize,
    max: Option<usize>,
}

impl Quantifier {
    const fn exactly_one() -> Self {
        Quantifier {
            min: 1,
            max: Some(1),
        }
    }
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

fn parse_matching(chars: &mut Chars) -> Result<Matching, PatternParsingError> {
    use Matching::*;

    match chars.next().unwrap() {
        ch => Ok(Character { value: ch }),
    }
}

fn parse_state(chars: &mut Chars) -> Result<State, PatternParsingError> {
    Ok(State {
        matching: parse_matching(chars)?,
        quantifier: Quantifier::exactly_one(),
    })
}

impl TryFrom<&str> for Pattern {
    type Error = PatternParsingError;

    fn try_from(input: &str) -> Result<Self, PatternParsingError> {
        if input.is_empty() {
            return Err(PatternParsingError::EmptyInput);
        }

        let mut chars = input.chars();
        let mut states = vec![];
        while chars.clone().next().is_some() {
            states.push(parse_state(&mut chars)?);
        }

        Ok(Pattern { states })
    }
}

#[cfg(test)]
mod tests {
    use super::{Matching::*, PatternParsingError::*, *};

    #[test]
    fn empty_regex_is_error() {
        assert_eq!(Pattern::try_from(""), Err(EmptyInput));
    }

    #[test]
    fn simple_letters() {
        let pattern = Pattern::try_from("abc");
        assert_eq!(
            pattern,
            Ok(Pattern {
                states: vec![
                    State {
                        matching: Character { value: 'a' },
                        quantifier: Quantifier::exactly_one(),
                    },
                    State {
                        matching: Character { value: 'b' },
                        quantifier: Quantifier::exactly_one(),
                    },
                    State {
                        matching: Character { value: 'c' },
                        quantifier: Quantifier::exactly_one(),
                    },
                ],
            })
        );
    }
}
