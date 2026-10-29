use super::tokens::{analyze, heredoc_tag, pairs, LineInfo, Tok};
use super::Plan;
use crate::model::buffer::Buffer;

pub struct KwProfile {
    pub line_openers: &'static [&'static str],
    pub any_openers: &'static [&'static str],
    pub closers: &'static [&'static str],
    pub middles: &'static [&'static str],
    pub no_do_after: &'static [&'static str],
    pub index_end: bool,
    pub command_position: bool,
    pub case_arms: bool,
}

pub struct Profile {
    pub brackets: &'static str,
    pub switch_labels: bool,
    pub braceless: bool,
    pub continuation: bool,
    pub preproc: bool,
    pub heredoc: bool,
    pub backslash: bool,
    pub guard_if: bool,
    pub preserve: bool,
    pub kw: Option<&'static KwProfile>,
}

#[derive(PartialEq)]
enum Kind {
    Br(char),
    Kw,
    Case,
}

struct Ent {
    inline: bool,
    kind: Kind,
    level: usize,
    in_case: bool,
    switch: bool,
    body: bool,
}

fn is_kw_closer(kw: &KwProfile, w: &str) -> bool {
    kw.closers.contains(&w)
}

fn is_kw_middle(kw: &KwProfile, w: &str) -> bool {
    kw.middles.contains(&w)
}

fn kw_ok(kw: &KwProfile, toks: &[Tok], k: usize, top: Option<&Ent>, closer: bool) -> bool {
    if k > 0 && toks[k - 1] == Tok::Sym('.') {
        return false;
    }
    if !kw.command_position
        && toks.get(k + 1) == Some(&Tok::Sym(':'))
        && toks.get(k + 2) != Some(&Tok::Sym(':'))
    {
        return false;
    }
    if kw.command_position {
        let at_command = k == 0
            || match &toks[k - 1] {
                Tok::Sym(c) => matches!(c, ';' | '&' | '|' | '(' | '!'),
                Tok::Open('{') => true,
                Tok::Word(w) => matches!(w.as_str(), "then" | "do" | "else" | "elif"),
                _ => false,
            };
        if !at_command {
            return false;
        }
        if toks.get(k + 1) == Some(&Tok::Sym('=')) {
            return false;
        }
    }
    if closer && kw.index_end {
        if let Some(e) = top {
            if e.kind == Kind::Br('[') {
                return false;
            }
        }
    }
    true
}

fn is_label_line(toks: &[Tok]) -> bool {
    match toks.first() {
        Some(Tok::Word(w)) if w == "case" => true,
        Some(Tok::Word(w)) if w == "default" => toks.get(1) == Some(&Tok::Sym(':')),
        _ => false,
    }
}

fn is_header(toks: &[Tok], nclose: usize) -> bool {
    let t = &toks[nclose..];
    let Some(Tok::Word(w)) = t.first() else {
        return false;
    };
    if !matches!(
        w.as_str(),
        "if" | "for" | "while" | "foreach" | "else" | "do" | "elseif" | "unless"
    ) {
        return false;
    }
    if nclose > 0 && w == "while" {
        return false;
    }
    let mut depth = 0i32;
    for x in t {
        match x {
            Tok::Open(_) => depth += 1,
            Tok::Close(_) => depth -= 1,
            _ => {}
        }
    }
    if depth != 0 {
        return false;
    }
    match t.last() {
        Some(Tok::Close(')')) => true,
        Some(Tok::Word(l)) => l == "else" || l == "do",
        _ => false,
    }
}

fn continues_previous(tail: &[Tok]) -> bool {
    let Some(last) = tail.last() else {
        return false;
    };
    let prev = if tail.len() >= 2 {
        tail.get(tail.len() - 2)
    } else {
        None
    };
    match last {
        Tok::Sym(c) if "=+-*/%&|^?.".contains(*c) => {
            !((*c == '+' || *c == '-') && prev == Some(&Tok::Sym(*c)))
        }
        Tok::Sym('>') => matches!(prev, Some(Tok::Sym('=')) | Some(Tok::Sym('-'))),
        _ => false,
    }
}

fn starts_continuation(toks: &[Tok], tail: &[Tok], rust: bool) -> bool {
    let Some(last) = tail.last() else {
        return false;
    };
    if matches!(
        last,
        Tok::Open(_) | Tok::Sym(',') | Tok::Sym(';') | Tok::Close('}')
    ) {
        return false;
    }
    let second = toks.get(1);
    match toks.first() {
        Some(Tok::Sym('.')) => second != Some(&Tok::Sym('.')),
        Some(Tok::Sym('?')) => true,
        Some(Tok::Sym('|')) => !rust || second == Some(&Tok::Sym('|')),
        Some(Tok::Sym('&')) => second == Some(&Tok::Sym('&')),
        Some(Tok::Sym('-')) => second == Some(&Tok::Sym('>')),
        Some(Tok::Sym('+')) => second != Some(&Tok::Sym('+')),
        Some(Tok::Sym(':')) => second != Some(&Tok::Sym(':')),
        Some(Tok::Sym('<')) => second == Some(&Tok::Sym('<')),
        _ => false,
    }
}

fn is_chain_start(toks: &[Tok]) -> bool {
    matches!(toks.first(), Some(Tok::Sym('.')))
        || (toks.first() == Some(&Tok::Sym('-')) && toks.get(1) == Some(&Tok::Sym('>')))
}

fn guard_context(tail: &[Tok]) -> bool {
    match tail.last() {
        None => false,
        Some(Tok::Open(_)) | Some(Tok::Sym(',')) | Some(Tok::Sym(';')) | Some(Tok::Close('}')) => {
            false
        }
        Some(Tok::Word(w)) => w != "else",
        Some(_) => true,
    }
}

fn ends_arm(toks: &[Tok]) -> bool {
    toks.len() >= 2
        && toks[toks.len() - 1] == Tok::Sym(';')
        && toks[toks.len() - 2] == Tok::Sym(';')
}

pub fn plan(buffer: &Buffer, p: &Profile, unit: usize) -> Option<Vec<Plan>> {
    let n = buffer.num_rows();
    let mut plans = Vec::with_capacity(n);
    let mut stack: Vec<Ent> = Vec::new();
    let mut heredoc: Option<String> = None;
    let mut tail: Vec<Tok> = Vec::new();
    let mut prev_switch = false;
    let mut prev_backslash = false;
    let mut pending = 0usize;
    let mut prev_level = 0usize;
    let mut cont_head = 0usize;
    let mut chain_at: Option<usize> = None;
    let mut prev_op_start = false;
    let mut prev_only_close = false;
    let mut prev_guard = false;

    for i in 0..n {
        let info: LineInfo = analyze(buffer, i, p.brackets);

        if let Some(tag) = &heredoc {
            if info.text == *tag {
                heredoc = None;
            }
            plans.push(Plan::Raw);
            continue;
        }
        if info.start_string {
            plans.push(Plan::Raw);
            continue;
        }
        if info.start_comment {
            plans.push(Plan::Shift);
            continue;
        }
        if info.blank {
            plans.push(Plan::Blank);
            continue;
        }
        if p.preproc && info.text.starts_with('#') {
            plans.push(if prev_backslash {
                Plan::Shift
            } else {
                Plan::Cols(0)
            });
            prev_backslash = info.text.ends_with('\\');
            continue;
        }

        let toks = &info.toks;
        let has_code = !toks.is_empty();
        let nclose = toks
            .iter()
            .take_while(|t| matches!(t, Tok::Close(_)))
            .count();
        let first_word: Option<&str> = match toks.first() {
            Some(Tok::Word(w)) => Some(w.as_str()),
            _ => None,
        };

        let level: usize;
        let mut is_label = false;
        let mut op_start = false;
        let mut guard = false;
        let mut shift_row = false;
        let kw_first = match (p.kw, first_word) {
            (Some(kw), Some(w)) if kw_ok(kw, toks, 0, stack.last(), false) => {
                if is_kw_closer(kw, w) || is_kw_middle(kw, w) {
                    Some(w)
                } else {
                    None
                }
            }
            _ => None,
        };

        if nclose > 0 {
            if stack.len() < nclose {
                return None;
            }
            for (k, t) in toks.iter().take(nclose).enumerate() {
                let Tok::Close(c) = t else {
                    return None;
                };
                match &stack[stack.len() - 1 - k].kind {
                    Kind::Br(o) if pairs(*o, *c) => {}
                    _ => return None,
                }
            }
            level = stack[stack.len() - nclose].level;
        } else if kw_first.is_some() {
            match stack.last() {
                Some(e) if e.kind == Kind::Kw || e.kind == Kind::Case => level = e.level,
                _ => return None,
            }
        } else {
            let mut l = stack.last().map_or(0, |e| e.level + 1);
            if let Some(e) = stack.last_mut() {
                match e.kind {
                    Kind::Br('{') if p.switch_labels && e.switch => {
                        if is_label_line(toks) {
                            l = e.level + 1;
                            e.in_case = true;
                            is_label = true;
                        } else if e.in_case {
                            l += 1;
                        }
                    }
                    Kind::Case => {
                        l = if e.body { e.level + 2 } else { e.level + 1 };
                    }
                    _ => {}
                }
            }
            if p.braceless && pending > 0 && !is_label {
                let starts_brace = matches!(toks.first(), Some(Tok::Open('{')));
                l += if starts_brace { pending - 1 } else { pending };
            } else if p.continuation && has_code && !is_label {
                let chain = is_chain_start(toks);
                let top_is_brace = stack.last().is_none_or(|e| matches!(e.kind, Kind::Br('{')));
                let top_is_group = stack
                    .last()
                    .is_some_and(|e| matches!(e.kind, Kind::Br('(') | Kind::Br('[')));
                let starts_block = matches!(toks.first(), Some(Tok::Open('{')));
                let is_guard = p.guard_if
                    && top_is_brace
                    && matches!(toks.first(), Some(Tok::Word(w)) if w == "if")
                    && guard_context(&tail);
                if is_guard {
                    l += 1;
                    guard = true;
                    chain_at = None;
                } else if (top_is_brace || top_is_group)
                    && starts_continuation(toks, &tail, p.guard_if && top_is_brace)
                {
                    if chain && prev_only_close {
                        l = prev_level;
                        chain_at = Some(l);
                    } else if chain && chain_at.is_some() {
                        l = chain_at.unwrap_or(l);
                    } else if chain && prev_op_start {
                        l = prev_level + 1;
                        chain_at = Some(l);
                    } else if !chain && prev_guard {
                        l = prev_level + 1;
                        chain_at = None;
                    } else {
                        if top_is_group {
                            if !stack.last().is_some_and(|e| e.inline) {
                                l += 1;
                            }
                        } else {
                            l += cont_head + 1;
                        }
                        chain_at = if chain { Some(l) } else { None };
                    }
                    op_start = !chain;
                    shift_row = p.preserve;
                } else if top_is_brace && continues_previous(&tail) && !starts_block {
                    l += 1;
                    cont_head = 1;
                    chain_at = None;
                    shift_row = p.preserve;
                } else {
                    cont_head = 0;
                    chain_at = None;
                }
            }
            if p.preserve
                && has_code
                && stack
                    .last()
                    .is_some_and(|e| e.inline && matches!(e.kind, Kind::Br('(') | Kind::Br('[')))
            {
                shift_row = true;
            }
            level = l;
        }

        let forced_shift = (p.backslash && prev_backslash) || shift_row;
        plans.push(if forced_shift {
            Plan::Shift
        } else {
            Plan::Cols(level * unit)
        });

        if let Some(e) = stack.last_mut() {
            if e.kind == Kind::Case && !e.body && !ends_arm(toks) && kw_first.is_none() {
                e.body = true;
            }
        }

        let line_has_switch = toks
            .iter()
            .any(|t| matches!(t, Tok::Word(w) if w == "switch" || w == "select"));
        let first_closer_word = toks.first().is_some_and(|t| matches!(t, Tok::Word(_)));

        for (k, t) in toks.iter().enumerate() {
            match t {
                Tok::Open(c) => stack.push(Ent {
                    inline: k + 1 < toks.len(),
                    kind: Kind::Br(*c),
                    level,
                    in_case: false,
                    switch: *c == '{' && (line_has_switch || (k == 0 && prev_switch)),
                    body: false,
                }),
                Tok::Close(c) => match stack.pop() {
                    Some(Ent {
                        kind: Kind::Br(o), ..
                    }) if pairs(o, *c) => {}
                    _ => return None,
                },
                Tok::Word(w) => {
                    let Some(kw) = p.kw else {
                        continue;
                    };
                    if is_kw_closer(kw, w) {
                        if !kw_ok(kw, toks, k, stack.last(), true) {
                            continue;
                        }
                        match stack.pop() {
                            Some(Ent {
                                kind: Kind::Kw | Kind::Case,
                                ..
                            }) => {}
                            _ => return None,
                        }
                    } else if kw.any_openers.contains(&w.as_str()) {
                        if !kw_ok(kw, toks, k, stack.last(), false) {
                            continue;
                        }
                        if w == "do" {
                            if let Some(fw) = first_word {
                                if kw.no_do_after.contains(&fw) {
                                    continue;
                                }
                            }
                        }
                        stack.push(Ent {
                            inline: false,
                            kind: Kind::Kw,
                            level,
                            in_case: false,
                            switch: false,
                            body: false,
                        });
                    } else if k == 0
                        && first_closer_word
                        && kw.line_openers.contains(&w.as_str())
                        && kw_ok(kw, toks, k, stack.last(), false)
                    {
                        stack.push(Ent {
                            inline: false,
                            kind: if kw.case_arms && w == "case" {
                                Kind::Case
                            } else {
                                Kind::Kw
                            },
                            level,
                            in_case: false,
                            switch: false,
                            body: false,
                        });
                    }
                }
                _ => {}
            }
        }

        if ends_arm(toks) {
            if let Some(e) = stack.last_mut() {
                if e.kind == Kind::Case {
                    e.body = false;
                }
            }
        }

        if has_code {
            if p.braceless {
                pending = if is_header(toks, nclose) {
                    pending + 1
                } else {
                    0
                };
            }
            let from = toks.len().saturating_sub(2);
            tail = toks[from..].to_vec();
            prev_switch = line_has_switch;
            prev_level = level;
            prev_only_close = toks
                .iter()
                .all(|t| matches!(t, Tok::Close(_) | Tok::Sym('?') | Tok::Sym(';')));
            prev_op_start = op_start;
            prev_guard = guard;
        }
        prev_backslash = info.text.ends_with('\\');

        if p.heredoc {
            let strict = p.kw.is_some_and(|k| !k.command_position);
            if let Some(tag) = heredoc_tag(&info, !strict, strict) {
                heredoc = Some(tag);
            }
        }
    }

    if !stack.is_empty() || heredoc.is_some() {
        return None;
    }
    Some(plans)
}
