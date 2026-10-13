use super::{HighlightFlags, Syntax};
use crate::model::row::{Highlight, HlState};

pub struct RowHighlight {
    pub hl: Vec<Highlight>,
    pub state: HlState,
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

fn language_by_name(name: &str) -> Option<(u8, &'static Syntax)> {
    let wanted = name.to_ascii_lowercase();
    if wanted.is_empty() {
        return None;
    }
    let database = super::database();
    let index = database
        .iter()
        .position(|s| s.filetype == wanted)
        .or_else(|| {
            database
                .iter()
                .position(|s| s.aliases.iter().any(|alias| *alias == wanted))
        })?;
    Some(((index + 1) as u8, &database[index]))
}

fn language_at(index_plus_one: u8) -> Option<&'static Syntax> {
    super::database().get((index_plus_one as usize).checked_sub(1)?)
}

fn sniff_language(content: &[char], allowed: &[&str]) -> Option<&'static str> {
    let start = content.iter().position(|c| !c.is_whitespace())?;
    let rest = &content[start..];
    if allowed.contains(&"html")
        && rest[0] == '<'
        && rest
            .get(1)
            .is_some_and(|c| c.is_alphabetic() || matches!(c, '!' | '/' | '?'))
    {
        return Some("html");
    }
    if allowed.contains(&"sql") {
        let text: String = rest.iter().collect();
        let word: String = text
            .chars()
            .take_while(|c| c.is_ascii_alphabetic())
            .collect();
        let lower = text.to_ascii_lowercase();
        let uniform_case = word == word.to_ascii_uppercase() || word == word.to_ascii_lowercase();
        let after = text[word.len()..].chars().next();
        let starts_like_sql = matches!(
            word.to_ascii_lowercase().as_str(),
            "select" | "insert" | "update" | "delete" | "create" | "drop" | "alter"
        );
        let has_marker = [
            " from ", " into ", " set ", " values", " table ", " where ", " join ",
        ]
        .iter()
        .any(|marker| lower.contains(marker));
        if starts_like_sql && uniform_case && after.is_some_and(|c| c.is_whitespace()) && has_marker
        {
            return Some("sql");
        }
    }
    None
}

fn matches_closing_tag(render: &[char], at: usize, tag: &str) -> bool {
    if render.get(at) != Some(&'<') || render.get(at + 1) != Some(&'/') {
        return false;
    }
    let mut idx = at + 2;
    for expected in tag.chars() {
        match render.get(idx) {
            Some(c) if c.eq_ignore_ascii_case(&expected) => idx += 1,
            _ => return false,
        }
    }
    !render.get(idx).is_some_and(|c| is_ident_char(*c, "-"))
}

fn region_terminator(
    render: &[char],
    start: usize,
    state: &HlState,
    host: &Syntax,
) -> Option<usize> {
    let n = render.len();
    if state.embed_end == 0 {
        let (_, end) = host
            .multiline_strings
            .get((state.string as usize).checked_sub(1)?)?;
        (start..n).find(|&p| matches_at(render, p, end))
    } else {
        let (tag, _) = host.embed_tags.get(state.embed_end as usize - 1)?;
        (start..n).find(|&p| matches_closing_tag(render, p, tag))
    }
}

fn run_region(
    render: &[char],
    start: usize,
    state: &mut HlState,
    hl: &mut [Highlight],
    host: &Syntax,
) -> usize {
    let n = render.len();
    let Some(inner) = language_at(state.embed) else {
        state.embed = 0;
        state.embed_end = 0;
        return start;
    };
    let terminator = region_terminator(render, start, state, host);
    let stop = terminator.unwrap_or(n);
    let inner_state = HlState {
        comment: state.inner_comment,
        string: state.inner_string,
        ..HlState::default()
    };
    let result = highlight_row_inner(&render[start..stop], inner, inner_state, false);
    for (offset, kind) in result.hl.into_iter().enumerate() {
        hl[start + offset] = kind;
    }
    if terminator.is_some() {
        state.embed = 0;
        state.embed_end = 0;
        state.inner_comment = false;
        state.inner_string = 0;
    } else {
        state.inner_comment = result.state.comment;
        state.inner_string = result.state.string;
    }
    stop
}

fn resolve_undecided_string(render: &[char], host: &Syntax, state: &mut HlState) {
    let Some(first) = render.iter().position(|c| !c.is_whitespace()) else {
        return;
    };
    state.undecided = false;
    let Some((_, end)) = host
        .multiline_strings
        .get((state.string as usize).saturating_sub(1))
    else {
        return;
    };
    let terminator = (0..render.len()).find(|&p| matches_at(render, p, end));
    if terminator.is_some_and(|t| t <= first) {
        return;
    }
    let content_end = terminator.unwrap_or(render.len());
    if let Some(name) = sniff_language(&render[..content_end], host.string_embeds) {
        if let Some((index, _)) = language_by_name(name) {
            state.embed = index;
            state.embed_end = 0;
        }
    }
}

fn start_string_embed(
    render: &[char],
    from: usize,
    state: &mut HlState,
    hl: &mut [Highlight],
    host: &Syntax,
    delimiter: &str,
) -> usize {
    let n = render.len();
    let Some((_, end)) = host
        .multiline_strings
        .get((state.string as usize).saturating_sub(1))
    else {
        return from;
    };
    let terminator = (from..n).find(|&p| matches_at(render, p, end));
    if host.flags.contains(HighlightFlags::FENCES) && matches!(delimiter, "```" | "~~~") {
        if terminator.is_some() {
            return from;
        }
        let info: String = render[from..].iter().collect();
        let word: String = info
            .trim()
            .trim_start_matches(['{', '.'])
            .chars()
            .take_while(|c| c.is_alphanumeric() || matches!(c, '+' | '#' | '-' | '_'))
            .collect();
        fill(hl, from, n, Highlight::String);
        if let Some((index, _)) = language_by_name(&word) {
            state.embed = index;
            state.embed_end = 0;
        }
        return n;
    }
    if host.string_embeds.is_empty() {
        return from;
    }
    let content_end = terminator.unwrap_or(n);
    if render[from..content_end].iter().all(|c| c.is_whitespace()) {
        if terminator.is_none() {
            state.undecided = true;
        }
        return from;
    }
    if let Some(name) = sniff_language(&render[from..content_end], host.string_embeds) {
        if let Some((index, _)) = language_by_name(name) {
            state.embed = index;
            state.embed_end = 0;
            return run_region(render, from, state, hl, host);
        }
    }
    from
}

fn start_tag_embed(
    render: &[char],
    from: usize,
    state: &mut HlState,
    hl: &mut [Highlight],
    host: &Syntax,
    tag: &str,
) -> usize {
    let Some(rule) = host.embed_tags.iter().position(|(name, _)| *name == tag) else {
        return from;
    };
    let Some((index, _)) = language_by_name(host.embed_tags[rule].1) else {
        return from;
    };
    state.embed = index;
    state.embed_end = (rule + 1) as u8;
    run_region(render, from, state, hl, host)
}

fn single_line_string_embed(
    render: &[char],
    open: usize,
    quote: char,
    hl: &mut [Highlight],
    host: &Syntax,
) -> Option<usize> {
    let n = render.len();
    let mut close = open + 1;
    while close < n && render[close] != quote {
        if render[close] == '\\' {
            close += 1;
        }
        close += 1;
    }
    let close = close.min(n);
    let name = sniff_language(&render[open + 1..close], host.string_embeds)?;
    let (_, inner) = language_by_name(name)?;
    hl[open] = Highlight::String;
    let result = highlight_row_inner(&render[open + 1..close], inner, HlState::default(), false);
    for (offset, kind) in result.hl.into_iter().enumerate() {
        hl[open + 1 + offset] = kind;
    }
    if close < n {
        hl[close] = Highlight::String;
        Some(close + 1)
    } else {
        Some(n)
    }
}

pub fn highlight_row(render: &[char], syntax: &Syntax, incoming: HlState) -> RowHighlight {
    highlight_row_inner(render, syntax, incoming, true)
}

fn highlight_row_inner(
    render: &[char],
    syntax: &Syntax,
    incoming: HlState,
    allow_embeds: bool,
) -> RowHighlight {
    let n = render.len();
    let mut hl = vec![Highlight::Normal; n];
    let mut state = incoming;
    let flags = syntax.flags;
    let extra = syntax.ident_chars;
    let markup = flags.contains(HighlightFlags::MARKUP);
    let strings = flags.contains(HighlightFlags::STRINGS);
    let mut open_tag: Option<String> = None;

    if !state.comment && state.string == 0 && state.embed == 0 {
        for (prefix, kind) in syntax.line_rules {
            if matches_at(render, 0, prefix) {
                fill(&mut hl, 0, n, *kind);
                return RowHighlight {
                    hl,
                    state: HlState::default(),
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
        && !state.comment
        && state.string == 0
        && state.embed == 0
    {
        if let Some((start, end)) = key_value_span(render, first_non_space) {
            fill(&mut hl, start, end, Highlight::Type);
            i = end;
        }
    }

    if allow_embeds {
        if state.undecided && state.embed == 0 && state.string != 0 {
            resolve_undecided_string(render, syntax, &mut state);
        }
        if state.embed != 0 {
            i = run_region(render, 0, &mut state, &mut hl, syntax);
        }
    }

    while i < n {
        let c = render[i];
        let prev = if i > 0 { Some(render[i - 1]) } else { None };
        let prev_is_ident = prev.is_some_and(|p| is_ident_char(p, extra));

        if state.string != 0 {
            let (_, end) = syntax.multiline_strings[state.string as usize - 1];
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
                state.string = 0;
                continue;
            }
            hl[i] = Highlight::String;
            i += 1;
            continue;
        }

        if state.comment {
            let mce = syntax.multiline_comment_end;
            if matches_at(render, i, mce) {
                let len = mce.chars().count();
                fill(&mut hl, i, i + len, Highlight::MlComment);
                i += len;
                state.comment = false;
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
            state.comment = true;
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
                state.string = (k + 1) as u8;
                if allow_embeds {
                    i = start_string_embed(render, i, &mut state, &mut hl, syntax, start);
                }
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
                    let tag = open_tag.take();
                    if allow_embeds && c == '>' {
                        if let Some(tag) = tag {
                            i = start_tag_embed(render, i, &mut state, &mut hl, syntax, &tag);
                        }
                    }
                    continue;
                }
            } else if c == '<' && markup_tag_start(render, i) {
                let mut j = i + 1;
                let closing_tag = render.get(j) == Some(&'/');
                if matches!(render.get(j), Some('/' | '!' | '?')) {
                    j += 1;
                }
                let name_start = j;
                while j < n && is_ident_char(render[j], "-:.") {
                    j += 1;
                }
                fill(&mut hl, i, j, Highlight::Keyword1);
                open_tag = if closing_tag {
                    None
                } else {
                    Some(
                        render[name_start..j]
                            .iter()
                            .collect::<String>()
                            .to_ascii_lowercase(),
                    )
                };
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
            if allow_embeds && !syntax.string_embeds.is_empty() {
                if let Some(end) = single_line_string_embed(render, i, c, &mut hl, syntax) {
                    i = end;
                    continue;
                }
            }
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

    RowHighlight { hl, state }
}
