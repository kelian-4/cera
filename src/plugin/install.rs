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

pub fn install_bundled_plugins(source: &Path, destination: &Path) {
    let Ok(entries) = fs::read_dir(source) else {
        return;
    };
    for entry in entries.flatten() {
        let plugin_directory = entry.path();
        if !plugin_directory.join("plugin.json").is_file() {
            continue;
        }
        let target = destination.join(entry.file_name());
        if target.exists() {
            continue;
        }
        if copy_directory(&plugin_directory, &target).is_err() {
            let _ = fs::remove_dir_all(&target);
        }
    }
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
        install_bundled_plugins(&source, &destination);
        assert!(destination.join("finder/plugin.json").is_file());
        assert!(destination.join("finder/main.py").is_file());
        assert!(destination.join("finder/nested/data.json").is_file());
        assert!(destination.join("grep/plugin.json").is_file());
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
        install_bundled_plugins(&source, &destination);
        assert!(destination.join("finder").is_dir());
        assert!(!destination.join("not-a-plugin").exists());
        assert!(!destination.join("install.sh").exists());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_install_never_overwrites_an_existing_plugin() {
        let root = temp_dir("keep");
        let source = root.join("source");
        let destination = root.join("destination");
        write_plugin(&source, "finder");
        fs::create_dir_all(destination.join("finder")).unwrap();
        fs::write(destination.join("finder/main.py"), "mine").unwrap();
        install_bundled_plugins(&source, &destination);
        assert_eq!(
            fs::read_to_string(destination.join("finder/main.py")).unwrap(),
            "mine"
        );
        assert!(!destination.join("finder/plugin.json").exists());
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
        install_bundled_plugins(&source, &destination);
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
        install_bundled_plugins(&root.join("nope"), &destination);
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
