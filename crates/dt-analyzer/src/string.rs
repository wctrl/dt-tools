use std::{iter::Peekable, str::Chars};

#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum StringParseError {
    /// escape at end of string
    EscapeAtEndOfString,
    /// hex escape with no valid digits
    HexNoDigits,
}

struct InterpretEscapedString<'a> {
    s: Peekable<Chars<'a>>,
}

impl Iterator for InterpretEscapedString<'_> {
    type Item = Result<char, StringParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.s.next().map(|c| match c {
            '\\' => match self.s.next() {
                None => Err(StringParseError::EscapeAtEndOfString),
                Some('a') => Ok('\x07'),
                Some('b') => Ok('\x08'),
                Some('v') => Ok('\x0b'),
                Some('f') => Ok('\x0c'),
                Some('n') => Ok('\n'),
                Some('r') => Ok('\r'),
                Some('t') => Ok('\t'),
                Some('\\') => Ok('\\'),
                Some('x') => {
                    let Some(mut num) = self.s.next().and_then(|c| c.to_digit(16)) else {
                        return Err(StringParseError::HexNoDigits);
                    };
                    if let Some(second) = self.s.peek().and_then(|c| c.to_digit(16)) {
                        self.s.next();
                        num += second << 4;
                    }

                    #[expect(
                        clippy::cast_possible_truncation,
                        reason = "guaranteed to be in range"
                    )]
                    Ok(num as u8 as char)
                }
                Some(c) => Ok(c),
            },
            c => Ok(c),
        })
    }
}

pub fn interpret_escaped_string(s: &str) -> Result<String, StringParseError> {
    debug_assert!(s.starts_with('"'));
    debug_assert!(s.ends_with('"'));
    let s = s.get(1..(s.len() - 1)).expect("lexer safe");
    (InterpretEscapedString {
        s: s.chars().peekable(),
    })
    .collect()
}
