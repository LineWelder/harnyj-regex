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
