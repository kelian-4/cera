use std::path::PathBuf;
use std::process::{Command, Output};

fn home_for(name: &str) -> PathBuf {
    let home = std::env::temp_dir().join(format!("cera-version-{}-{}", name, std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).unwrap();
    home
}

fn run(home: &PathBuf, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_cera"))
        .args(args)
        .env("HOME", home)
        .env_remove("XDG_CONFIG_HOME")
        .output()
        .unwrap()
}

fn expected_output() -> String {
    format!("cera {}\n", env!("CARGO_PKG_VERSION"))
}

#[test]
fn version_prints_name_and_version() {
    let output = run(&home_for("plain"), &["--version"]);
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output());
}

#[test]
fn version_wins_over_other_options() {
    let output = run(&home_for("options"), &["-g", "x", "--version"]);
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected_output());
}

#[test]
fn version_does_not_create_the_config() {
    let home = home_for("config");
    run(&home, &["--version"]);
    assert!(!home.join(".config").exists());
}
