use crate::{cursor::Cursor, *};
use std::ops::Range;

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

struct PatternMatcher<'a>(Cursor<'a>);

impl<'a> PatternMatcher<'a> {
    fn check(&mut self, states: &[State]) -> Result<(), MatchingError> {
        use MatchingErrorType::*;

        if !states.is_empty() {
            todo!();
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
}
