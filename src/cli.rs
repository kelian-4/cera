#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Edit {
        explorer: Option<String>,
        files: Vec<String>,
    },
    Show {
        files: Vec<String>,
    },
    Find {
        name: String,
    },
    Grep {
        pattern: String,
        paths: Vec<String>,
    },
    Diff {
        files: Vec<String>,
    },
    Help,
    Version,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CliError {
    UnknownOption(String),
    ShowWithoutFile,
    ShowAndExplorer,
    IncompatibleOptions,
    FindExpectsOneName,
    GrepWithoutPattern,
    DiffNeedsTwoFiles,
}

fn is_option(arg: &str) -> bool {
    arg.len() > 1 && arg.starts_with('-')
}

fn single_argument(mut positional: Vec<String>) -> Option<String> {
    let first = if positional.is_empty() {
        None
    } else {
        Some(positional.remove(0))
    };
    if positional.is_empty() {
        first
    } else {
        None
    }
}

fn option_requested<S: AsRef<str>>(args: &[S], names: &[&str]) -> bool {
    args.iter()
        .map(|arg| arg.as_ref())
        .take_while(|arg| *arg != "--")
        .any(|arg| names.contains(&arg))
}

pub fn parse<S: AsRef<str>>(args: &[S]) -> Result<Command, CliError> {
    if option_requested(args, &["-h", "--help"]) {
        return Ok(Command::Help);
    }
    if option_requested(args, &["--version"]) {
        return Ok(Command::Version);
    }

    let mut explorer_requested = false;
    let mut show_requested = false;
    let mut find_requested = false;
    let mut grep_requested = false;
    let mut diff_requested = false;
    let mut positional: Vec<String> = Vec::new();
    let mut options_done = false;

    for arg in args {
        let arg = arg.as_ref();
        if options_done || !is_option(arg) {
            positional.push(arg.to_string());
            continue;
        }
        match arg {
            "--" => options_done = true,
            "-e" | "--explorer" => explorer_requested = true,
            "-a" | "--afficher" => show_requested = true,
            "-f" | "--find" => find_requested = true,
            "-g" | "--grep" => grep_requested = true,
            "-d" | "--diff" => diff_requested = true,
            other => return Err(CliError::UnknownOption(other.to_string())),
        }
    }

    let requested = [
        show_requested,
        explorer_requested,
        find_requested,
        grep_requested,
        diff_requested,
    ]
    .iter()
    .filter(|r| **r)
    .count();

    if requested > 1 {
        if show_requested && explorer_requested && requested == 2 {
            return Err(CliError::ShowAndExplorer);
        }
        return Err(CliError::IncompatibleOptions);
    }

    if show_requested {
        if positional.is_empty() {
            return Err(CliError::ShowWithoutFile);
        }
        return Ok(Command::Show { files: positional });
    }

    if find_requested {
        return single_argument(positional)
            .map(|name| Command::Find { name })
            .ok_or(CliError::FindExpectsOneName);
    }

    if grep_requested {
        let mut iter = positional.into_iter();
        return match iter.next() {
            Some(pattern) => Ok(Command::Grep {
                pattern,
                paths: iter.collect(),
            }),
            None => Err(CliError::GrepWithoutPattern),
        };
    }

    if diff_requested {
        if positional.len() < 2 {
            return Err(CliError::DiffNeedsTwoFiles);
        }
        return Ok(Command::Diff { files: positional });
    }

    if explorer_requested {
        let mut iter = positional.into_iter();
        let dir = iter.next().unwrap_or_else(|| ".".to_string());
        return Ok(Command::Edit {
            explorer: Some(dir),
            files: iter.collect(),
        });
    }

    Ok(Command::Edit {
        explorer: None,
        files: positional,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(args: &[&str]) -> Result<Command, CliError> {
        parse(args)
    }

    #[test]
    fn test_no_args_opens_editor_without_files() {
        assert_eq!(
            p(&[]),
            Ok(Command::Edit {
                explorer: None,
                files: vec![]
            })
        );
    }

    #[test]
    fn test_plain_files_open_in_editor() {
        assert_eq!(
            p(&["a.rs", "b.rs"]),
            Ok(Command::Edit {
                explorer: None,
                files: vec!["a.rs".into(), "b.rs".into()]
            })
        );
    }

    #[test]
    fn test_explorer_without_dir_uses_current_directory() {
        for flag in ["-e", "--explorer"] {
            assert_eq!(
                p(&[flag]),
                Ok(Command::Edit {
                    explorer: Some(".".into()),
                    files: vec![]
                })
            );
        }
    }

    #[test]
    fn test_explorer_with_dir() {
        assert_eq!(
            p(&["-e", "mon-projet"]),
            Ok(Command::Edit {
                explorer: Some("mon-projet".into()),
                files: vec![]
            })
        );
    }

    #[test]
    fn test_explorer_dir_can_come_before_the_flag() {
        assert_eq!(
            p(&["mon-projet", "-e"]),
            Ok(Command::Edit {
                explorer: Some("mon-projet".into()),
                files: vec![]
            })
        );
    }

    #[test]
    fn test_explorer_with_dir_and_extra_files() {
        assert_eq!(
            p(&["-e", "proj", "src/main.rs"]),
            Ok(Command::Edit {
                explorer: Some("proj".into()),
                files: vec!["src/main.rs".into()]
            })
        );
    }

    #[test]
    fn test_show_collects_all_files() {
        for flag in ["-a", "--afficher"] {
            assert_eq!(
                p(&[flag, "a.rs", "b.rs"]),
                Ok(Command::Show {
                    files: vec!["a.rs".into(), "b.rs".into()]
                })
            );
        }
    }

    #[test]
    fn test_show_without_file_is_an_error() {
        assert_eq!(p(&["-a"]), Err(CliError::ShowWithoutFile));
    }

    #[test]
    fn test_show_and_explorer_together_is_an_error() {
        assert_eq!(p(&["-a", "-e", "x"]), Err(CliError::ShowAndExplorer));
    }

    #[test]
    fn test_unknown_option_is_an_error() {
        assert_eq!(p(&["-x"]), Err(CliError::UnknownOption("-x".into())));
        assert_eq!(
            p(&["--nope", "f"]),
            Err(CliError::UnknownOption("--nope".into()))
        );
    }

    #[test]
    fn test_double_dash_ends_options() {
        assert_eq!(
            p(&["--", "-e", "-a"]),
            Ok(Command::Edit {
                explorer: None,
                files: vec!["-e".into(), "-a".into()]
            })
        );
    }

    #[test]
    fn test_single_dash_is_a_filename_not_an_option() {
        assert_eq!(
            p(&["-"]),
            Ok(Command::Edit {
                explorer: None,
                files: vec!["-".into()]
            })
        );
    }

    #[test]
    fn test_find_takes_one_name() {
        for flag in ["-f", "--find"] {
            assert_eq!(
                p(&[flag, "main"]),
                Ok(Command::Find {
                    name: "main".into()
                })
            );
        }
    }

    #[test]
    fn test_find_requires_exactly_one_name() {
        assert_eq!(p(&["-f"]), Err(CliError::FindExpectsOneName));
        assert_eq!(p(&["-f", "a", "b"]), Err(CliError::FindExpectsOneName));
    }

    #[test]
    fn test_grep_takes_pattern_without_paths() {
        for flag in ["-g", "--grep"] {
            assert_eq!(
                p(&[flag, "fn main"]),
                Ok(Command::Grep {
                    pattern: "fn main".into(),
                    paths: vec![]
                })
            );
        }
    }

    #[test]
    fn test_grep_takes_pattern_then_paths() {
        assert_eq!(
            p(&["-g", "fn main", "src/main.rs", "plugins"]),
            Ok(Command::Grep {
                pattern: "fn main".into(),
                paths: vec!["src/main.rs".into(), "plugins".into()]
            })
        );
    }

    #[test]
    fn test_grep_requires_a_pattern() {
        assert_eq!(p(&["-g"]), Err(CliError::GrepWithoutPattern));
    }

    #[test]
    fn test_grep_pattern_starting_with_dash_needs_double_dash() {
        assert_eq!(
            p(&["-g", "--", "-x", "src"]),
            Ok(Command::Grep {
                pattern: "-x".into(),
                paths: vec!["src".into()]
            })
        );
    }

    #[test]
    fn test_diff_takes_two_or_more_files() {
        for flag in ["-d", "--diff"] {
            assert_eq!(
                p(&[flag, "a", "b", "c"]),
                Ok(Command::Diff {
                    files: vec!["a".into(), "b".into(), "c".into()]
                })
            );
        }
    }

    #[test]
    fn test_diff_requires_two_files() {
        assert_eq!(p(&["-d"]), Err(CliError::DiffNeedsTwoFiles));
        assert_eq!(p(&["-d", "a"]), Err(CliError::DiffNeedsTwoFiles));
    }

    #[test]
    fn test_modes_are_mutually_exclusive() {
        assert_eq!(p(&["-f", "-g", "x"]), Err(CliError::IncompatibleOptions));
        assert_eq!(
            p(&["-d", "-a", "x", "y"]),
            Err(CliError::IncompatibleOptions)
        );
        assert_eq!(p(&["-e", "-f", "x"]), Err(CliError::IncompatibleOptions));
    }

    #[test]
    fn test_help_flags() {
        for flag in ["-h", "--help"] {
            assert_eq!(p(&[flag]), Ok(Command::Help));
        }
    }

    #[test]
    fn test_help_takes_priority_over_other_arguments() {
        assert_eq!(p(&["-g", "x", "--help"]), Ok(Command::Help));
        assert_eq!(p(&["--bogus", "-h"]), Ok(Command::Help));
    }

    #[test]
    fn test_version_flag() {
        assert_eq!(p(&["--version"]), Ok(Command::Version));
    }

    #[test]
    fn test_version_takes_priority_over_other_arguments() {
        assert_eq!(p(&["-g", "x", "--version"]), Ok(Command::Version));
        assert_eq!(p(&["--bogus", "--version"]), Ok(Command::Version));
    }

    #[test]
    fn test_help_takes_priority_over_version() {
        assert_eq!(p(&["--version", "--help"]), Ok(Command::Help));
    }

    #[test]
    fn test_version_after_double_dash_is_a_plain_argument() {
        assert_eq!(
            p(&["--", "--version"]),
            Ok(Command::Edit {
                explorer: None,
                files: vec!["--version".into()]
            })
        );
    }

    #[test]
    fn test_help_after_double_dash_is_a_plain_argument() {
        assert_eq!(
            p(&["--", "-h"]),
            Ok(Command::Edit {
                explorer: None,
                files: vec!["-h".into()]
            })
        );
    }
}
