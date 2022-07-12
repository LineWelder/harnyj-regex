use std::iter::Enumerate;
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
    LoneQuantifier { location: usize },
}

struct PatternParser<'a> {
    location: usize,
    chars: Enumerate<Chars<'a>>,
}

impl<'a> PatternParser<'a> {
    fn new(input: &'a str) -> Self {
        PatternParser {
            location: 0,
            chars: input.chars().enumerate(),
        }
    }

    fn next(&mut self) -> Option<char> {
        let (next_location, next_char) = self.chars.next()?;
        self.location = next_location;
        Some(next_char)
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next().map(|x| x.1)
    }

    fn parse_matching(&mut self) -> Result<Matching, PatternParsingError> {
        use Matching::*;
        use PatternParsingError::*;

        match self.next().unwrap() {
            '?' | '*' | '+' => Err(LoneQuantifier { location: self.location }),
            ch => Ok(Character { value: ch }),
        }
    }

    fn parse_state(&mut self) -> Result<State, PatternParsingError> {
        Ok(State {
            matching: self.parse_matching()?,
            quantifier: Quantifier::exactly_one(),
        })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, PatternParsingError> {
        if self.peek().is_none() {
            return Err(PatternParsingError::EmptyInput);
        }

        let mut states = vec![];
        while self.peek().is_some() {
            states.push(self.parse_state()?);
        }

        Ok(Pattern { states })
    }
}

impl TryFrom<&str> for Pattern {
    type Error = PatternParsingError;

    fn try_from(input: &str) -> Result<Self, PatternParsingError> {
        PatternParser::new(input).parse_pattern()
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
