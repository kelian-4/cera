use super::*;
use crate::model::row::Highlight;

pub struct Sample {
    pub file: &'static str,
    pub filetype: &'static str,
    pub lines: &'static [&'static str],
    pub expect: &'static [(usize, &'static str, Highlight)],
}

pub const SAMPLES: &[Sample] = &[
    Sample {
        file: "main.c",
        filetype: "c",
        lines: &["#include <stdio.h>", "int main(void) { return 0; } // c"],
        expect: &[
            (0, "#include", Highlight::Preprocessor),
            (0, "<stdio.h>", Highlight::String),
            (1, "int", Highlight::Type),
            (1, "main", Highlight::Function),
            (1, "void", Highlight::Type),
            (1, "return", Highlight::Keyword1),
            (1, "0", Highlight::Number),
            (1, "// c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.cpp",
        filetype: "cpp",
        lines: &["#include <vector>", "std::vector<int> v = {1, 2.5f}; // c"],
        expect: &[
            (0, "#include", Highlight::Preprocessor),
            (1, "vector", Highlight::Type),
            (1, "int", Highlight::Type),
            (1, "1", Highlight::Number),
            (1, "2.5f", Highlight::Number),
            (1, "// c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.py",
        filetype: "python",
        lines: &[
            "@app.route(\"/x\")",
            "def calc(self, n: int = 0x1F):",
            "    \"\"\"doc",
            "    more\"\"\"",
            "    return None # c",
        ],
        expect: &[
            (0, "@app.route", Highlight::Preprocessor),
            (0, "\"/x\"", Highlight::String),
            (1, "def", Highlight::Keyword1),
            (1, "calc", Highlight::Function),
            (1, "int", Highlight::Type),
            (1, "0x1F", Highlight::Number),
            (2, "\"\"\"doc", Highlight::String),
            (3, "more\"\"\"", Highlight::String),
            (4, "None", Highlight::Constant),
            (4, "# c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.js",
        filetype: "javascript",
        lines: &[
            "const f = (x) => `a${x}`; // c",
            "class Foo extends Bar { }",
        ],
        expect: &[
            (0, "const", Highlight::Keyword2),
            (0, "`a${x}`", Highlight::String),
            (0, "// c", Highlight::Comment),
            (1, "class", Highlight::Keyword1),
            (1, "Foo", Highlight::Type),
        ],
    },
    Sample {
        file: "a.rs",
        filetype: "rust",
        lines: &[
            "#[derive(Debug)]",
            "fn main() { let s: &str = \"a\\n\"; let c = 'x'; Some(1u8) }",
            "fn g<'a>(x: &'a str) {}",
        ],
        expect: &[
            (0, "#[derive", Highlight::Preprocessor),
            (1, "fn", Highlight::Keyword1),
            (1, "main", Highlight::Function),
            (1, "str", Highlight::Type),
            (1, "\\n", Highlight::Escape),
            (1, "'x'", Highlight::String),
            (1, "Some", Highlight::Constant),
            (1, "1u8", Highlight::Number),
            (2, "'a", Highlight::Normal),
        ],
    },
    Sample {
        file: "a.go",
        filetype: "go",
        lines: &[
            "func main() {",
            "    fmt.Println(\"hi\", 3.5)",
            "    var x int = nil }",
        ],
        expect: &[
            (0, "func", Highlight::Keyword1),
            (0, "main", Highlight::Function),
            (1, "Println", Highlight::Keyword2),
            (1, "3.5", Highlight::Number),
            (2, "int", Highlight::Type),
            (2, "nil", Highlight::Constant),
        ],
    },
    Sample {
        file: "A.java",
        filetype: "java",
        lines: &["@Override public String f(int x) { return null; }"],
        expect: &[
            (0, "@Override", Highlight::Preprocessor),
            (0, "String", Highlight::Type),
            (0, "f", Highlight::Function),
            (0, "int", Highlight::Type),
            (0, "null", Highlight::Constant),
        ],
    },
    Sample {
        file: "A.cs",
        filetype: "csharp",
        lines: &[
            "#region x",
            "public static void Main(string[] a) { var s = 1; }",
        ],
        expect: &[
            (0, "#region", Highlight::Preprocessor),
            (1, "void", Highlight::Type),
            (1, "Main", Highlight::Function),
            (1, "string", Highlight::Type),
            (1, "1", Highlight::Number),
        ],
    },
    Sample {
        file: "a.rb",
        filetype: "ruby",
        lines: &["def foo?(x) # c", "  @a = 1 if x", "end"],
        expect: &[
            (0, "def", Highlight::Keyword1),
            (0, "foo?", Highlight::Function),
            (0, "# c", Highlight::Comment),
            (1, "@a", Highlight::Variable),
            (1, "1", Highlight::Number),
            (1, "if", Highlight::Keyword1),
            (2, "end", Highlight::Keyword1),
        ],
    },
    Sample {
        file: "a.php",
        filetype: "php",
        lines: &[
            "<?php $a = 1; // c",
            "# also c",
            "function calc(int $x): string { return null; }",
        ],
        expect: &[
            (0, "$a", Highlight::Variable),
            (0, "1", Highlight::Number),
            (0, "// c", Highlight::Comment),
            (1, "# also c", Highlight::Comment),
            (2, "function", Highlight::Keyword1),
            (2, "calc", Highlight::Function),
            (2, "int", Highlight::Type),
            (2, "$x", Highlight::Variable),
            (2, "null", Highlight::Constant),
        ],
    },
    Sample {
        file: "a.swift",
        filetype: "swift",
        lines: &["@objc func calc(_ x: Int) -> Bool { return nil }"],
        expect: &[
            (0, "@objc", Highlight::Preprocessor),
            (0, "func", Highlight::Keyword1),
            (0, "calc", Highlight::Function),
            (0, "Int", Highlight::Type),
            (0, "->", Highlight::Operator),
            (0, "nil", Highlight::Constant),
        ],
    },
    Sample {
        file: "a.kt",
        filetype: "kotlin",
        lines: &["@JvmStatic fun calc(x: Int): String? = null"],
        expect: &[
            (0, "@JvmStatic", Highlight::Preprocessor),
            (0, "fun", Highlight::Keyword1),
            (0, "calc", Highlight::Function),
            (0, "Int", Highlight::Type),
            (0, "null", Highlight::Constant),
        ],
    },
    Sample {
        file: "a.sh",
        filetype: "shell",
        lines: &[
            "#!/bin/sh",
            "echo \"x\" | grep -q y # c",
            "if [ -n \"$A\" ]; then",
            "x=$1 $@ ${B} $(cmd)",
        ],
        expect: &[
            (0, "#!/bin/sh", Highlight::Comment),
            (1, "\"x\"", Highlight::String),
            (1, "|", Highlight::Operator),
            (1, "# c", Highlight::Comment),
            (2, "if", Highlight::Keyword1),
            (2, "then", Highlight::Keyword1),
            (3, "$1", Highlight::Variable),
            (3, "$@", Highlight::Variable),
            (3, "${B}", Highlight::Variable),
            (3, "$(cmd)", Highlight::Variable),
        ],
    },
    Sample {
        file: "a.sql",
        filetype: "sql",
        lines: &[
            "select id, count(*) from t where n >= 10 -- c",
            "/* x */ INSERT INTO t VALUES ('a', NULL);",
            "create table t (id int);",
        ],
        expect: &[
            (0, "select", Highlight::Keyword1),
            (0, "count", Highlight::Function),
            (0, "10", Highlight::Number),
            (0, "-- c", Highlight::Comment),
            (1, "/* x */", Highlight::MlComment),
            (1, "INSERT", Highlight::Keyword1),
            (1, "'a'", Highlight::String),
            (1, "NULL", Highlight::Constant),
            (2, "int", Highlight::Type),
        ],
    },
    Sample {
        file: "a.yaml",
        filetype: "yaml",
        lines: &[
            "---",
            "name: cera # c",
            "- key: 1",
            "  ok: true",
            "  ref: *a",
        ],
        expect: &[
            (0, "---", Highlight::Preprocessor),
            (1, "name", Highlight::Type),
            (1, "# c", Highlight::Comment),
            (2, "key", Highlight::Type),
            (2, "1", Highlight::Number),
            (3, "true", Highlight::Constant),
            (4, "*a", Highlight::Variable),
        ],
    },
    Sample {
        file: "a.json",
        filetype: "json",
        lines: &["{\"a\": [1, -2.5e3, true, null, \"x\"]}"],
        expect: &[
            (0, "\"a\"", Highlight::Type),
            (0, "1", Highlight::Number),
            (0, "2.5e3", Highlight::Number),
            (0, "true", Highlight::Constant),
            (0, "null", Highlight::Constant),
            (0, "\"x\"", Highlight::String),
        ],
    },
    Sample {
        file: "a.lua",
        filetype: "lua",
        lines: &[
            "local function calc(x) return x ~= nil end -- c",
            "--[[ ml",
            "still ]] print(1)",
        ],
        expect: &[
            (0, "local", Highlight::Keyword1),
            (0, "function", Highlight::Keyword1),
            (0, "calc", Highlight::Function),
            (0, "nil", Highlight::Constant),
            (0, "-- c", Highlight::Comment),
            (1, "--[[ ml", Highlight::MlComment),
            (2, "still ]]", Highlight::MlComment),
            (2, "1", Highlight::Number),
        ],
    },
    Sample {
        file: "a.pl",
        filetype: "perl",
        lines: &[
            "my $x = 1; my @a = (1); my %h; # c",
            "print \"hi\\n\" if $x;",
        ],
        expect: &[
            (0, "my", Highlight::Keyword1),
            (0, "$x", Highlight::Variable),
            (0, "@a", Highlight::Variable),
            (0, "%h", Highlight::Variable),
            (0, "# c", Highlight::Comment),
            (1, "\\n", Highlight::Escape),
            (1, "$x", Highlight::Variable),
        ],
    },
    Sample {
        file: "a.hs",
        filetype: "haskell",
        lines: &[
            "module Main where",
            "f :: Int -> Maybe Bool -- c",
            "f x = Just 'a' {- ml -}",
        ],
        expect: &[
            (0, "module", Highlight::Keyword1),
            (0, "Main", Highlight::Type),
            (1, "Int", Highlight::Type),
            (1, "Maybe", Highlight::Type),
            (1, "->", Highlight::Operator),
            (1, "-- c", Highlight::Comment),
            (2, "Just", Highlight::Constant),
            (2, "'a'", Highlight::String),
            (2, "{- ml -}", Highlight::MlComment),
        ],
    },
    Sample {
        file: "a.R",
        filetype: "r",
        lines: &["x <- c(1, 2L, TRUE, NA) # c", "f <- function(a) a$b"],
        expect: &[
            (0, "<-", Highlight::Operator),
            (0, "c", Highlight::Function),
            (0, "2L", Highlight::Number),
            (0, "TRUE", Highlight::Constant),
            (0, "NA", Highlight::Constant),
            (0, "# c", Highlight::Comment),
            (1, "function", Highlight::Keyword1),
        ],
    },
    Sample {
        file: "a.zig",
        filetype: "zig",
        lines: &[
            "const std = @import(\"std\");",
            "pub fn main() !void { var x: u8 = 0; }",
        ],
        expect: &[
            (0, "const", Highlight::Keyword1),
            (0, "@import", Highlight::Preprocessor),
            (1, "fn", Highlight::Keyword1),
            (1, "main", Highlight::Function),
            (1, "void", Highlight::Type),
            (1, "u8", Highlight::Type),
            (1, "0", Highlight::Number),
        ],
    },
    Sample {
        file: "a.html",
        filetype: "html",
        lines: &[
            "<!DOCTYPE html>",
            "<div class=\"a\">Hi &lt; there</div> <!-- c -->",
        ],
        expect: &[
            (0, "<!DOCTYPE", Highlight::Keyword1),
            (1, "<div", Highlight::Keyword1),
            (1, "class", Highlight::Type),
            (1, "\"a\"", Highlight::String),
            (1, "&lt;", Highlight::Escape),
            (1, "</div", Highlight::Keyword1),
            (1, "<!-- c -->", Highlight::MlComment),
        ],
    },
    Sample {
        file: "a.css",
        filetype: "css",
        lines: &[
            "@media (x) { .btn { color: #fff; margin: 10px; } } /* c */",
            "a:hover { background: url(x.png) }",
        ],
        expect: &[
            (0, "@media", Highlight::Preprocessor),
            (0, ".btn", Highlight::Variable),
            (0, "#fff", Highlight::Number),
            (0, "10px", Highlight::Number),
            (0, "/* c */", Highlight::MlComment),
            (1, "url", Highlight::Function),
        ],
    },
    Sample {
        file: "a.md",
        filetype: "markdown",
        lines: &[
            "# Title",
            "text `code` end",
            "```",
            "# not heading",
            "```",
            "> quote",
        ],
        expect: &[
            (0, "# Title", Highlight::Keyword1),
            (1, "`code`", Highlight::String),
            (2, "```", Highlight::String),
            (3, "# not heading", Highlight::String),
            (4, "```", Highlight::String),
            (5, "> quote", Highlight::Comment),
        ],
    },
    Sample {
        file: "app.ts",
        filetype: "typescript",
        lines: &["interface User { name: string; }", "const n: number = 42;"],
        expect: &[
            (0, "interface", Highlight::Keyword1),
            (0, "User", Highlight::Type),
            (0, "string", Highlight::Type),
            (1, "const", Highlight::Keyword1),
            (1, "number", Highlight::Type),
            (1, "42", Highlight::Number),
        ],
    },
    Sample {
        file: "main.dart",
        filetype: "dart",
        lines: &["void main() { print('hi'); }"],
        expect: &[
            (0, "void", Highlight::Type),
            (0, "main", Highlight::Function),
            (0, "print", Highlight::Function),
            (0, "'hi'", Highlight::String),
        ],
    },
    Sample {
        file: "Main.scala",
        filetype: "scala",
        lines: &["object Main { def main(args: Array[String]): Unit = {} }"],
        expect: &[
            (0, "object", Highlight::Keyword1),
            (0, "def", Highlight::Keyword1),
            (0, "Array", Highlight::Type),
            (0, "Unit", Highlight::Type),
        ],
    },
    Sample {
        file: "a.jl",
        filetype: "julia",
        lines: &["function add(x::Int, y)", "  return x' + 1 # c"],
        expect: &[
            (0, "function", Highlight::Keyword1),
            (0, "add", Highlight::Function),
            (0, "Int", Highlight::Type),
            (1, "return", Highlight::Keyword1),
            (1, "1", Highlight::Number),
            (1, "# c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.ex",
        filetype: "elixir",
        lines: &["defmodule Foo do", "  def bar(x), do: x + 1"],
        expect: &[
            (0, "defmodule", Highlight::Keyword1),
            (0, "Foo", Highlight::Type),
            (1, "def", Highlight::Keyword1),
            (1, "bar", Highlight::Function),
            (1, "1", Highlight::Number),
        ],
    },
    Sample {
        file: "a.erl",
        filetype: "erlang",
        lines: &["-module(foo).", "add(X, Y) -> X + 1. % c"],
        expect: &[
            (0, "-module", Highlight::Preprocessor),
            (1, "add", Highlight::Function),
            (1, "1", Highlight::Number),
            (1, "% c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.clj",
        filetype: "clojure",
        lines: &["(defn add [x y] (+ x y)) ; c", "(println :ok 3)"],
        expect: &[
            (0, "defn", Highlight::Keyword1),
            (0, "; c", Highlight::Comment),
            (1, "println", Highlight::Function),
            (1, ":ok", Highlight::Variable),
            (1, "3", Highlight::Number),
        ],
    },
    Sample {
        file: "a.ml",
        filetype: "ocaml",
        lines: &["let rec fact n = if n = 0 then 1 else n", "(* c *)"],
        expect: &[
            (0, "let", Highlight::Keyword1),
            (0, "rec", Highlight::Keyword1),
            (0, "0", Highlight::Number),
            (1, "(* c *)", Highlight::MlComment),
        ],
    },
    Sample {
        file: "a.fs",
        filetype: "fsharp",
        lines: &["let add x y = x + y // c"],
        expect: &[
            (0, "let", Highlight::Keyword1),
            (0, "// c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.ps1",
        filetype: "powershell",
        lines: &["Write-Host $name # hi"],
        expect: &[
            (0, "Write-Host", Highlight::Keyword2),
            (0, "$name", Highlight::Variable),
            (0, "# hi", Highlight::Comment),
        ],
    },
    Sample {
        file: "build.gradle",
        filetype: "groovy",
        lines: &["def x = 1 // c"],
        expect: &[
            (0, "def", Highlight::Keyword1),
            (0, "1", Highlight::Number),
            (0, "// c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.m",
        filetype: "matlab",
        lines: &["x = [1 2 3]'; % t", "disp('hi')"],
        expect: &[
            (0, "1", Highlight::Number),
            (0, "% t", Highlight::Comment),
            (1, "disp", Highlight::Keyword2),
            (1, "'hi'", Highlight::String),
        ],
    },
    Sample {
        file: "a.f90",
        filetype: "fortran",
        lines: &["program hi", "  integer :: x = 1 ! c"],
        expect: &[
            (0, "program", Highlight::Keyword1),
            (1, "integer", Highlight::Type),
            (1, "1", Highlight::Number),
            (1, "! c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.pas",
        filetype: "pascal",
        lines: &["program Hello;", "begin writeln('hi'); { c } end."],
        expect: &[
            (0, "program", Highlight::Keyword1),
            (1, "begin", Highlight::Keyword1),
            (1, "'hi'", Highlight::String),
            (1, "{ c }", Highlight::MlComment),
        ],
    },
    Sample {
        file: "a.adb",
        filetype: "ada",
        lines: &["procedure Main is -- c", "begin null; end Main;"],
        expect: &[
            (0, "procedure", Highlight::Keyword1),
            (0, "-- c", Highlight::Comment),
            (1, "begin", Highlight::Keyword1),
            (1, "null", Highlight::Keyword1),
        ],
    },
    Sample {
        file: "a.cob",
        filetype: "cobol",
        lines: &["       DISPLAY 'HI' *> c", "       MOVE 1 TO X."],
        expect: &[
            (0, "DISPLAY", Highlight::Keyword1),
            (0, "'HI'", Highlight::String),
            (0, "*> c", Highlight::Comment),
            (1, "MOVE", Highlight::Keyword1),
            (1, "1", Highlight::Number),
        ],
    },
    Sample {
        file: "a.s",
        filetype: "assembly",
        lines: &[".text", "main: mov %rax, 1 # c", "  mov rax, rbx"],
        expect: &[
            (0, ".text", Highlight::Preprocessor),
            (1, "mov", Highlight::Keyword1),
            (1, "%rax", Highlight::Variable),
            (1, "1", Highlight::Number),
            (1, "# c", Highlight::Comment),
            (2, "rbx", Highlight::Keyword2),
        ],
    },
    Sample {
        file: "a.vb",
        filetype: "visualbasic",
        lines: &["Dim x As Integer = 1 ' c"],
        expect: &[
            (0, "Dim", Highlight::Keyword1),
            (0, "Integer", Highlight::Type),
            (0, "1", Highlight::Number),
            (0, "' c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.lisp",
        filetype: "lisp",
        lines: &["(defun add (a b) (+ a b)) ; c", "(print :ok)"],
        expect: &[
            (0, "defun", Highlight::Keyword1),
            (0, "; c", Highlight::Comment),
            (1, "print", Highlight::Function),
            (1, ":ok", Highlight::Variable),
        ],
    },
    Sample {
        file: "a.scm",
        filetype: "scheme",
        lines: &["(define (f x) (* x 2)) ; c", "(display x)"],
        expect: &[
            (0, "define", Highlight::Keyword1),
            (0, "; c", Highlight::Comment),
            (0, "2", Highlight::Number),
            (1, "display", Highlight::Function),
        ],
    },
    Sample {
        file: "a.nim",
        filetype: "nim",
        lines: &["proc add(x: int): int = x + 1 # c"],
        expect: &[
            (0, "proc", Highlight::Keyword1),
            (0, "add", Highlight::Function),
            (0, "int", Highlight::Type),
            (0, "1", Highlight::Number),
            (0, "# c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.cr",
        filetype: "crystal",
        lines: &["def hi(x : Int32) # c", "  @name = 1"],
        expect: &[
            (0, "def", Highlight::Keyword1),
            (0, "Int32", Highlight::Type),
            (0, "# c", Highlight::Comment),
            (1, "@name", Highlight::Variable),
            (1, "1", Highlight::Number),
        ],
    },
    Sample {
        file: "a.d",
        filetype: "d",
        lines: &["void main() { int x = 1; }"],
        expect: &[
            (0, "void", Highlight::Type),
            (0, "main", Highlight::Function),
            (0, "int", Highlight::Type),
            (0, "1", Highlight::Number),
        ],
    },
    Sample {
        file: "a.tcl",
        filetype: "tcl",
        lines: &["set x 1 ;# c", "puts $x"],
        expect: &[
            (0, "set", Highlight::Keyword1),
            (0, "1", Highlight::Number),
            (0, "# c", Highlight::Comment),
            (1, "puts", Highlight::Keyword1),
            (1, "$x", Highlight::Variable),
        ],
    },
    Sample {
        file: "a.awk",
        filetype: "awk",
        lines: &["BEGIN { print $1 } # c"],
        expect: &[
            (0, "BEGIN", Highlight::Keyword1),
            (0, "print", Highlight::Keyword1),
            (0, "$1", Highlight::Variable),
            (0, "# c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.sol",
        filetype: "solidity",
        lines: &["pragma solidity ^0.8.0;", "contract Foo { uint256 x; }"],
        expect: &[
            (0, "pragma", Highlight::Keyword1),
            (1, "contract", Highlight::Keyword1),
            (1, "Foo", Highlight::Type),
            (1, "uint256", Highlight::Type),
        ],
    },
    Sample {
        file: "a.v",
        filetype: "verilog",
        lines: &[
            "`define W 8",
            "module top(input clk); // c",
            "  assign y = $time;",
        ],
        expect: &[
            (0, "`define", Highlight::Preprocessor),
            (1, "module", Highlight::Keyword1),
            (1, "input", Highlight::Keyword1),
            (1, "// c", Highlight::Comment),
            (2, "assign", Highlight::Keyword1),
            (2, "$time", Highlight::Variable),
        ],
    },
    Sample {
        file: "a.vhd",
        filetype: "vhdl",
        lines: &["entity top is -- c", "  port (clk : in std_logic);"],
        expect: &[
            (0, "entity", Highlight::Keyword1),
            (0, "-- c", Highlight::Comment),
            (1, "port", Highlight::Keyword1),
            (1, "std_logic", Highlight::Type),
        ],
    },
    Sample {
        file: "a.coffee",
        filetype: "coffeescript",
        lines: &["square = (x) -> x * 2 # c", "@name = Foo"],
        expect: &[
            (0, "2", Highlight::Number),
            (0, "# c", Highlight::Comment),
            (1, "@name", Highlight::Variable),
            (1, "Foo", Highlight::Type),
        ],
    },
    Sample {
        file: "a.elm",
        filetype: "elm",
        lines: &["module Main exposing (main)", "add x y = x + 1 -- c"],
        expect: &[
            (0, "module", Highlight::Keyword1),
            (0, "Main", Highlight::Type),
            (1, "1", Highlight::Number),
            (1, "-- c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.vim",
        filetype: "vim",
        lines: &["set number \" c", "let x = 'a'"],
        expect: &[
            (0, "set", Highlight::Keyword1),
            (0, "\" c", Highlight::Comment),
            (1, "let", Highlight::Keyword1),
            (1, "'a'", Highlight::String),
        ],
    },
    Sample {
        file: "a.pro",
        filetype: "prolog",
        lines: &["parent(X) :- father(X). % c"],
        expect: &[
            (0, "parent", Highlight::Function),
            (0, ":-", Highlight::Operator),
            (0, "% c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.fish",
        filetype: "fish",
        lines: &["set -x PATH $PATH # c"],
        expect: &[
            (0, "set", Highlight::Keyword1),
            (0, "$PATH", Highlight::Variable),
            (0, "# c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.nix",
        filetype: "nix",
        lines: &[
            "{ pkgs, ... }: let",
            "  x = 1; # c",
            "in pkgs.mkShell { y = ''a''; }",
        ],
        expect: &[
            (0, "let", Highlight::Keyword1),
            (1, "x", Highlight::Type),
            (1, "1", Highlight::Number),
            (1, "# c", Highlight::Comment),
            (2, "in", Highlight::Keyword1),
            (2, "mkShell", Highlight::Keyword2),
            (2, "''a''", Highlight::String),
        ],
    },
    Sample {
        file: "a.toml",
        filetype: "toml",
        lines: &["[package]", "name = \"cera\" # c", "version = 1"],
        expect: &[
            (0, "[package]", Highlight::Keyword1),
            (1, "name", Highlight::Type),
            (1, "\"cera\"", Highlight::String),
            (1, "# c", Highlight::Comment),
            (2, "1", Highlight::Number),
        ],
    },
    Sample {
        file: "a.ini",
        filetype: "ini",
        lines: &["[main]", "key = value ; c", "n = 12"],
        expect: &[
            (0, "[main]", Highlight::Keyword1),
            (1, "key", Highlight::Type),
            (1, "; c", Highlight::Comment),
            (2, "12", Highlight::Number),
        ],
    },
    Sample {
        file: "a.xml",
        filetype: "xml",
        lines: &["<?xml version=\"1.0\"?>", "<a id=\"1\">x &amp; y</a>"],
        expect: &[
            (0, "<?xml", Highlight::Keyword1),
            (0, "version", Highlight::Type),
            (0, "\"1.0\"", Highlight::String),
            (1, "<a", Highlight::Keyword1),
            (1, "id", Highlight::Type),
            (1, "&amp;", Highlight::Escape),
            (1, "</a", Highlight::Keyword1),
        ],
    },
    Sample {
        file: "a.tex",
        filetype: "latex",
        lines: &["\\section{Hi} % c", "$x$"],
        expect: &[
            (0, "\\section", Highlight::Preprocessor),
            (0, "% c", Highlight::Comment),
            (1, "$x$", Highlight::String),
        ],
    },
    Sample {
        file: "a.diff",
        filetype: "diff",
        lines: &["diff --git a/x b/x", "@@ -1 +1 @@", "+new", "-old", " ctx"],
        expect: &[
            (0, "diff --git a/x b/x", Highlight::Keyword1),
            (1, "@@ -1 +1 @@", Highlight::Preprocessor),
            (2, "+new", Highlight::String),
            (3, "-old", Highlight::Constant),
            (4, " ctx", Highlight::Normal),
        ],
    },
    Sample {
        file: "a.tf",
        filetype: "hcl",
        lines: &["resource \"aws\" \"x\" { # c", "  count = 2"],
        expect: &[
            (0, "resource", Highlight::Keyword1),
            (0, "\"aws\"", Highlight::String),
            (0, "# c", Highlight::Comment),
            (1, "count", Highlight::Type),
            (1, "2", Highlight::Number),
        ],
    },
    Sample {
        file: "a.proto",
        filetype: "protobuf",
        lines: &["message Foo { string name = 1; } // c"],
        expect: &[
            (0, "message", Highlight::Keyword1),
            (0, "Foo", Highlight::Type),
            (0, "string", Highlight::Type),
            (0, "1", Highlight::Number),
            (0, "// c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.gql",
        filetype: "graphql",
        lines: &["query Q($id: ID!) { user(id: $id) { name } } # c"],
        expect: &[
            (0, "query", Highlight::Keyword1),
            (0, "$id", Highlight::Variable),
            (0, "ID", Highlight::Type),
            (0, "user", Highlight::Function),
            (0, "# c", Highlight::Comment),
        ],
    },
    Sample {
        file: "a.scss",
        filetype: "scss",
        lines: &["$c: #fff; // c", "@mixin m { margin: 10px; }"],
        expect: &[
            (0, "$c", Highlight::Variable),
            (0, "#fff", Highlight::Number),
            (0, "// c", Highlight::Comment),
            (1, "@mixin", Highlight::Preprocessor),
            (1, "10px", Highlight::Number),
        ],
    },
    Sample {
        file: "Makefile",
        filetype: "makefile",
        lines: &[
            "CC = gcc # c",
            "all: main.o",
            "  $(CC) -o app $@",
            ".PHONY: all",
        ],
        expect: &[
            (0, "CC", Highlight::Type),
            (0, "# c", Highlight::Comment),
            (1, "all", Highlight::Type),
            (2, "$(CC)", Highlight::Variable),
            (2, "$@", Highlight::Variable),
            (3, ".PHONY", Highlight::Preprocessor),
        ],
    },
    Sample {
        file: "CMakeLists.txt",
        filetype: "cmake",
        lines: &["project(Foo) # c", "set(X ${Y} ON)"],
        expect: &[
            (0, "project", Highlight::Keyword2),
            (0, "# c", Highlight::Comment),
            (1, "set", Highlight::Keyword2),
            (1, "${Y}", Highlight::Variable),
            (1, "ON", Highlight::Constant),
        ],
    },
    Sample {
        file: "Dockerfile",
        filetype: "dockerfile",
        lines: &["FROM alpine AS build # c", "ENV A=\"x\" B=$HOME"],
        expect: &[
            (0, "FROM", Highlight::Keyword1),
            (0, "AS", Highlight::Keyword1),
            (0, "# c", Highlight::Comment),
            (1, "ENV", Highlight::Keyword1),
            (1, "\"x\"", Highlight::String),
            (1, "$HOME", Highlight::Variable),
        ],
    },
    Sample {
        file: "page.html",
        filetype: "html",
        lines: &[
            "<script type=\"x\">",
            "const n = 42; // c",
            "</script>",
            "<style>a { margin: 10px; }</style>",
            "<p>text</p>",
        ],
        expect: &[
            (0, "<script", Highlight::Keyword1),
            (0, "type", Highlight::Type),
            (1, "const", Highlight::Keyword2),
            (1, "42", Highlight::Number),
            (1, "// c", Highlight::Comment),
            (2, "</script", Highlight::Keyword1),
            (3, "a", Highlight::Type),
            (3, "10px", Highlight::Number),
            (3, "</style", Highlight::Keyword1),
            (4, "<p", Highlight::Keyword1),
        ],
    },
    Sample {
        file: "t.py",
        filetype: "python",
        lines: &[
            "page = \"\"\"<div class=\"a\">hi &amp; bye</div>",
            "<script>var x = 1;</script>\"\"\"",
            "y = 2",
        ],
        expect: &[
            (0, "<div", Highlight::Keyword1),
            (0, "class", Highlight::Type),
            (0, "\"a\"", Highlight::String),
            (0, "&amp;", Highlight::Escape),
            (0, "</div", Highlight::Keyword1),
            (1, "<script", Highlight::Keyword1),
            (1, "var", Highlight::Normal),
            (1, "\"\"\"", Highlight::String),
            (2, "y", Highlight::Normal),
            (2, "2", Highlight::Number),
        ],
    },
    Sample {
        file: "u.py",
        filetype: "python",
        lines: &[
            "q = \"\"\"",
            "  <p>hi</p>",
            "\"\"\"",
            "def f():",
            "    \"\"\"Select the best from list\"\"\"",
        ],
        expect: &[
            (1, "<p", Highlight::Keyword1),
            (1, "</p", Highlight::Keyword1),
            (2, "\"\"\"", Highlight::String),
            (3, "def", Highlight::Keyword1),
            (4, "Select", Highlight::String),
            (4, "from", Highlight::String),
        ],
    },
    Sample {
        file: "v.py",
        filetype: "python",
        lines: &[
            "q = \"SELECT id FROM t WHERE n = 1\"",
            "x = '<b class=\"c\">y</b>'",
        ],
        expect: &[
            (0, "\"", Highlight::String),
            (0, "SELECT", Highlight::Keyword1),
            (0, "FROM", Highlight::Keyword1),
            (0, "WHERE", Highlight::Keyword1),
            (0, "1", Highlight::Number),
            (1, "<b", Highlight::Keyword1),
            (1, "class", Highlight::Type),
            (1, "\"c\"", Highlight::String),
            (1, "</b", Highlight::Keyword1),
        ],
    },
    Sample {
        file: "w.js",
        filetype: "javascript",
        lines: &["const t = `<ul><li>${x}</li></ul>`;"],
        expect: &[
            (0, "<ul", Highlight::Keyword1),
            (0, "</li", Highlight::Keyword1),
            (0, "</ul", Highlight::Keyword1),
        ],
    },
    Sample {
        file: "x.rs",
        filetype: "rust",
        lines: &["let q = \"SELECT * FROM t\";"],
        expect: &[
            (0, "SELECT", Highlight::Keyword1),
            (0, "FROM", Highlight::Keyword1),
        ],
    },
    Sample {
        file: "n.md",
        filetype: "markdown",
        lines: &[
            "# T",
            "```python",
            "def f(x):",
            "    return 1",
            "```",
            "```text",
            "# not a heading",
            "```",
            "end",
        ],
        expect: &[
            (0, "# T", Highlight::Keyword1),
            (1, "```python", Highlight::String),
            (2, "def", Highlight::Keyword1),
            (3, "return", Highlight::Keyword1),
            (3, "1", Highlight::Number),
            (4, "```", Highlight::String),
            (6, "# not a heading", Highlight::String),
            (8, "end", Highlight::Normal),
        ],
    },
    Sample {
        file: "m.md",
        filetype: "markdown",
        lines: &[
            "```js",
            "const a = 1;",
            "```",
            "```yaml",
            "name: x",
            "```",
            "```sh",
            "echo $HOME",
            "```",
        ],
        expect: &[
            (1, "const", Highlight::Keyword2),
            (1, "1", Highlight::Number),
            (4, "name", Highlight::Type),
            (7, "$HOME", Highlight::Variable),
        ],
    },
];

pub fn highlight_lines(syntax: &Syntax, lines: &[&str]) -> Vec<Vec<Highlight>> {
    let mut result = Vec::new();
    let mut state = crate::model::row::HlState::default();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let row = engine::highlight_row(&chars, syntax, state);
        state = row.state;
        result.push(row.hl);
    }
    result
}

#[test]
fn every_language_sample_is_highlighted_as_expected() {
    let database = build_database();
    let mut failures: Vec<String> = Vec::new();
    for sample in SAMPLES {
        let Some(syntax) = select_for_filename(&database, sample.file) else {
            failures.push(format!("no syntax selected for {}", sample.file));
            continue;
        };
        if syntax.filetype != sample.filetype {
            failures.push(format!(
                "file {} selected {} instead of {}",
                sample.file, syntax.filetype, sample.filetype
            ));
            continue;
        }
        let highlighted = highlight_lines(syntax, sample.lines);
        for (line_idx, token, expected) in sample.expect {
            let line = sample.lines[*line_idx];
            let Some(byte_start) = line.find(token) else {
                failures.push(format!(
                    "{}: token {:?} not in line {:?}",
                    sample.filetype, token, line
                ));
                continue;
            };
            let start = line[..byte_start].chars().count();
            let len = token.chars().count();
            let got = &highlighted[*line_idx][start..start + len];
            if got.iter().any(|k| k != expected) {
                failures.push(format!(
                    "{}: {:?} in {:?} expected {:?}, got {:?}",
                    sample.filetype,
                    token,
                    line,
                    expected,
                    got.first()
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "
{}",
        failures.join(
            "
"
        )
    );
}
