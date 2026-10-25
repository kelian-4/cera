use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const PLUGIN_MANIFEST: &str = r#"{
  "name": "Finder",
  "description": "bundled finder",
  "help": "bundled-finder-help",
  "command": "python3",
  "args": ["main.py"]
}
"#;

fn fresh_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cera_bundled_{}_{}", name, std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn install_layout(root: &Path) -> PathBuf {
    let bin = root.join("bin");
    let plugin = root.join("share/cera/plugins/finder");
    fs::create_dir_all(&bin).unwrap();
    fs::create_dir_all(&plugin).unwrap();
    fs::write(plugin.join("plugin.json"), PLUGIN_MANIFEST).unwrap();
    fs::write(plugin.join("main.py"), "print('finder')\n").unwrap();
    let binary = bin.join("cera");
    fs::copy(env!("CARGO_BIN_EXE_cera"), &binary).unwrap();
    binary
}

fn run_help(binary: &Path, home: &Path) -> Output {
    Command::new(binary)
        .arg("--help")
        .env("HOME", home)
        .env_remove("XDG_CONFIG_HOME")
        .output()
        .unwrap()
}

#[test]
fn first_run_installs_bundled_plugins_and_enables_them() {
    let root = fresh_dir("first_run");
    let binary = install_layout(&root);
    let home = root.join("home");
    fs::create_dir_all(&home).unwrap();

    let output = run_help(&binary, &home);

    assert!(output.status.success());
    assert!(home.join(".config/cera/config.json").is_file());
    assert!(home
        .join(".config/cera/plugins/finder/plugin.json")
        .is_file());
    assert!(home.join(".config/cera/plugins/finder/main.py").is_file());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("bundled-finder-help"), "{stdout}");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn existing_config_keeps_its_plugins_untouched() {
    let root = fresh_dir("existing");
    let binary = install_layout(&root);
    let home = root.join("home");
    fs::create_dir_all(home.join(".config/cera")).unwrap();
    fs::write(home.join(".config/cera/config.json"), "{}").unwrap();

    run_help(&binary, &home);

    assert!(!home.join(".config/cera/plugins").exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn binary_without_bundled_plugins_creates_no_plugins_directory() {
    let root = fresh_dir("without");
    let bin = root.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let binary = bin.join("cera");
    fs::copy(env!("CARGO_BIN_EXE_cera"), &binary).unwrap();
    let home = root.join("home");
    fs::create_dir_all(&home).unwrap();

    run_help(&binary, &home);

    assert!(home.join(".config/cera/config.json").is_file());
    assert!(!home.join(".config/cera/plugins").exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn new_version_updates_installed_plugins() {
    let root = fresh_dir("update");
    let binary = install_layout(&root);
    let home = root.join("home");
    fs::create_dir_all(&home).unwrap();
    run_help(&binary, &home);
    let plugins = home.join(".config/cera/plugins");
    fs::write(plugins.join("finder/main.py"), "stale").unwrap();
    fs::write(plugins.join(".cera-version"), "0.0.1").unwrap();

    run_help(&binary, &home);

    assert_eq!(
        fs::read_to_string(plugins.join("finder/main.py")).unwrap(),
        "print('finder')\n"
    );
    assert_eq!(
        fs::read_to_string(plugins.join(".cera-version")).unwrap(),
        env!("CARGO_PKG_VERSION")
    );
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn same_version_keeps_local_changes() {
    let root = fresh_dir("same_version");
    let binary = install_layout(&root);
    let home = root.join("home");
    fs::create_dir_all(&home).unwrap();
    run_help(&binary, &home);
    let plugins = home.join(".config/cera/plugins");
    fs::write(plugins.join("finder/main.py"), "edited").unwrap();

    run_help(&binary, &home);

    assert_eq!(
        fs::read_to_string(plugins.join("finder/main.py")).unwrap(),
        "edited"
    );
    let _ = fs::remove_dir_all(&root);
}
