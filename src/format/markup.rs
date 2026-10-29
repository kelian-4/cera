use super::Plan;
use crate::model::buffer::Buffer;

const VOID: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr", "keygen", "command",
];
const VERBATIM: &[&str] = &["pre", "textarea", "script", "style"];

enum State {
    Normal,
    Tag {
        level: usize,
        quote: Option<char>,
        name: String,
        closing: bool,
        special: bool,
    },
    Comment,
    Cdata,
    Verbatim(String),
}

fn is_name_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '-' | '_' | ':' | '.')
}

fn starts_with(chars: &[char], at: usize, pat: &str) -> bool {
    let p: Vec<char> = pat.chars().collect();
    chars.len() >= at + p.len() && chars[at..at + p.len()] == p[..]
}

fn starts_with_ci(chars: &[char], at: usize, pat: &str) -> bool {
    let p: Vec<char> = pat.chars().collect();
    chars.len() >= at + p.len()
        && chars[at..at + p.len()]
            .iter()
            .zip(p.iter())
            .all(|(a, b)| a.eq_ignore_ascii_case(b))
}

fn read_name(chars: &[char], from: usize, html: bool) -> String {
    let mut end = from;
    while end < chars.len() && is_name_char(chars[end]) {
        end += 1;
    }
    let name: String = chars[from..end].iter().collect();
    if html {
        name.to_ascii_lowercase()
    } else {
        name
    }
}

pub fn plan(buffer: &Buffer, html: bool, unit: usize) -> Option<Vec<Plan>> {
    let mut plans = Vec::with_capacity(buffer.num_rows());
    let mut stack: Vec<String> = Vec::new();
    let mut state = State::Normal;

    for i in 0..buffer.num_rows() {
        let chars: Vec<char> = buffer.row(i).chars().chars().collect();
        let first = chars.iter().position(|c| !c.is_whitespace());
        let blank = first.is_none();
        let at = first.unwrap_or(0);
        let closing_row = !blank && starts_with(&chars, at, "</");

        let closing_level = |stack: &Vec<String>| -> Option<usize> {
            let name = read_name(&chars, at + 2, html);
            if stack.last() == Some(&name) {
                Some(stack.len() - 1)
            } else {
                None
            }
        };

        let row_level;
        match &state {
            State::Comment | State::Cdata => {
                plans.push(Plan::Shift);
                row_level = stack.len();
            }
            State::Verbatim(name) => {
                if closing_row && starts_with_ci(&chars, at, &format!("</{name}")) {
                    let level = closing_level(&stack)?;
                    plans.push(Plan::Cols(level * unit));
                    row_level = level;
                } else {
                    plans.push(Plan::Raw);
                    row_level = stack.len();
                }
            }
            State::Tag { level, .. } => {
                let ends_tag = starts_with(&chars, at, ">") || starts_with(&chars, at, "/>");
                let l = if ends_tag { *level } else { *level + 1 };
                plans.push(if blank {
                    Plan::Blank
                } else {
                    Plan::Cols(l * unit)
                });
                row_level = *level;
            }
            State::Normal => {
                if blank {
                    plans.push(Plan::Blank);
                    row_level = stack.len();
                } else if closing_row {
                    let level = closing_level(&stack)?;
                    plans.push(Plan::Cols(level * unit));
                    row_level = level;
                } else {
                    plans.push(Plan::Cols(stack.len() * unit));
                    row_level = stack.len();
                }
            }
        }

        let mut j = 0;
        while j < chars.len() {
            match &mut state {
                State::Comment => {
                    if starts_with(&chars, j, "-->") {
                        state = State::Normal;
                        j += 3;
                    } else {
                        j += 1;
                    }
                }
                State::Cdata => {
                    if starts_with(&chars, j, "]]>") {
                        state = State::Normal;
                        j += 3;
                    } else {
                        j += 1;
                    }
                }
                State::Verbatim(name) => {
                    if starts_with_ci(&chars, j, &format!("</{name}")) {
                        state = State::Normal;
                    } else {
                        j += 1;
                    }
                }
                State::Normal => {
                    if chars[j] != '<' {
                        j += 1;
                    } else if starts_with(&chars, j, "<!--") {
                        state = State::Comment;
                        j += 4;
                    } else if starts_with(&chars, j, "<![CDATA[") {
                        state = State::Cdata;
                        j += 9;
                    } else if starts_with(&chars, j, "</")
                        && chars.get(j + 2).is_some_and(|c| is_name_char(*c))
                    {
                        let name = read_name(&chars, j + 2, html);
                        j += 2 + name.chars().count();
                        state = State::Tag {
                            level: row_level,
                            quote: None,
                            name,
                            closing: true,
                            special: false,
                        };
                    } else if chars.get(j + 1).is_some_and(|c| c.is_alphabetic()) {
                        let name = read_name(&chars, j + 1, html);
                        j += 1 + name.chars().count();
                        state = State::Tag {
                            level: row_level,
                            quote: None,
                            name,
                            closing: false,
                            special: false,
                        };
                    } else if chars.get(j + 1).is_some_and(|c| matches!(c, '!' | '?')) {
                        j += 2;
                        state = State::Tag {
                            level: row_level,
                            quote: None,
                            name: String::new(),
                            closing: false,
                            special: true,
                        };
                    } else {
                        j += 1;
                    }
                }
                State::Tag {
                    quote,
                    name,
                    closing,
                    special,
                    ..
                } => {
                    let c = chars[j];
                    if let Some(q) = *quote {
                        if c == q {
                            *quote = None;
                        }
                    } else if c == '"' || c == '\'' {
                        *quote = Some(c);
                    } else if c == '>' {
                        let self_closed = j > 0 && chars[j - 1] == '/';
                        let name = std::mem::take(name);
                        let closing = *closing;
                        let special = *special;
                        state = State::Normal;
                        if closing {
                            if stack.pop().as_ref() != Some(&name) {
                                return None;
                            }
                        } else if !special
                            && !self_closed
                            && !(html && VOID.contains(&name.as_str()))
                        {
                            let verbatim = html && VERBATIM.contains(&name.as_str());
                            stack.push(name.clone());
                            if verbatim {
                                state = State::Verbatim(name);
                            }
                        }
                    }
                    j += 1;
                }
            }
        }
    }

    if !matches!(state, State::Normal) || !stack.is_empty() {
        return None;
    }
    Some(plans)
}
