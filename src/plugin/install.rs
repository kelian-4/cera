use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub fn bundled_plugins_directory() -> Option<PathBuf> {
    let executable = std::env::current_exe().ok()?;
    bundled_plugins_directory_for(&executable)
}

pub fn bundled_plugins_directory_for(executable: &Path) -> Option<PathBuf> {
    let binary_directory = executable.parent()?;
    [
        binary_directory.join("../share/cera/plugins"),
        binary_directory.join("plugins"),
    ]
    .into_iter()
    .find(|directory| directory.is_dir())
}

fn copy_directory(source: &Path, destination: &Path) -> std::io::Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_directory(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target)?;
            let mut permissions = fs::metadata(&target)?.permissions();
            permissions.set_mode(permissions.mode() | 0o200);
            fs::set_permissions(&target, permissions)?;
        }
    }
    Ok(())
}

const VERSION_FILE: &str = ".cera-version";

fn replace_plugin(source: &Path, target: &Path) -> std::io::Result<()> {
    let name = target.file_name().unwrap_or_default().to_string_lossy();
    let staging = target.with_file_name(format!(".{}.new", name));
    let _ = fs::remove_dir_all(&staging);
    if let Err(error) = copy_directory(source, &staging) {
        let _ = fs::remove_dir_all(&staging);
        return Err(error);
    }
    if target.exists() {
        fs::remove_dir_all(target)?;
    }
    fs::rename(&staging, target)
}

pub fn install_bundled_plugins(source: &Path, destination: &Path, version: &str) {
    let Ok(entries) = fs::read_dir(source) else {
        return;
    };
    if fs::create_dir_all(destination).is_err() {
        return;
    }
    let mut all_installed = true;
    for entry in entries.flatten() {
        let plugin_directory = entry.path();
        if !plugin_directory.join("plugin.json").is_file() {
            continue;
        }
        let target = destination.join(entry.file_name());
        if replace_plugin(&plugin_directory, &target).is_err() {
            all_installed = false;
        }
    }
    if all_installed {
        let _ = fs::write(destination.join(VERSION_FILE), version);
    }
}

pub fn update_bundled_plugins(source: &Path, destination: &Path, version: &str) {
    if !destination.is_dir() {
        return;
    }
    let installed_version = fs::read_to_string(destination.join(VERSION_FILE)).unwrap_or_default();
    if installed_version.trim() == version {
        return;
    }
    install_bundled_plugins(source, destination, version);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("cera_install_{}_{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_plugin(root: &Path, name: &str) {
        let dir = root.join(name);
        fs::create_dir_all(dir.join("nested")).unwrap();
        fs::write(dir.join("plugin.json"), "{}").unwrap();
        fs::write(dir.join("main.py"), "print()").unwrap();
        fs::write(dir.join("nested").join("data.json"), "[]").unwrap();
    }

    #[test]
    fn test_install_copies_plugins_recursively() {
        let root = temp_dir("copy");
        let source = root.join("source");
        let destination = root.join("destination");
        write_plugin(&source, "finder");
        write_plugin(&source, "grep");
        install_bundled_plugins(&source, &destination, "1.0.0");
        assert!(destination.join("finder/plugin.json").is_file());
        assert!(destination.join("finder/main.py").is_file());
        assert!(destination.join("finder/nested/data.json").is_file());
        assert!(destination.join("grep/plugin.json").is_file());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_install_writes_the_version_marker() {
        let root = temp_dir("marker");
        let source = root.join("source");
        let destination = root.join("destination");
        write_plugin(&source, "finder");
        install_bundled_plugins(&source, &destination, "1.2.3");
        assert_eq!(
            fs::read_to_string(destination.join(VERSION_FILE)).unwrap(),
            "1.2.3"
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_install_skips_entries_without_manifest() {
        let root = temp_dir("skip");
        let source = root.join("source");
        write_plugin(&source, "finder");
        fs::create_dir_all(source.join("not-a-plugin")).unwrap();
        fs::write(source.join("install.sh"), "#!/bin/sh").unwrap();
        let destination = root.join("destination");
        install_bundled_plugins(&source, &destination, "1.0.0");
        assert!(destination.join("finder").is_dir());
        assert!(!destination.join("not-a-plugin").exists());
        assert!(!destination.join("install.sh").exists());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_install_replaces_an_existing_bundled_plugin() {
        let root = temp_dir("replace");
        let source = root.join("source");
        let destination = root.join("destination");
        write_plugin(&source, "finder");
        fs::create_dir_all(destination.join("finder")).unwrap();
        fs::write(destination.join("finder/main.py"), "old").unwrap();
        fs::write(destination.join("finder/stale.py"), "old").unwrap();
        install_bundled_plugins(&source, &destination, "1.0.0");
        assert_eq!(
            fs::read_to_string(destination.join("finder/main.py")).unwrap(),
            "print()"
        );
        assert!(!destination.join("finder/stale.py").exists());
        assert!(!destination.join(".finder.new").exists());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_install_keeps_custom_plugins() {
        let root = temp_dir("custom");
        let source = root.join("source");
        let destination = root.join("destination");
        write_plugin(&source, "finder");
        write_plugin(&destination, "mine");
        install_bundled_plugins(&source, &destination, "1.0.0");
        assert!(destination.join("mine/plugin.json").is_file());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_install_makes_copied_files_writable() {
        let root = temp_dir("writable");
        let source = root.join("source");
        write_plugin(&source, "finder");
        let mut permissions = fs::metadata(source.join("finder/main.py"))
            .unwrap()
            .permissions();
        permissions.set_mode(0o444);
        fs::set_permissions(source.join("finder/main.py"), permissions).unwrap();
        let destination = root.join("destination");
        install_bundled_plugins(&source, &destination, "1.0.0");
        let mode = fs::metadata(destination.join("finder/main.py"))
            .unwrap()
            .permissions()
            .mode();
        assert_ne!(mode & 0o200, 0);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_install_with_missing_source_does_nothing() {
        let root = temp_dir("missing");
        let destination = root.join("destination");
        install_bundled_plugins(&root.join("nope"), &destination, "1.0.0");
        assert!(!destination.exists());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_update_replaces_plugins_when_the_version_changes() {
        let root = temp_dir("update_new");
        let source = root.join("source");
        let destination = root.join("destination");
        write_plugin(&source, "finder");
        install_bundled_plugins(&source, &destination, "1.0.0");
        fs::write(source.join("finder/main.py"), "print('v2')").unwrap();
        update_bundled_plugins(&source, &destination, "2.0.0");
        assert_eq!(
            fs::read_to_string(destination.join("finder/main.py")).unwrap(),
            "print('v2')"
        );
        assert_eq!(
            fs::read_to_string(destination.join(VERSION_FILE)).unwrap(),
            "2.0.0"
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_update_does_nothing_when_the_version_is_the_same() {
        let root = temp_dir("update_same");
        let source = root.join("source");
        let destination = root.join("destination");
        write_plugin(&source, "finder");
        install_bundled_plugins(&source, &destination, "1.0.0");
        fs::write(destination.join("finder/main.py"), "edited").unwrap();
        update_bundled_plugins(&source, &destination, "1.0.0");
        assert_eq!(
            fs::read_to_string(destination.join("finder/main.py")).unwrap(),
            "edited"
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_update_handles_plugins_installed_without_a_marker() {
        let root = temp_dir("update_unmarked");
        let source = root.join("source");
        let destination = root.join("destination");
        write_plugin(&source, "finder");
        fs::create_dir_all(destination.join("finder")).unwrap();
        fs::write(destination.join("finder/main.py"), "old").unwrap();
        update_bundled_plugins(&source, &destination, "1.0.0");
        assert_eq!(
            fs::read_to_string(destination.join("finder/main.py")).unwrap(),
            "print()"
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_update_never_creates_a_missing_plugins_directory() {
        let root = temp_dir("update_missing");
        let source = root.join("source");
        let destination = root.join("destination");
        write_plugin(&source, "finder");
        update_bundled_plugins(&source, &destination, "1.0.0");
        assert!(!destination.exists());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_bundled_directory_found_in_share_layout() {
        let root = temp_dir("share");
        fs::create_dir_all(root.join("bin")).unwrap();
        fs::create_dir_all(root.join("share/cera/plugins")).unwrap();
        let found = bundled_plugins_directory_for(&root.join("bin/cera")).unwrap();
        assert!(found.ends_with("share/cera/plugins"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_bundled_directory_found_next_to_binary() {
        let root = temp_dir("archive");
        fs::create_dir_all(root.join("plugins")).unwrap();
        let found = bundled_plugins_directory_for(&root.join("cera")).unwrap();
        assert!(found.ends_with("plugins"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_bundled_directory_absent() {
        let root = temp_dir("absent");
        assert!(bundled_plugins_directory_for(&root.join("bin/cera")).is_none());
        let _ = fs::remove_dir_all(&root);
    }
}
