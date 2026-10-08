use super::{HighlightFlags, Syntax};
use crate::model::row::Highlight;

pub struct RowHighlight {
    pub hl: Vec<Highlight>,
    pub open_comment: bool,
    pub open_string: u8,
}

fn matches_at(chars: &[char], i: usize, pat: &str) -> bool {
    if pat.is_empty() {
        return false;
    }
    for (idx, p) in (i..).zip(pat.chars()) {
        if idx >= chars.len() || chars[idx] != p {
            return false;
        }
    }
    true
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_char(c: char, extra: &str) -> bool {
    c.is_alphanumeric() || c == '_' || extra.contains(c)
}

fn fill(hl: &mut [Highlight], from: usize, to: usize, kind: Highlight) {
    for slot in hl.iter_mut().take(to).skip(from) {
        *slot = kind;
    }
}

fn next_non_space(chars: &[char], from: usize) -> Option<char> {
    chars
        .iter()
        .skip(from)
        .find(|c| !c.is_whitespace())
        .copied()
}

fn word_eq(a: &str, b: &str, case_insensitive: bool) -> bool {
    if case_insensitive {
        a.eq_ignore_ascii_case(b)
    } else {
        a == b
    }
}

fn escape_len(chars: &[char], i: usize) -> usize {
    let Some(&next) = chars.get(i + 1) else {
        return 1;
    };
    let is_hex = |idx: usize| chars.get(idx).is_some_and(|c| c.is_ascii_hexdigit());
    let mut len = 2;
    match next {
        'x' => {
            while len < 4 && is_hex(i + len) {
                len += 1;
            }
        }
        'u' | 'U' => {
            if chars.get(i + 2) == Some(&'{') {
                while let Some(&c) = chars.get(i + len) {
                    len += 1;
                    if c == '}' {
                        break;
                    }
                }
            } else {
                let max = if next == 'u' { 6 } else { 10 };
                while len < max && is_hex(i + len) {
                    len += 1;
                }
            }
        }
        '0'..='7' => {
            while len < 4 && chars.get(i + len).is_some_and(|c| ('0'..='7').contains(c)) {
                len += 1;
            }
        }
        _ => {}
    }
    len
}

fn scan_number(chars: &[char], start: usize, css_values: bool) -> usize {
    let n = chars.len();
    let mut j = start;
    let is_digit = |idx: usize| chars.get(idx).is_some_and(|c| c.is_ascii_digit());
    if chars[j] == '0' && matches!(chars.get(j + 1), Some('x' | 'X' | 'b' | 'B' | 'o' | 'O')) {
        j += 2;
        while j < n && (chars[j].is_ascii_hexdigit() || chars[j] == '_') {
            j += 1;
        }
    } else {
        while j < n && (chars[j].is_ascii_digit() || chars[j] == '_') {
            j += 1;
        }
        if chars.get(j) == Some(&'.') && is_digit(j + 1) {
            j += 1;
            while j < n && (chars[j].is_ascii_digit() || chars[j] == '_') {
                j += 1;
            }
        }
        if matches!(chars.get(j), Some('e' | 'E')) {
            let mut k = j + 1;
            if matches!(chars.get(k), Some('+' | '-')) {
                k += 1;
            }
            if is_digit(k) {
                j = k;
                while j < n && chars[j].is_ascii_digit() {
                    j += 1;
                }
            }
        }
    }
    while j < n && (chars[j].is_alphanumeric() || chars[j] == '_') {
        j += 1;
    }
    if css_values && chars.get(j) == Some(&'%') {
        j += 1;
    }
    j
}

fn ident_end(chars: &[char], start: usize, extra: &str) -> usize {
    let n = chars.len();
    let mut j = start;
    while j < n && is_ident_char(chars[j], extra) {
        let c = chars[j];
        if (c == '?' || c == '!') && chars.get(j + 1) == Some(&'=') {
            break;
        }
        j += 1;
    }
    while j > start + 1 && chars[j - 1] == '-' {
        j -= 1;
    }
    j
}

fn classify_word(syntax: &Syntax, word: &str, next: Option<char>, prev: Option<char>) -> Highlight {
    let flags = syntax.flags;
    let ci = flags.contains(HighlightFlags::CASE_INSENSITIVE);
    for kw in syntax.keywords {
        let (body, second) = match kw.strip_suffix('|') {
            Some(b) => (b, true),
            None => (*kw, false),
        };
        if word_eq(body, word, ci) {
            return if second {
                Highlight::Keyword2
            } else {
                Highlight::Keyword1
            };
        }
    }
    if syntax.types.iter().any(|t| word_eq(t, word, ci)) {
        return Highlight::Type;
    }
    if syntax.constants.iter().any(|c| word_eq(c, word, ci)) {
        return Highlight::Constant;
    }
    if flags.contains(HighlightFlags::FUNCTIONS) && next == Some('(') {
        return Highlight::Function;
    }
    if flags.contains(HighlightFlags::PAREN_CALLS) && prev == Some('(') {
        return Highlight::Function;
    }
    let first = word.chars().next().unwrap_or('_');
    if flags.contains(HighlightFlags::CAPITALIZED_TYPES)
        && first.is_uppercase()
        && word.chars().any(|c| c.is_lowercase())
    {
        return Highlight::Type;
    }
    if flags.contains(HighlightFlags::UPPER_CONSTANTS)
        && word.chars().count() >= 2
        && word
            .chars()
            .all(|c| c.is_uppercase() || c.is_ascii_digit() || c == '_')
        && word.chars().any(|c| c.is_uppercase())
    {
        return Highlight::Constant;
    }
    Highlight::Normal
}

fn annotation_end(chars: &[char], i: usize, syntax: &Syntax) -> Option<usize> {
    if syntax.annotation_prefixes.is_empty() || (i > 0 && is_ident_char(chars[i - 1], "")) {
        return None;
    }
    for prefix in syntax.annotation_prefixes {
        if !matches_at(chars, i, prefix) {
            continue;
        }
        let mut j = i + prefix.chars().count();
        if !chars.get(j).is_some_and(|c| is_ident_start(*c)) {
            continue;
        }
        while j < chars.len() {
            let c = chars[j];
            let dotted = c == '.' && chars.get(j + 1).is_some_and(|n| is_ident_start(*n));
            if is_ident_char(c, "") || dotted {
                j += 1;
            } else {
                break;
            }
        }
        return Some(j);
    }
    None
}

fn variable_end(chars: &[char], i: usize, syntax: &Syntax) -> Option<usize> {
    let c = chars[i];
    if !syntax.variable_prefixes.contains(c) {
        return None;
    }
    let prev = if i > 0 { Some(chars[i - 1]) } else { None };
    if prev.is_some_and(|p| is_ident_char(p, syntax.ident_chars)) {
        return None;
    }
    if c == '%'
        && !(prev.is_none() || prev.is_some_and(|p| p.is_whitespace() || p == '(' || p == ','))
    {
        return None;
    }
    let n = chars.len();
    let mut j = i + 1;
    if c == '@' && chars.get(j) == Some(&'@') {
        j += 1;
    }
    match chars.get(j) {
        Some(&next) if is_ident_start(next) => {
            while j < n && is_ident_char(chars[j], "") {
                j += 1;
            }
            Some(j)
        }
        Some('{') => chars[j..]
            .iter()
            .position(|&ch| ch == '}')
            .map(|offset| j + offset + 1),
        Some('(') if c == '$' => {
            let mut k = j + 1;
            while k < n && is_ident_char(chars[k], "") {
                k += 1;
            }
            if k > j + 1 && chars.get(k) == Some(&')') {
                Some(k + 1)
            } else {
                Some(j + 1)
            }
        }
        Some(&next) if c == '$' && (next.is_ascii_digit() || "@?#!*-$<^".contains(next)) => {
            Some(j + 1)
        }
        _ => None,
    }
}

fn char_literal_end(chars: &[char], i: usize) -> Option<usize> {
    if chars.get(i + 1) == Some(&'\\') {
        let limit = (i + 9).min(chars.len());
        return (i + 3..limit).find(|&k| chars[k] == '\'').map(|k| k + 1);
    }
    if chars.get(i + 1).is_some_and(|c| *c != '\'') && chars.get(i + 2) == Some(&'\'') {
        return Some(i + 3);
    }
    None
}

fn hex_color_end(chars: &[char], i: usize) -> Option<usize> {
    let mut j = i + 1;
    while chars.get(j).is_some_and(|c| c.is_ascii_hexdigit()) {
        j += 1;
    }
    let digits = j - i - 1;
    let followed_by_ident = chars.get(j).is_some_and(|c| is_ident_char(*c, "-"));
    if matches!(digits, 3 | 4 | 6 | 8) && !followed_by_ident {
        Some(j)
    } else {
        None
    }
}

fn entity_end(chars: &[char], i: usize) -> Option<usize> {
    let mut j = i + 1;
    if chars.get(j) == Some(&'#') {
        j += 1;
    }
    let start = j;
    while j < chars.len() && j - i <= 10 && chars[j].is_alphanumeric() {
        j += 1;
    }
    if j > start && chars.get(j) == Some(&';') {
        Some(j + 1)
    } else {
        None
    }
}

fn markup_tag_start(chars: &[char], i: usize) -> bool {
    match chars.get(i + 1) {
        Some(c) if c.is_alphabetic() => true,
        Some('/') | Some('!') => chars.get(i + 2).is_some_and(|c| c.is_alphabetic()),
        Some('?') => true,
        _ => false,
    }
}

fn key_value_span(chars: &[char], from: Option<usize>) -> Option<(usize, usize)> {
    let mut start = from?;
    while chars.get(start) == Some(&'-') && chars.get(start + 1) == Some(&' ') {
        start += 2;
        while chars.get(start) == Some(&' ') {
            start += 1;
        }
    }
    let mut end = start;
    while end < chars.len() && (chars[end].is_alphanumeric() || "_.-/@".contains(chars[end])) {
        end += 1;
    }
    if end == start {
        return None;
    }
    let mut j = end;
    while chars.get(j) == Some(&' ') {
        j += 1;
    }
    match chars.get(j) {
        Some(':') if matches!(chars.get(j + 1), None | Some(' ')) => Some((start, end)),
        Some('=') => Some((start, end)),
        _ => None,
    }
}

pub fn highlight_row(
    render: &[char],
    syntax: &Syntax,
    in_comment: bool,
    open_string: u8,
) -> RowHighlight {
    let n = render.len();
    let mut hl = vec![Highlight::Normal; n];
    let mut in_comment = in_comment;
    let mut ml_string = open_string;
    let flags = syntax.flags;
    let extra = syntax.ident_chars;
    let markup = flags.contains(HighlightFlags::MARKUP);
    let strings = flags.contains(HighlightFlags::STRINGS);

    if !in_comment && ml_string == 0 {
        for (prefix, kind) in syntax.line_rules {
            if matches_at(render, 0, prefix) {
                fill(&mut hl, 0, n, *kind);
                return RowHighlight {
                    hl,
                    open_comment: false,
                    open_string: 0,
                };
            }
        }
    }

    let first_non_space = render.iter().position(|c| !c.is_whitespace());
    let mut in_string: Option<(char, usize)> = None;
    let mut in_tag = false;
    let mut i = 0usize;

    let starts_with_directive = !syntax.directive_prefix.is_empty()
        && first_non_space.is_some_and(|at| matches_at(render, at, syntax.directive_prefix));
    if flags.contains(HighlightFlags::KEY_VALUES)
        && !starts_with_directive
        && !in_comment
        && ml_string == 0
    {
        if let Some((start, end)) = key_value_span(render, first_non_space) {
            fill(&mut hl, start, end, Highlight::Type);
            i = end;
        }
    }

    while i < n {
        let c = render[i];
        let prev = if i > 0 { Some(render[i - 1]) } else { None };
        let prev_is_ident = prev.is_some_and(|p| is_ident_char(p, extra));

        if ml_string != 0 {
            let (_, end) = syntax.multiline_strings[ml_string as usize - 1];
            if c == '\\' && i + 1 < n {
                let len = escape_len(render, i);
                fill(&mut hl, i, i + len, Highlight::Escape);
                i += len;
                continue;
            }
            if matches_at(render, i, end) {
                let len = end.chars().count();
                fill(&mut hl, i, i + len, Highlight::String);
                i += len;
                ml_string = 0;
                continue;
            }
            hl[i] = Highlight::String;
            i += 1;
            continue;
        }

        if in_comment {
            let mce = syntax.multiline_comment_end;
            if matches_at(render, i, mce) {
                let len = mce.chars().count();
                fill(&mut hl, i, i + len, Highlight::MlComment);
                i += len;
                in_comment = false;
            } else {
                hl[i] = Highlight::MlComment;
                i += 1;
            }
            continue;
        }

        if let Some((quote, start)) = in_string {
            if c == '\\' && i + 1 < n {
                let len = escape_len(render, i);
                fill(&mut hl, i, i + len, Highlight::Escape);
                i += len;
                continue;
            }
            hl[i] = Highlight::String;
            if c == quote {
                in_string = None;
                if flags.contains(HighlightFlags::KEY_STRINGS)
                    && next_non_space(render, i + 1) == Some(':')
                {
                    fill(&mut hl, start, i + 1, Highlight::Type);
                }
            }
            i += 1;
            continue;
        }

        let mcs = syntax.multiline_comment_start;
        if !mcs.is_empty() && !syntax.multiline_comment_end.is_empty() && matches_at(render, i, mcs)
        {
            let len = mcs.chars().count();
            fill(&mut hl, i, i + len, Highlight::MlComment);
            i += len;
            in_comment = true;
            continue;
        }

        let scs = syntax.singleline_comment_start;
        let shell_hash = scs == "#" && matches!(prev, Some('$') | Some('{'));
        if !shell_hash
            && (matches_at(render, i, scs) || matches_at(render, i, syntax.alt_comment_start))
        {
            fill(&mut hl, i, n, Highlight::Comment);
            break;
        }

        let mut started_string = false;
        for (k, (start, _)) in syntax.multiline_strings.iter().enumerate() {
            let starts_with_ident = start.chars().next().is_some_and(|ch| is_ident_char(ch, ""));
            if matches_at(render, i, start) && !(starts_with_ident && prev_is_ident) {
                let len = start.chars().count();
                fill(&mut hl, i, i + len, Highlight::String);
                i += len;
                ml_string = (k + 1) as u8;
                started_string = true;
                break;
            }
        }
        if started_string {
            continue;
        }

        if markup {
            if in_tag {
                let closes =
                    c == '>' || (matches!(c, '/' | '?') && render.get(i + 1) == Some(&'>'));
                if closes {
                    let len = if c == '>' { 1 } else { 2 };
                    fill(&mut hl, i, i + len, Highlight::Keyword1);
                    i += len;
                    in_tag = false;
                    continue;
                }
            } else if c == '<' && markup_tag_start(render, i) {
                let mut j = i + 1;
                if matches!(render.get(j), Some('/' | '!' | '?')) {
                    j += 1;
                }
                while j < n && is_ident_char(render[j], "-:.") {
                    j += 1;
                }
                fill(&mut hl, i, j, Highlight::Keyword1);
                i = j;
                in_tag = true;
                continue;
            } else {
                if c == '&' {
                    if let Some(end) = entity_end(render, i) {
                        fill(&mut hl, i, end, Highlight::Escape);
                        i = end;
                        continue;
                    }
                }
                i += 1;
                continue;
            }
        }

        if first_non_space == Some(i)
            && !syntax.directive_prefix.is_empty()
            && matches_at(render, i, syntax.directive_prefix)
        {
            let mut j = i + syntax.directive_prefix.chars().count();
            while render.get(j) == Some(&' ') {
                j += 1;
            }
            let word_start = j;
            while j < n && is_ident_char(render[j], "") {
                j += 1;
            }
            if j > word_start {
                fill(&mut hl, i, j, Highlight::Preprocessor);
                let word: String = render[word_start..j].iter().collect();
                i = j;
                if matches!(word.as_str(), "include" | "import" | "include_next") {
                    let mut k = i;
                    while render.get(k) == Some(&' ') {
                        k += 1;
                    }
                    if render.get(k) == Some(&'<') {
                        if let Some(offset) = render[k..].iter().position(|&ch| ch == '>') {
                            fill(&mut hl, k, k + offset + 1, Highlight::String);
                            i = k + offset + 1;
                        }
                    }
                }
                continue;
            }
        }

        if let Some(end) = annotation_end(render, i, syntax) {
            fill(&mut hl, i, end, Highlight::Preprocessor);
            i = end;
            continue;
        }

        if flags.contains(HighlightFlags::CSS_VALUES) && c == '#' {
            if let Some(end) = hex_color_end(render, i) {
                fill(&mut hl, i, end, Highlight::Number);
                i = end;
                continue;
            }
        }

        if let Some(end) = variable_end(render, i, syntax) {
            fill(&mut hl, i, end, Highlight::Variable);
            i = end;
            continue;
        }

        let tick_blocked = c == '\''
            && flags.contains(HighlightFlags::TICK_STRINGS)
            && prev
                .is_some_and(|p| is_ident_char(p, "") || matches!(p, ')' | ']' | '}' | '.' | '\''));
        if strings && syntax.quotes.contains(c) && (!markup || in_tag) && !tick_blocked {
            in_string = Some((c, i));
            hl[i] = Highlight::String;
            i += 1;
            continue;
        }

        if flags.contains(HighlightFlags::CHAR_LITERALS) && c == '\'' {
            if let Some(end) = char_literal_end(render, i) {
                fill(&mut hl, i, end, Highlight::String);
                i = end;
                continue;
            }
        }

        let number_start = c.is_ascii_digit()
            || (c == '.'
                && render.get(i + 1).is_some_and(|d| d.is_ascii_digit())
                && prev != Some('.'));
        if flags.contains(HighlightFlags::NUMBERS) && number_start && !prev_is_ident {
            let end = scan_number(render, i, flags.contains(HighlightFlags::CSS_VALUES));
            fill(&mut hl, i, end, Highlight::Number);
            i = end;
            continue;
        }

        if is_ident_start(c) && !prev_is_ident {
            let end = ident_end(render, i, extra);
            let word: String = render[i..end].iter().collect();
            let next = next_non_space(render, end);
            let kind = if in_tag {
                if next == Some('=') {
                    Highlight::Type
                } else {
                    Highlight::Normal
                }
            } else {
                classify_word(syntax, &word, next, prev)
            };
            fill(&mut hl, i, end, kind);
            i = end;
            continue;
        }

        if syntax.operators.contains(c) {
            hl[i] = Highlight::Operator;
        }
        i += 1;
    }

    RowHighlight {
        hl,
        open_comment: in_comment,
        open_string: ml_string,
    }
}
