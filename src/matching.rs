use crate::*;
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

impl Pattern {
    pub fn check(&self, string: &str) -> Result<(), MatchingError> {
        use MatchingErrorType::*;

        if self.states.is_empty() {
            return if string.is_empty() {
                Ok(())
            } else {
                Err(MatchingError {
                    r#type: ExtraCharacters,
                    location: 0..string.len(),
                })
            };
        }

        todo!();
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
