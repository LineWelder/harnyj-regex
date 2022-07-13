use std::iter::Enumerate;
use std::str::Chars;

pub(crate) struct Cursor<'a> {
    location: usize,
    chars: Enumerate<Chars<'a>>,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Cursor {
            location: 0,
            chars: input.chars().enumerate(),
        }
    }

    pub(crate) fn next(&mut self) -> Option<char> {
        let (next_location, next_char) = self.chars.next()?;
        self.location = next_location;
        Some(next_char)
    }

    pub(crate) fn peek(&self) -> Option<char> {
        self.chars.clone().next().map(|x| x.1)
    }

    pub(crate) fn get_location(&self) -> usize {
        self.location
    }

    pub(crate) fn len_left(&self) -> usize {
        self.chars.clone().count()
    }
}
