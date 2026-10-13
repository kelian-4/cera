use cera::cli::{self, CliError, Command};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = match cli::parse(&args[1..]) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("cera: {}", describe_cli_error(&e));
            std::process::exit(1);
        }
    };

    match command {
        Command::Show { files } => run_cli_plugin(
            "pretty-cat",
            "--cat",
            &files,
            "cli.pretty_cat_plugin_not_loaded",
        ),
        Command::Find { name } => {
            run_cli_plugin("finder", "--find", &[name], "cli.finder_plugin_not_loaded")
        }
        Command::Grep { pattern, paths } => {
            let mut args = vec![pattern];
            args.extend(paths);
            run_cli_plugin("grep", "--grep", &args, "cli.grep_plugin_not_loaded")
        }
        Command::Diff { files } => {
            run_cli_plugin("diff", "--diff", &files, "cli.diff_plugin_not_loaded")
        }
        Command::Edit { explorer, files } => run_edit(explorer, &files),
        Command::Help => run_help(),
        Command::Version => run_version(),
    }
}

fn run_version() -> ! {
    println!("cera {}", env!("CARGO_PKG_VERSION"));
    std::process::exit(0);
}

fn run_help() -> ! {
    let config_path = cera::io::config::default_config_path();
    cera::io::config::ensure_config_file_exists(&config_path);
    let (config, _) = cera::io::config::load_from_file(&config_path);
    let i18n = cera::i18n::load(config.language.as_deref());
    let plugins = cera::plugin::registry::plugins_directory(config.plugins_directory.as_deref())
        .map(|dir| cera::plugin::registry::load_enabled_plugins(&dir, &config.plugins_enabled))
        .unwrap_or_default();
    print!("{}", cera::view::help_text::cli_help_text(&i18n, &plugins));
    std::process::exit(0);
}

fn describe_cli_error(e: &CliError) -> String {
    match e {
        CliError::UnknownOption(opt) => format!("option inconnue : {}", opt),
        CliError::ShowWithoutFile => "-a/--afficher attend au moins un fichier".to_string(),
        CliError::ShowAndExplorer => {
            "-a/--afficher et -e/--explorer sont incompatibles".to_string()
        }
        CliError::IncompatibleOptions => {
            "-a, -e, -f, -g et -d sont incompatibles entre eux".to_string()
        }
        CliError::FindExpectsOneName => "-f/--find attend exactement un nom".to_string(),
        CliError::GrepWithoutPattern => {
            "-g/--grep attend un motif, suivi de fichiers ou dossiers optionnels".to_string()
        }
        CliError::DiffNeedsTwoFiles => "-d/--diff attend au moins deux fichiers".to_string(),
    }
}

fn run_cli_plugin(plugin_id: &str, mode_flag: &str, args: &[String], not_loaded_key: &str) -> ! {
    let config_path = cera::io::config::default_config_path();
    cera::io::config::ensure_config_file_exists(&config_path);
    let (config, _) = cera::io::config::load_from_file(&config_path);
    let i18n = cera::i18n::load(config.language.as_deref());

    let plugins_dir =
        cera::plugin::registry::plugins_directory(config.plugins_directory.as_deref());
    let manifest =
        plugins_dir.and_then(|dir| cera::plugin::registry::load_manifest(&dir, plugin_id));

    let is_enabled = config.plugins_enabled.iter().any(|id| id == plugin_id);
    let Some(manifest) = manifest.filter(|_| is_enabled) else {
        eprintln!("cera: {}", i18n.t(not_loaded_key));
        std::process::exit(1);
    };

    let mut command_args = manifest.args.clone();
    command_args.push(mode_flag.to_string());
    command_args.extend(args.iter().cloned());

    let status = std::process::Command::new(&manifest.command)
        .args(&command_args)
        .status();

    match status {
        Ok(status) => std::process::exit(status.code().unwrap_or(1)),
        Err(e) => {
            eprintln!(
                "cera: {}",
                i18n.t1(
                    "status.plugin_failed_to_start",
                    &format!("{} ({})", plugin_id, e)
                )
            );
            std::process::exit(1);
        }
    }
}

fn run_edit(explorer: Option<String>, files: &[String]) -> ! {
    for f in files {
        if std::path::Path::new(f).is_dir() {
            report_directory_error(f);
        }
    }

    let mut editor = match cera::editor::core::Editor::new() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Erreur d'initialisation : {}", e);
            std::process::exit(1);
        }
    };

    for filename in files {
        editor.open_file(filename);
    }

    if let Some(dir) = explorer {
        if let Err(msg) = editor.open_explorer_on(&dir) {
            eprintln!("cera: {}", msg);
            std::process::exit(1);
        }
    }

    editor.run();
}

fn report_directory_error(path: &str) -> ! {
    let config_path = cera::io::config::default_config_path();
    cera::io::config::ensure_config_file_exists(&config_path);
    let (config, _) = cera::io::config::load_from_file(&config_path);
    let i18n = cera::i18n::load(config.language.as_deref());

    let plugins_dir =
        cera::plugin::registry::plugins_directory(config.plugins_directory.as_deref());
    let explorer_loaded = config
        .plugins_enabled
        .iter()
        .any(|id| id == "file-explorer")
        && plugins_dir
            .map(|dir| cera::plugin::registry::load_manifest(&dir, "file-explorer").is_some())
            .unwrap_or(false);

    let key = if explorer_loaded {
        "cli.is_a_directory_use_e"
    } else {
        "cli.is_a_directory_enable_plugin"
    };
    eprintln!("cera: {}", i18n.t1(key, path));
    std::process::exit(1);
}
