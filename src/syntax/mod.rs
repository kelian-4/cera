pub mod ada_syntax;
pub mod assembly_syntax;
pub mod awk_syntax;
pub mod c_syntax;
pub mod clojure_syntax;
pub mod cmake_syntax;
pub mod cobol_syntax;
pub mod coffeescript_syntax;
pub mod cpp_syntax;
pub mod crystal_syntax;
pub mod csharp_syntax;
pub mod css_syntax;
pub mod d_syntax;
pub mod dart_syntax;
pub mod diff_syntax;
pub mod dockerfile_syntax;
pub mod elixir_syntax;
pub mod elm_syntax;
pub mod engine;
pub mod erlang_syntax;
pub mod fish_syntax;
pub mod fortran_syntax;
pub mod fsharp_syntax;
pub mod go_syntax;
pub mod graphql_syntax;
pub mod groovy_syntax;
pub mod haskell_syntax;
pub mod hcl_syntax;
pub mod html_syntax;
pub mod ini_syntax;
pub mod java_syntax;
pub mod javascript_syntax;
pub mod json_syntax;
pub mod julia_syntax;
pub mod kotlin_syntax;
pub mod latex_syntax;
pub mod lisp_syntax;
pub mod lua_syntax;
pub mod makefile_syntax;
pub mod markdown_syntax;
pub mod matlab_syntax;
pub mod nim_syntax;
pub mod nix_syntax;
pub mod ocaml_syntax;
pub mod pascal_syntax;
pub mod perl_syntax;
pub mod php_syntax;
pub mod powershell_syntax;
pub mod prolog_syntax;
pub mod protobuf_syntax;
pub mod python_syntax;
pub mod r_syntax;
pub mod ruby_syntax;
pub mod rust_syntax;
pub mod scala_syntax;
pub mod scheme_syntax;
pub mod scss_syntax;
pub mod shell_syntax;
pub mod solidity_syntax;
pub mod sql_syntax;
pub mod swift_syntax;
pub mod tcl_syntax;
pub mod toml_syntax;
pub mod typescript_syntax;
pub mod verilog_syntax;
pub mod vhdl_syntax;
pub mod vim_syntax;
pub mod visualbasic_syntax;
pub mod xml_syntax;
pub mod yaml_syntax;
pub mod zig_syntax;

use crate::model::buffer::Buffer;
use crate::model::row::Highlight;

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    pub struct HighlightFlags: u32 {
        const NUMBERS = 1 << 0;
        const STRINGS = 1 << 1;
        const FUNCTIONS = 1 << 2;
        const CAPITALIZED_TYPES = 1 << 3;
        const UPPER_CONSTANTS = 1 << 4;
        const CASE_INSENSITIVE = 1 << 5;
        const PAREN_CALLS = 1 << 6;
        const MARKUP = 1 << 7;
        const KEY_STRINGS = 1 << 8;
        const KEY_VALUES = 1 << 9;
        const CHAR_LITERALS = 1 << 10;
        const TICK_STRINGS = 1 << 11;
        const CSS_VALUES = 1 << 12;
    }
}

pub struct Syntax {
    pub filetype: &'static str,
    pub filematch: &'static [&'static str],
    pub keywords: &'static [&'static str],
    pub types: &'static [&'static str],
    pub constants: &'static [&'static str],
    pub singleline_comment_start: &'static str,
    pub alt_comment_start: &'static str,
    pub multiline_comment_start: &'static str,
    pub multiline_comment_end: &'static str,
    pub multiline_strings: &'static [(&'static str, &'static str)],
    pub quotes: &'static str,
    pub directive_prefix: &'static str,
    pub annotation_prefixes: &'static [&'static str],
    pub variable_prefixes: &'static str,
    pub ident_chars: &'static str,
    pub operators: &'static str,
    pub line_rules: &'static [(&'static str, Highlight)],
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
    let previous_comment = row_idx > 0 && buffer.row(row_idx - 1).hl_open_comment;
    let previous_string = if row_idx > 0 {
        buffer.row(row_idx - 1).hl_open_string
    } else {
        0
    };
    let result = engine::highlight_row(&render, syntax, previous_comment, previous_string);

    for (idx, kind) in result.hl.into_iter().enumerate() {
        if let Some(slot) = buffer.row_mut(row_idx).hl_mut().get_mut(idx) {
            *slot = kind;
        }
    }

    let changed = buffer.row(row_idx).hl_open_comment != result.open_comment
        || buffer.row(row_idx).hl_open_string != result.open_string;
    buffer.row_mut(row_idx).hl_open_comment = result.open_comment;
    buffer.row_mut(row_idx).hl_open_string = result.open_string;
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
        cpp_syntax::cpp_syntax(),
        python_syntax::python_syntax(),
        javascript_syntax::javascript_syntax(),
        rust_syntax::rust_syntax(),
        go_syntax::go_syntax(),
        java_syntax::java_syntax(),
        csharp_syntax::csharp_syntax(),
        ruby_syntax::ruby_syntax(),
        php_syntax::php_syntax(),
        swift_syntax::swift_syntax(),
        kotlin_syntax::kotlin_syntax(),
        shell_syntax::shell_syntax(),
        sql_syntax::sql_syntax(),
        yaml_syntax::yaml_syntax(),
        json_syntax::json_syntax(),
        lua_syntax::lua_syntax(),
        perl_syntax::perl_syntax(),
        haskell_syntax::haskell_syntax(),
        r_syntax::r_syntax(),
        zig_syntax::zig_syntax(),
        html_syntax::html_syntax(),
        css_syntax::css_syntax(),
        markdown_syntax::markdown_syntax(),
        typescript_syntax::typescript_syntax(),
        dart_syntax::dart_syntax(),
        scala_syntax::scala_syntax(),
        julia_syntax::julia_syntax(),
        elixir_syntax::elixir_syntax(),
        erlang_syntax::erlang_syntax(),
        clojure_syntax::clojure_syntax(),
        ocaml_syntax::ocaml_syntax(),
        fsharp_syntax::fsharp_syntax(),
        powershell_syntax::powershell_syntax(),
        groovy_syntax::groovy_syntax(),
        matlab_syntax::matlab_syntax(),
        fortran_syntax::fortran_syntax(),
        pascal_syntax::pascal_syntax(),
        ada_syntax::ada_syntax(),
        cobol_syntax::cobol_syntax(),
        assembly_syntax::assembly_syntax(),
        visualbasic_syntax::visualbasic_syntax(),
        lisp_syntax::lisp_syntax(),
        scheme_syntax::scheme_syntax(),
        nim_syntax::nim_syntax(),
        crystal_syntax::crystal_syntax(),
        d_syntax::d_syntax(),
        tcl_syntax::tcl_syntax(),
        awk_syntax::awk_syntax(),
        solidity_syntax::solidity_syntax(),
        verilog_syntax::verilog_syntax(),
        vhdl_syntax::vhdl_syntax(),
        coffeescript_syntax::coffeescript_syntax(),
        elm_syntax::elm_syntax(),
        vim_syntax::vim_syntax(),
        prolog_syntax::prolog_syntax(),
        fish_syntax::fish_syntax(),
        nix_syntax::nix_syntax(),
        toml_syntax::toml_syntax(),
        ini_syntax::ini_syntax(),
        xml_syntax::xml_syntax(),
        latex_syntax::latex_syntax(),
        diff_syntax::diff_syntax(),
        hcl_syntax::hcl_syntax(),
        protobuf_syntax::protobuf_syntax(),
        graphql_syntax::graphql_syntax(),
        scss_syntax::scss_syntax(),
        makefile_syntax::makefile_syntax(),
        cmake_syntax::cmake_syntax(),
        dockerfile_syntax::dockerfile_syntax(),
    ]
}

#[cfg(test)]
mod language_samples;
#[cfg(test)]
mod tests;
