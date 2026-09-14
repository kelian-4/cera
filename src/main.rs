fn main() {
    let mut editor = match cera::editor::core::Editor::new() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Erreur d'initialisation : {}", e);
            std::process::exit(1);
        }
    };

    let args: Vec<String> = std::env::args().collect();
    for filename in &args[1..] {
        editor.open_file(filename);
    }

    editor.run();
}
