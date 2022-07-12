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

    const fn zero_or_one() -> Self {
        Quantifier {
            min: 0,
            max: Some(1),
        }
    }

    const fn zero_or_more() -> Self {
        Quantifier { min: 0, max: None }
    }

    const fn one_or_more() -> Self {
        Quantifier { min: 1, max: None }
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
            '?' | '*' | '+' => Err(LoneQuantifier {
                location: self.location,
            }),
            '.' => Ok(Wildcard),
            ch => Ok(Character { value: ch }),
        }
    }

    fn parse_quantifier(&mut self) -> Quantifier {
        let quantifier = match self.peek() {
            Some('?') => Quantifier::zero_or_one(),
            Some('*') => Quantifier::zero_or_more(),
            Some('+') => Quantifier::one_or_more(),
            _ => return Quantifier::exactly_one(),
        };

        self.next();
        quantifier
    }

    fn parse_state(&mut self) -> Result<State, PatternParsingError> {
        Ok(State {
            matching: self.parse_matching()?,
            quantifier: self.parse_quantifier(),
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

    #[test]
    fn quantifier_in_the_beginning_is_error() {
        let pattern = Pattern::try_from("?");
        assert_eq!(pattern, Err(LoneQuantifier { location: 0 }));
    }

    #[test]
    fn two_quantifiers_is_error() {
        let pattern = Pattern::try_from("a?*b");
        assert_eq!(pattern, Err(LoneQuantifier { location: 2 }));
    }

    #[test]
    fn right_quantifiers() {
        let pattern = Pattern::try_from("a?b*c+");
        assert_eq!(
            pattern,
            Ok(Pattern {
                states: vec![
                    State {
                        matching: Character { value: 'a' },
                        quantifier: Quantifier::zero_or_one(),
                    },
                    State {
                        matching: Character { value: 'b' },
                        quantifier: Quantifier::zero_or_more(),
                    },
                    State {
                        matching: Character { value: 'c' },
                        quantifier: Quantifier::one_or_more(),
                    },
                ],
            })
        );
    }

    #[test]
    fn wildcard() {
        let pattern = Pattern::try_from(".*");
        assert_eq!(
            pattern,
            Ok(Pattern {
                states: vec![State {
                    matching: Wildcard,
                    quantifier: Quantifier::zero_or_more(),
                }],
            })
        );
    }
}
