use crate::{cursor::Cursor, *};

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum PatternParsingError {
    LoneQuantifier { location: usize },
}

struct PatternParser<'a>(Cursor<'a>);

impl<'a> PatternParser<'a> {
    fn parse_matching(&mut self) -> Result<Matching, PatternParsingError> {
        use Matching::*;
        use PatternParsingError::*;

        match self.0.next().unwrap() {
            '?' | '*' | '+' => Err(LoneQuantifier {
                location: self.0.get_next_location() - 1,
            }),
            '.' => Ok(Wildcard),
            ch => Ok(Character { value: ch }),
        }
    }

    fn parse_quantifier(&mut self) -> Quantifier {
        let quantifier = match self.0.peek() {
            Some('?') => Quantifier::zero_or_one(),
            Some('*') => Quantifier::zero_or_more(),
            Some('+') => Quantifier::one_or_more(),
            _ => return Quantifier::exactly_one(),
        };

        self.0.next();
        quantifier
    }

    fn parse_state(&mut self) -> Result<State, PatternParsingError> {
        Ok(State {
            matching: self.parse_matching()?,
            quantifier: self.parse_quantifier(),
        })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, PatternParsingError> {
        let mut states = vec![];
        while self.0.peek().is_some() {
            states.push(self.parse_state()?);
        }

        Ok(Pattern { states })
    }
}

impl TryFrom<&str> for Pattern {
    type Error = PatternParsingError;

    fn try_from(input: &str) -> Result<Self, PatternParsingError> {
        PatternParser(Cursor::new(input)).parse_pattern()
    }
}

#[cfg(test)]
mod tests {
    use super::{Matching::*, PatternParsingError::*, *};

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
