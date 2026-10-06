#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Edit {
        explorer: Option<String>,
        files: Vec<String>,
    },
    Show {
        files: Vec<String>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum CliError {
    UnknownOption(String),
    ShowWithoutFile,
    ShowAndExplorer,
}

fn is_option(arg: &str) -> bool {
    arg.len() > 1 && arg.starts_with('-')
}

pub fn parse<S: AsRef<str>>(args: &[S]) -> Result<Command, CliError> {
    let mut explorer_requested = false;
    let mut show_requested = false;
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
            other => return Err(CliError::UnknownOption(other.to_string())),
        }
    }

    if show_requested && explorer_requested {
        return Err(CliError::ShowAndExplorer);
    }

    if show_requested {
        if positional.is_empty() {
            return Err(CliError::ShowWithoutFile);
        }
        return Ok(Command::Show { files: positional });
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
}
