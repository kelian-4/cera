use super::tokens::{analyze, Tok};
use super::Plan;
use crate::model::buffer::Buffer;

pub struct Profile {
    pub unit: usize,
    pub yaml: bool,
}

fn dash_prefixes(text: &str) -> Vec<usize> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut off = 0usize;
    while off < chars.len() && chars[off] == '-' {
        let mut sp = 0usize;
        while chars.get(off + 1 + sp) == Some(&' ') {
            sp += 1;
        }
        let after = off + 1 + sp;
        if sp == 0 || after >= chars.len() {
            break;
        }
        out.push(1 + sp);
        off = after;
    }
    out
}

fn opens_block_scalar(toks: &[Tok]) -> bool {
    let mut end = toks.len();
    if end > 0 {
        if let Tok::Word(w) = &toks[end - 1] {
            if w.chars().all(|c| c.is_ascii_digit()) {
                end -= 1;
            }
        }
    }
    if end > 0 && matches!(toks[end - 1], Tok::Sym('-') | Tok::Sym('+')) {
        end -= 1;
    }
    if end > 0 {
        if let Tok::Word(w) = &toks[end - 1] {
            if w.chars().all(|c| c.is_ascii_digit()) {
                end -= 1;
            }
        }
    }
    if end < 2 {
        return false;
    }
    matches!(toks[end - 1], Tok::Sym('|') | Tok::Sym('>'))
        && matches!(toks[end - 2], Tok::Sym(':') | Tok::Sym('-'))
}

pub fn plan(buffer: &Buffer, prof: &Profile) -> Option<Vec<Plan>> {
    let n = buffer.num_rows();
    let mut plans = vec![Plan::Keep; n];
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut depth: i32 = 0;
    let mut prev_backslash = false;
    let mut scalar_parent: Option<usize> = None;
    let mut pending: Vec<usize> = Vec::new();
    let mut last_new: Option<usize> = None;

    for i in 0..n {
        let info = analyze(buffer, i, "()[]{}");

        if let Some(pw) = scalar_parent {
            if info.blank {
                plans[i] = Plan::ShiftKeep;
                continue;
            }
            if info.lead > pw {
                plans[i] = Plan::ShiftKeep;
                continue;
            }
            scalar_parent = None;
        }
        if info.start_string {
            plans[i] = Plan::Raw;
            continue;
        }
        if info.start_comment {
            plans[i] = Plan::Shift;
            continue;
        }
        if info.blank {
            plans[i] = Plan::Blank;
            continue;
        }

        let continued = depth > 0 || prev_backslash;
        for t in &info.toks {
            match t {
                Tok::Open(_) => depth += 1,
                Tok::Close(_) => depth -= 1,
                _ => {}
            }
        }
        if depth < 0 {
            depth = 0;
        }
        prev_backslash = info.text.ends_with('\\');
        if continued {
            plans[i] = Plan::Shift;
            continue;
        }

        let w = info.lead;
        if info.comment_only {
            match stack.iter().rev().find(|e| e.0 == w) {
                Some(&(_, nw)) => plans[i] = Plan::Cols(nw),
                None => pending.push(i),
            }
            continue;
        }

        while stack.last().is_some_and(|&(o, _)| o > w) {
            stack.pop();
        }
        let new = match stack.last() {
            Some(&(o, nw)) if o == w => nw,
            Some(&(_, nw)) => {
                let v = nw + prof.unit;
                stack.push((w, v));
                v
            }
            None => {
                stack.push((w, 0));
                0
            }
        };
        plans[i] = Plan::Cols(new);
        last_new = Some(new);
        for j in pending.drain(..) {
            plans[j] = Plan::Cols(new);
        }

        let mut dash_total = 0usize;
        if prof.yaml {
            for step in dash_prefixes(&info.text) {
                dash_total += step;
                stack.push((w + dash_total, new + dash_total));
            }
            if opens_block_scalar(&info.toks) {
                scalar_parent = Some(w + dash_total);
            }
        }
    }

    if let Some(new) = last_new {
        for j in pending {
            plans[j] = Plan::Cols(new);
        }
    }
    Some(plans)
}
