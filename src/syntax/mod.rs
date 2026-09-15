pub mod c_syntax;
pub mod javascript_syntax;
pub mod markdown_syntax;
pub mod python_syntax;

use crate::model::buffer::Buffer;
use crate::model::row::Highlight;

bitflags::bitflags! {
    pub struct HighlightFlags: u32 {
        const NUMBERS = 1 << 0;
        const STRINGS = 1 << 1;
    }
}

pub struct Syntax {
    pub filetype: &'static str,
    pub filematch: &'static [&'static str],
    pub keywords: &'static [&'static str],
    pub singleline_comment_start: &'static str,
    pub multiline_comment_start: &'static str,
    pub multiline_comment_end: &'static str,
    pub flags: HighlightFlags,
}

pub fn matches(syntax: &Syntax, filename: &str) -> bool {
    let ext = filename.rfind('.').map(|i| &filename[i..]);

    for pattern in syntax.filematch {
        let is_ext = pattern.starts_with('.');
        if is_ext {
            if let Some(e) = ext {
                if e == *pattern {
                    return true;
                }
            }
        } else if filename.contains(pattern) {
            return true;
        }
    }
    false
}

fn is_separator(c: char) -> bool {
    c.is_whitespace() || c == '\0' || ",.()+-/*=~%<>[];".contains(c)
}

fn is_keyword2(kw: &str) -> bool {
    kw.ends_with('|')
}

pub fn update_syntax(buffer: &mut Buffer, row_idx: usize, syntax: Option<&Syntax>) {
    let rsize = buffer.row(row_idx).rsize();
    buffer.row_mut(row_idx).hl_mut().clear();
    buffer
        .row_mut(row_idx)
        .hl_mut()
        .resize(rsize, Highlight::Normal);

    let syntax = match syntax {
        Some(s) => s,
        None => return,
    };

    let render: Vec<char> = buffer.row(row_idx).render().chars().collect();
    let mut hl_by_char = vec![Highlight::Normal; render.len()];

    let scs = syntax.singleline_comment_start;
    let mcs = syntax.multiline_comment_start;
    let mce = syntax.multiline_comment_end;

    let mut prev_sep = true;
    let mut in_string: Option<char> = None;
    let mut in_comment = row_idx > 0 && buffer.row(row_idx - 1).hl_open_comment;

    let matches_at = |chars: &[char], i: usize, pat: &str| -> bool {
        if pat.is_empty() {
            return false;
        }
        let pat_chars: Vec<char> = pat.chars().collect();
        if i + pat_chars.len() > chars.len() {
            return false;
        }
        chars[i..i + pat_chars.len()] == pat_chars[..]
    };

    let n = render.len();
    let mut i = 0usize;

    while i < n {
        let c = render[i];
        let prev_hl = if i > 0 {
            hl_by_char[i - 1]
        } else {
            Highlight::Normal
        };

        if !scs.is_empty() && in_string.is_none() && !in_comment && matches_at(&render, i, scs) {
            for slot in hl_by_char.iter_mut().skip(i) {
                *slot = Highlight::Comment;
            }
            break;
        }

        if !mcs.is_empty() && !mce.is_empty() && in_string.is_none() {
            if in_comment {
                hl_by_char[i] = Highlight::MlComment;
                if matches_at(&render, i, mce) {
                    let len = mce.chars().count();
                    for k in 0..len {
                        hl_by_char[i + k] = Highlight::MlComment;
                    }
                    i += len;
                    in_comment = false;
                    prev_sep = true;
                    continue;
                }
                i += 1;
                continue;
            } else if matches_at(&render, i, mcs) {
                let len = mcs.chars().count();
                for k in 0..len {
                    hl_by_char[i + k] = Highlight::MlComment;
                }
                i += len;
                in_comment = true;
                continue;
            }
        }

        if syntax.flags.contains(HighlightFlags::STRINGS) {
            if let Some(quote) = in_string {
                hl_by_char[i] = Highlight::String;
                if c == '\\' && i + 1 < n {
                    hl_by_char[i + 1] = Highlight::String;
                    i += 2;
                    continue;
                }
                if c == quote {
                    in_string = None;
                }
                i += 1;
                prev_sep = true;
                continue;
            } else if c == '"' || c == '\'' {
                in_string = Some(c);
                hl_by_char[i] = Highlight::String;
                i += 1;
                continue;
            }
        }

        if syntax.flags.contains(HighlightFlags::NUMBERS)
            && ((c.is_ascii_digit() && (prev_sep || prev_hl == Highlight::Number))
                || (c == '.' && prev_hl == Highlight::Number))
        {
            hl_by_char[i] = Highlight::Number;
            i += 1;
            prev_sep = false;
            continue;
        }

        if prev_sep {
            let mut matched = false;
            for kw in syntax.keywords {
                let kw2 = is_keyword2(kw);
                let kw_body = if kw2 { &kw[..kw.len() - 1] } else { kw };
                let klen = kw_body.chars().count();

                if matches_at(&render, i, kw_body) {
                    let next_char = render.get(i + klen).copied().unwrap_or('\0');
                    if is_separator(next_char) {
                        let h = if kw2 {
                            Highlight::Keyword2
                        } else {
                            Highlight::Keyword1
                        };
                        for slot in hl_by_char.iter_mut().skip(i).take(klen) {
                            *slot = h;
                        }
                        i += klen;
                        matched = true;
                        break;
                    }
                }
            }
            if matched {
                prev_sep = false;
                continue;
            }
        }

        prev_sep = is_separator(c);
        i += 1;
    }

    {
        let row = buffer.row_mut(row_idx);
        for (idx, h) in hl_by_char.into_iter().enumerate() {
            row.hl_mut()[idx] = h;
        }
    }

    let changed = buffer.row(row_idx).hl_open_comment != in_comment;
    buffer.row_mut(row_idx).hl_open_comment = in_comment;
    if changed && row_idx + 1 < buffer.num_rows() {
        update_syntax(buffer, row_idx + 1, Some(syntax));
    }
}

pub fn select_for_filename<'a>(database: &'a [Syntax], filename: &str) -> Option<&'a Syntax> {
    database.iter().find(|s| matches(s, filename))
}

pub fn build_database() -> Vec<Syntax> {
    vec![
        c_syntax::c_syntax(),
        python_syntax::python_syntax(),
        javascript_syntax::javascript_syntax(),
        markdown_syntax::markdown_syntax(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_selection() {
        let db = build_database();
        let c = select_for_filename(&db, "main.cpp");
        assert!(c.is_some());
        assert_eq!(c.unwrap().filetype, "c");

        let py = select_for_filename(&db, "script.py");
        assert!(py.is_some());
        assert_eq!(py.unwrap().filetype, "python");

        let js = select_for_filename(&db, "app.ts");
        assert!(js.is_some());
        assert_eq!(js.unwrap().filetype, "javascript");

        let none = select_for_filename(&db, "data.xyz");
        assert!(none.is_none());
    }

    #[test]
    fn test_update_syntax_keyword() {
        let db = build_database();
        let py = select_for_filename(&db, "script.py").unwrap();

        let mut buf = Buffer::new();
        buf.insert_row(0, "def foo():");
        update_syntax(&mut buf, 0, Some(py));

        assert_eq!(buf.row(0).hl()[0], Highlight::Keyword1);
    }
}
