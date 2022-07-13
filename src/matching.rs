use crate::*;
use std::ops::Range;
use std::str::Chars;
use std::slice;

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum MatchingErrorType {
    ExtraCharacters,
}

#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
pub struct MatchingError {
    pub r#type: MatchingErrorType,
    pub location: Range<usize>,
}

struct PatternMatcher<'a> {
    chars: Chars<'a>,
}

impl<'a> PatternMatcher<'a> {
    fn new(string: &'a str) -> Self {
        PatternMatcher { chars: string.chars() }
    }

    fn check(&mut self, states: &[State]) -> Result<(), MatchingError> {
        use MatchingErrorType::*;

        if states.is_empty() {
            return if self.chars.clone().next().is_none() {
                Ok(())
            } else {
                Err(MatchingError {
                    r#type: ExtraCharacters,
                    location: 0..self.chars.clone().count(),
                })
            };
        }

        todo!();
    }
}

impl Pattern {
    pub fn check(&self, string: &str) -> Result<(), MatchingError> {
        PatternMatcher::new(string).check(&self.states)
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
}
