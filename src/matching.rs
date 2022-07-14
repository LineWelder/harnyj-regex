use crate::{cursor::Cursor, *};
use std::ops::Range;

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum ExpectedCharacter {
    Specific(char),
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum MatchingErrorType {
    ExtraCharacters,
    UnexpectedCharacter { expected: ExpectedCharacter },
}

#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
pub struct MatchingError {
    pub r#type: MatchingErrorType,
    pub location: Range<usize>,
}

struct PatternMatcher<'a>(Cursor<'a>);

impl<'a> PatternMatcher<'a> {
    fn check_character(&mut self, expected: char) -> Result<(), MatchingError> {
        match self.0.next() {
            Some(ch) if ch == expected => Ok(()),
            _ => {
                let location = self.0.get_next_location();
                Err(MatchingError {
                    r#type: MatchingErrorType::UnexpectedCharacter {
                        expected: ExpectedCharacter::Specific(expected),
                    },
                    location: (location - 1)..location,
                })
            }
        }
    }

    fn check_state(&mut self, state: &State) -> Result<(), MatchingError> {
        use Matching::*;

        if state.quantifier.min != 1 || state.quantifier.max != Some(1) {
            todo!("Quantifiers other than exactly one");
        }

        match state.matching {
            Character { value } => self.check_character(value),
            _ => todo!("States other than Character"),
        }
    }

    fn check(&mut self, states: &[State]) -> Result<(), MatchingError> {
        use MatchingErrorType::*;

        for state in states {
            self.check_state(state)?;
        }

        if self.0.peek().is_none() {
            Ok(())
        } else {
            let location = self.0.get_next_location();
            Err(MatchingError {
                r#type: ExtraCharacters,
                location: location..(location + self.0.len_left()),
            })
        }
    }
}

impl Pattern {
    pub fn check(&self, string: &str) -> Result<(), MatchingError> {
        PatternMatcher(Cursor::new(string)).check(&self.states)
    }
}

#[cfg(test)]
mod tests {
    use super::{MatchingErrorType::*, *};

    #[test]
    fn empty_pattern_matches_empty_string() {
        let pattern = Pattern::try_from("").unwrap();
        assert_eq!(pattern.check(""), Ok(()));
    }

    #[test]
    fn empty_pattern_doesnt_match_non_empty_string() {
        let pattern = Pattern::try_from("").unwrap();
        assert_eq!(
            pattern.check("test"),
            Err(MatchingError {
                r#type: ExtraCharacters,
                location: 0..4,
            })
        );
    }

    #[test]
    fn simple_characters() {
        let pattern = Pattern::try_from("abc").unwrap();
        assert_eq!(pattern.check("abc"), Ok(()));
    }

    #[test]
    fn wrong_character() {
        let pattern = Pattern::try_from("abc").unwrap();
        assert_eq!(
            pattern.check("afc"),
            Err(MatchingError {
                r#type: UnexpectedCharacter {
                    expected: ExpectedCharacter::Specific('b')
                },
                location: 1..2,
            })
        );
    }

    #[test]
    fn unexpected_end_of_string() {
        let pattern = Pattern::try_from("abc").unwrap();
        assert_eq!(
            pattern.check("ab"),
            Err(MatchingError {
                r#type: UnexpectedCharacter {
                    expected: ExpectedCharacter::Specific('c')
                },
                location: 2..3,
            })
        );
    }

    #[test]
    fn extra_characters() {
        let pattern = Pattern::try_from("abc").unwrap();
        assert_eq!(
            pattern.check("abcd"),
            Err(MatchingError {
                r#type: ExtraCharacters,
                location: 3..4,
            })
        );
    }
}
