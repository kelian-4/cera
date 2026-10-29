use crate::model::buffer::Buffer;
use crate::model::row::Highlight;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok {
    Open(char),
    Close(char),
    Word(String),
    Sym(char),
    Lit,
}

pub struct LineInfo {
    pub toks: Vec<Tok>,
    pub lead: usize,
    pub blank: bool,
    pub comment_only: bool,
    pub start_comment: bool,
    pub start_string: bool,
    pub text: String,
}

fn is_code(h: Highlight) -> bool {
    !matches!(
        h,
        Highlight::Comment | Highlight::MlComment | Highlight::String | Highlight::Escape
    )
}

fn is_comment(h: Highlight) -> bool {
    matches!(h, Highlight::Comment | Highlight::MlComment)
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

pub fn analyze(buffer: &Buffer, idx: usize, brackets: &str) -> LineInfo {
    let row = buffer.row(idx);
    let render: Vec<char> = row.render().chars().collect();
    let hl = row.hl();
    let (start_comment, start_string) = if idx == 0 {
        (false, false)
    } else {
        let s = buffer.row(idx - 1).hl_state;
        (
            s.comment || s.inner_comment,
            s.string != 0 || s.embed != 0 || s.inner_string != 0,
        )
    };

    let lead = render.iter().take_while(|c| **c == ' ').count();
    let blank = render.iter().all(|c| c.is_whitespace());
    let hl_at = |i: usize| hl.get(i).copied().unwrap_or(Highlight::Normal);

    let mut toks = Vec::new();
    let mut comment_only = false;
    let mut seen_first = false;
    let mut i = lead;
    while i < render.len() {
        let c = render[i];
        let h = hl_at(i);
        if !seen_first && !c.is_whitespace() {
            seen_first = true;
            comment_only = is_comment(h);
        }
        if is_comment(h) {
            i += 1;
            continue;
        }
        if !is_code(h) {
            if toks.last() != Some(&Tok::Lit) {
                toks.push(Tok::Lit);
            }
            i += 1;
            continue;
        }
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if is_word_char(c) {
            let start = i;
            while i < render.len() && is_word_char(render[i]) && is_code(hl_at(i)) {
                i += 1;
            }
            toks.push(Tok::Word(render[start..i].iter().collect()));
            continue;
        }
        if brackets.contains(c) {
            match c {
                '(' | '[' | '{' => toks.push(Tok::Open(c)),
                _ => toks.push(Tok::Close(c)),
            }
        } else {
            toks.push(Tok::Sym(c));
        }
        i += 1;
    }

    LineInfo {
        toks,
        lead,
        blank,
        comment_only,
        start_comment,
        start_string,
        text: row.chars().trim().to_string(),
    }
}

pub fn pairs(open: char, close: char) -> bool {
    matches!((open, close), ('(', ')') | ('[', ']') | ('{', '}'))
}

pub fn heredoc_tag(info: &LineInfo, allow_space: bool, strict_ruby: bool) -> Option<String> {
    let has_shift = info
        .toks
        .windows(2)
        .any(|w| w[0] == Tok::Sym('<') && w[1] == Tok::Sym('<'));
    if !has_shift {
        return None;
    }
    let chars: Vec<char> = info.text.chars().collect();
    let mut i = 0;
    while i + 1 < chars.len() {
        if chars[i] == '<' && chars[i + 1] == '<' {
            if chars.get(i + 2) == Some(&'<') {
                i += 3;
                continue;
            }
            let mut j = i + 2;
            let mut flagged = false;
            if matches!(chars.get(j), Some('-') | Some('~')) {
                flagged = true;
                j += 1;
            }
            if allow_space {
                while chars.get(j) == Some(&' ') {
                    j += 1;
                }
            }
            let mut quoted = false;
            if matches!(chars.get(j), Some('\'') | Some('"')) {
                quoted = true;
                j += 1;
            }
            let start = j;
            while j < chars.len() && is_word_char(chars[j]) {
                j += 1;
            }
            if j > start && !chars[start].is_ascii_digit() {
                let tag: String = chars[start..j].iter().collect();
                let upper = chars[start].is_ascii_uppercase();
                if !strict_ruby || flagged || quoted || upper {
                    return Some(tag);
                }
            }
            i = j.max(i + 2);
        } else {
            i += 1;
        }
    }
    None
}

pub fn row_opens_string(buffer: &Buffer, idx: usize) -> bool {
    let row = buffer.row(idx);
    if row.hl_state.string != 0 || row.hl_state.inner_string != 0 || row.hl_state.embed != 0 {
        return false;
    }
    let render: Vec<char> = row.render().chars().collect();
    let hl = row.hl();
    let Some(last) = render.iter().rposition(|c| !c.is_whitespace()) else {
        return false;
    };
    let in_string =
        |i: usize| matches!(hl.get(i), Some(Highlight::String) | Some(Highlight::Escape));
    if !in_string(last) {
        return false;
    }
    let mut start = last;
    while start > 0 && in_string(start - 1) {
        start -= 1;
    }
    if let Some(first) = render.iter().position(|c| !c.is_whitespace()) {
        start = start.max(first);
    }
    let is_quote = |c: char| matches!(c, '"' | '\'' | '`');
    let continues = idx > 0 && {
        let prev = buffer.row(idx - 1).hl_state;
        prev.string != 0 || prev.inner_string != 0 || prev.embed != 0
    };
    let quoted = is_quote(render[last]) && (continues || (last > start && is_quote(render[start])));
    let angled = render[start] == '<' && render[last] == '>';
    let closed = quoted || angled;
    !closed
}
