pub mod markup;
pub mod offside;
pub mod structure;
pub mod tokens;

use crate::model::buffer::Buffer;
use structure::{KwProfile, Profile};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plan {
    Raw,
    Keep,
    Blank,
    Shift,
    ShiftKeep,
    Cols(usize),
}

pub struct OutRow {
    pub text: String,
    pub src: usize,
    pub old_lead: usize,
    pub new_lead: usize,
}

pub struct Formatted {
    pub rows: Vec<OutRow>,
    pub reindented: bool,
}

const C_LIKE: Profile = Profile {
    brackets: "()[]{}",
    switch_labels: true,
    braceless: true,
    continuation: true,
    preproc: true,
    heredoc: false,
    backslash: false,
    guard_if: false,
    preserve: false,
    kw: None,
};
const CPP_LIKE: Profile = Profile {
    preserve: true,
    ..C_LIKE
};
const JAVA_LIKE: Profile = Profile {
    preproc: false,
    ..C_LIKE
};
const GO_LIKE: Profile = Profile {
    braceless: false,
    preproc: false,
    ..C_LIKE
};
const RUST_LIKE: Profile = Profile {
    guard_if: true,
    switch_labels: false,
    braceless: false,
    preproc: false,
    ..C_LIKE
};
const DATA_LIKE: Profile = Profile {
    switch_labels: false,
    braceless: false,
    continuation: false,
    preproc: false,
    ..C_LIKE
};

static RUBY_KW: KwProfile = KwProfile {
    line_openers: &[
        "def", "class", "module", "if", "unless", "while", "until", "for", "case", "begin",
    ],
    any_openers: &["do"],
    closers: &["end"],
    middles: &["else", "elsif", "when", "in", "rescue", "ensure"],
    no_do_after: &["while", "until", "for"],
    index_end: false,
    command_position: false,
    case_arms: false,
};
static CRYSTAL_KW: KwProfile = KwProfile {
    line_openers: &[
        "def", "class", "module", "if", "unless", "while", "until", "case", "begin", "macro",
        "struct", "enum", "lib",
    ],
    ..RUBY_KW
};
static LUA_KW: KwProfile = KwProfile {
    line_openers: &["if"],
    any_openers: &["function", "do", "repeat"],
    closers: &["end", "until"],
    middles: &["else", "elseif"],
    no_do_after: &[],
    index_end: false,
    command_position: false,
    case_arms: false,
};
static JULIA_KW: KwProfile = KwProfile {
    line_openers: &[
        "function", "if", "for", "while", "begin", "let", "struct", "mutable", "module", "macro",
        "try", "quote",
    ],
    any_openers: &["do"],
    closers: &["end"],
    middles: &["else", "elseif", "catch", "finally"],
    no_do_after: &[],
    index_end: true,
    command_position: false,
    case_arms: false,
};
static SHELL_KW: KwProfile = KwProfile {
    line_openers: &["if", "case"],
    any_openers: &["do"],
    closers: &["fi", "done", "esac"],
    middles: &["else", "elif", "then"],
    no_do_after: &[],
    index_end: false,
    command_position: true,
    case_arms: true,
};

const RUBY_LIKE: Profile = Profile {
    brackets: "()[]{}",
    switch_labels: false,
    braceless: false,
    continuation: false,
    preproc: false,
    heredoc: true,
    backslash: false,
    guard_if: false,
    preserve: false,
    kw: Some(&RUBY_KW),
};
const CRYSTAL_LIKE: Profile = Profile {
    kw: Some(&CRYSTAL_KW),
    ..RUBY_LIKE
};
const LUA_LIKE: Profile = Profile {
    heredoc: false,
    kw: Some(&LUA_KW),
    ..RUBY_LIKE
};
const JULIA_LIKE: Profile = Profile {
    heredoc: false,
    kw: Some(&JULIA_KW),
    ..RUBY_LIKE
};
const SHELL_LIKE: Profile = Profile {
    brackets: "{}",
    backslash: true,
    kw: Some(&SHELL_KW),
    ..RUBY_LIKE
};

enum Policy {
    Off,
    Hygiene,
    Structure(&'static Profile),
    Offside(offside::Profile),
    Markup { html: bool },
}

struct Rules {
    policy: Policy,
    trim: bool,
    max_blank: usize,
    force_tabs: Option<bool>,
}

fn rules_for(buffer: &Buffer, filetype: &str) -> Rules {
    let mut r = Rules {
        policy: Policy::Hygiene,
        trim: true,
        max_blank: 1,
        force_tabs: None,
    };
    match filetype {
        "" | "markdown" | "diff" | "latex" => r.policy = Policy::Off,
        "html" => r.policy = Policy::Markup { html: true },
        "xml" => r.policy = Policy::Markup { html: false },
        "c" | "cpp" => r.policy = Policy::Structure(&CPP_LIKE),
        "csharp" => r.policy = Policy::Structure(&C_LIKE),
        "javascript" | "typescript" => {
            if !looks_like_jsx(buffer) {
                r.policy = Policy::Structure(&JAVA_LIKE);
            }
        }
        "php" => {
            if !looks_like_html(buffer) {
                r.policy = Policy::Structure(&JAVA_LIKE);
            }
        }
        "java" | "kotlin" | "dart" | "d" | "groovy" | "solidity" | "zig" => {
            r.policy = Policy::Structure(&JAVA_LIKE)
        }
        "go" => {
            r.policy = Policy::Structure(&GO_LIKE);
            r.force_tabs = Some(true);
        }
        "rust" => r.policy = Policy::Structure(&RUST_LIKE),
        "css" | "scss" | "json" | "protobuf" | "graphql" | "hcl" => {
            r.policy = Policy::Structure(&DATA_LIKE)
        }
        "ruby" => r.policy = Policy::Structure(&RUBY_LIKE),
        "crystal" => r.policy = Policy::Structure(&CRYSTAL_LIKE),
        "lua" => r.policy = Policy::Structure(&LUA_LIKE),
        "julia" => r.policy = Policy::Structure(&JULIA_LIKE),
        "shell" => r.policy = Policy::Structure(&SHELL_LIKE),
        "python" => {
            r.policy = Policy::Offside(offside::Profile {
                unit: 4,
                yaml: false,
            });
            r.max_blank = 2;
            r.force_tabs = Some(false);
        }
        "yaml" => {
            r.policy = Policy::Offside(offside::Profile {
                unit: 2,
                yaml: true,
            });
            r.force_tabs = Some(false);
        }
        "nim" | "coffeescript" => {
            r.policy = Policy::Offside(offside::Profile {
                unit: 2,
                yaml: false,
            });
            r.force_tabs = Some(false);
        }
        _ => {}
    }
    r
}

fn looks_like_jsx(buffer: &Buffer) -> bool {
    buffer.rows().iter().any(|row| {
        let t = row.chars();
        t.contains("</") || t.contains("/>")
    })
}

fn looks_like_html(buffer: &Buffer) -> bool {
    buffer.rows().iter().any(|row| {
        let t = row.chars();
        t.contains("?>")
            || t.trim_start()
                .strip_prefix('<')
                .and_then(|r| r.chars().next())
                .is_some_and(|c| c.is_ascii_alphabetic() || c == '/')
    })
}

fn uses_tabs(buffer: &Buffer) -> bool {
    let mut tabs = 0usize;
    let mut spaces = 0usize;
    for row in buffer.rows() {
        let c = row.chars();
        if c.starts_with('\t') {
            tabs += 1;
        } else if c.starts_with(' ') && !c.trim().is_empty() {
            spaces += 1;
        }
    }
    tabs > spaces
}

fn indent_string(cols: usize, tabs: bool, tab_stop: usize) -> String {
    if tabs {
        let ts = tab_stop.max(1);
        format!("{}{}", "\t".repeat(cols / ts), " ".repeat(cols % ts))
    } else {
        " ".repeat(cols)
    }
}

fn lead_cols(buffer: &Buffer, idx: usize) -> usize {
    buffer
        .row(idx)
        .render()
        .chars()
        .take_while(|c| *c == ' ')
        .count()
}

fn lead_chars(s: &str) -> usize {
    s.chars().take_while(|c| *c == ' ' || *c == '\t').count()
}

fn row_ends_in_string(buffer: &Buffer, idx: usize) -> bool {
    let s = buffer.row(idx).hl_state;
    s.string != 0 || s.inner_string != 0 || s.embed != 0
}

fn signature(texts: impl Iterator<Item = String>) -> String {
    texts
        .flat_map(|t| t.chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>())
        .collect()
}

fn hygiene_plans(buffer: &Buffer) -> Vec<Plan> {
    (0..buffer.num_rows())
        .map(|i| {
            let in_string = i > 0 && {
                let s = buffer.row(i - 1).hl_state;
                s.string != 0 || s.embed != 0 || s.inner_string != 0
            };
            if in_string {
                Plan::Raw
            } else if buffer.row(i).chars().trim().is_empty() {
                Plan::Blank
            } else {
                Plan::Keep
            }
        })
        .collect()
}

pub fn format_buffer(buffer: &Buffer, filetype: &str, tab_stop: usize) -> Option<Formatted> {
    let rules = rules_for(buffer, filetype);
    if matches!(rules.policy, Policy::Off) {
        return None;
    }

    let skip_guard = matches!(rules.policy, Policy::Markup { .. }) || filetype == "yaml";
    if !skip_guard && (0..buffer.num_rows()).any(|i| tokens::row_opens_string(buffer, i)) {
        return None;
    }

    let tabs = rules.force_tabs.unwrap_or_else(|| uses_tabs(buffer));
    let unit_cols = if tabs {
        tab_stop.max(1)
    } else {
        let detected = crate::view::indent_guides::detect_unit(buffer, filetype, tab_stop);
        if matches!(detected, 2 | 4 | 8) {
            detected
        } else {
            crate::view::indent_guides::default_unit(filetype)
        }
    };

    let planned = match &rules.policy {
        Policy::Structure(p) => structure::plan(buffer, p, unit_cols),
        Policy::Offside(p) => offside::plan(buffer, p),
        Policy::Markup { html } => Some(markup::plan(buffer, *html, unit_cols)?),
        _ => None,
    };
    let reindented = planned.is_some();
    let plans = planned.unwrap_or_else(|| hygiene_plans(buffer));

    let mut rows: Vec<(OutRow, bool)> = Vec::new();
    let mut delta: i64 = 0;
    let mut blank_run = 0usize;

    for (i, plan) in plans.iter().enumerate() {
        let chars = buffer.row(i).chars();
        let old_lead_cols = lead_cols(buffer, i) as i64;
        let old_lead = lead_chars(chars);
        let content = chars.trim_start_matches([' ', '\t']);
        let trim_end = rules.trim && !row_ends_in_string(buffer, i);
        let body = if trim_end {
            content.trim_end_matches([' ', '\t'])
        } else {
            content
        };

        let (text, collapsible) = match plan {
            Plan::Raw => (chars.to_string(), false),
            Plan::Blank => (String::new(), true),
            Plan::Keep => {
                delta = 0;
                if body.is_empty() {
                    (String::new(), true)
                } else {
                    let lead: String = chars.chars().take(old_lead).collect();
                    (format!("{lead}{body}"), false)
                }
            }
            Plan::Cols(c) => {
                delta = *c as i64 - old_lead_cols;
                (
                    format!("{}{}", indent_string(*c, tabs, tab_stop), body),
                    false,
                )
            }
            Plan::Shift | Plan::ShiftKeep => {
                let keep_ws = matches!(plan, Plan::ShiftKeep);
                if content.trim().is_empty() {
                    (String::new(), false)
                } else {
                    let cols = (old_lead_cols + delta).max(0) as usize;
                    let b = if keep_ws { content } else { body };
                    (
                        format!("{}{}", indent_string(cols, tabs, tab_stop), b),
                        false,
                    )
                }
            }
        };

        if collapsible {
            blank_run += 1;
            if rows.is_empty() || blank_run > rules.max_blank {
                continue;
            }
        } else {
            blank_run = 0;
        }
        let new_lead = lead_chars(&text);
        rows.push((
            OutRow {
                text,
                src: i,
                old_lead,
                new_lead,
            },
            collapsible,
        ));
    }

    while rows
        .last()
        .is_some_and(|(r, _)| r.text.is_empty() && plans[r.src] != Plan::Raw)
    {
        rows.pop();
    }
    let mut rows: Vec<OutRow> = rows.into_iter().map(|(r, _)| r).collect();
    if rows.is_empty() {
        rows.push(OutRow {
            text: String::new(),
            src: 0,
            old_lead: 0,
            new_lead: 0,
        });
    }

    let before = signature(buffer.rows().iter().map(|r| r.chars().to_string()));
    let after = signature(rows.iter().map(|r| r.text.clone()));
    if before != after {
        return None;
    }

    Some(Formatted { rows, reindented })
}

#[cfg(test)]
mod tests;
