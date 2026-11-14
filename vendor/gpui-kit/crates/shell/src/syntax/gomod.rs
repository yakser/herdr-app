//! HERDR: go.mod highlighting from the lexical rules at https://go.dev/ref/mod#go-mod-file-lexical.
//! Only `std`: the app's tests include this file directly.
use std::ops::Range;

enum Token {
    Word,
    Punctuation,
    Comment,
}

/// The styled ranges of a go.mod file, as ordered, non-overlapping byte
/// ranges: directives are "keyword", module paths "string", versions
/// "constant" and comments "comment". Punctuation and `=>` stay unstyled.
pub(crate) fn highlights(text: &str) -> Vec<(Range<usize>, &'static str)> {
    let mut highlights = Vec::new();
    // The directive of the `(` block the line is in.
    let mut block = None;
    let mut offset = 0;
    for line in text.split('\n') {
        highlight_line(
            line.strip_suffix('\r').unwrap_or(line),
            offset,
            &mut block,
            &mut highlights,
        );
        offset += line.len() + 1;
    }
    highlights
}

fn highlight_line<'a>(
    line: &'a str,
    offset: usize,
    block: &mut Option<&'a str>,
    highlights: &mut Vec<(Range<usize>, &'static str)>,
) {
    let mut directive = *block;
    let mut arguments = 0;
    // Each line of a block, and each directive, starts with a path.
    let mut expect_path = true;
    let mut at = 0;
    while let Some((range, token)) = next_token(line.as_bytes(), at) {
        at = range.end;
        let word = &line[range.clone()];
        let name = match token {
            Token::Comment => Some("comment"),
            Token::Punctuation => {
                if word == "(" && block.is_none() && directive.is_some() && arguments == 0 {
                    *block = directive;
                } else if word == ")" {
                    *block = None;
                }
                None
            }
            Token::Word if directive.is_none() => {
                directive = Some(word);
                Some("keyword")
            }
            Token::Word => {
                arguments += 1;
                argument(directive.unwrap_or_default(), word, &mut expect_path)
            }
        };
        if let Some(name) = name {
            highlights.push((offset + range.start..offset + range.end, name));
        }
    }
}

fn argument(directive: &str, word: &str, expect_path: &mut bool) -> Option<&'static str> {
    match directive {
        "go" | "toolchain" | "retract" => Some("constant"),
        // `path [version] [=> path [version]]`
        "module" | "require" | "exclude" | "replace" | "tool" | "ignore" => match word {
            "=>" => {
                *expect_path = true;
                None
            }
            _ if std::mem::replace(expect_path, false) => Some("string"),
            _ => Some("constant"),
        },
        // `godebug` settings and directives newer than this file.
        _ => None,
    }
}

fn is_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\r')
}

fn is_punctuation(byte: u8) -> bool {
    matches!(byte, b'(' | b')' | b'[' | b']' | b'{' | b'}' | b',')
}

/// The next token of `line` from `at`. Strings may not span lines, so an
/// unterminated one ends with the line.
fn next_token(line: &[u8], mut at: usize) -> Option<(Range<usize>, Token)> {
    while line.get(at).copied().is_some_and(is_space) {
        at += 1;
    }
    let start = at;
    let token = match *line.get(at)? {
        b'/' if line.get(at + 1) == Some(&b'/') => (start..line.len(), Token::Comment),
        byte if is_punctuation(byte) => (start..start + 1, Token::Punctuation),
        quote @ (b'"' | b'`') => {
            at += 1;
            while at < line.len() && line[at] != quote {
                at += if quote == b'"' && line[at] == b'\\' && at + 1 < line.len() {
                    2
                } else {
                    1
                };
            }
            (start..(at + 1).min(line.len()), Token::Word)
        }
        _ => {
            while at < line.len()
                && !is_space(line[at])
                && !is_punctuation(line[at])
                && !line[at..].starts_with(b"//")
            {
                at += 1;
            }
            (start..at, Token::Word)
        }
    };
    Some(token)
}
