mod cursor;
mod matching;
mod parsing;

pub use matching::*;
pub use parsing::PatternParsingError;

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
