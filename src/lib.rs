use std::{fs, io::Write, path::Path};

use serde::Deserialize;

/// Simple greet command left in place for quick sanity checks.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Deserialize)]
struct FilePayload {
    #[serde(rename = "relativePath")]
    relative_path: String,
    contents: String,
}

/// Writes multiple files under a base directory, creating folders as needed.
#[tauri::command]
fn write_files(base_directory: String, files: Vec<FilePayload>) -> Result<(), String> {
    let base = Path::new(&base_directory);

    for file in files {
        let target_path = base.join(&file.relative_path);

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory {:?}: {}", parent, e))?;
        }

        let mut f = fs::File::create(&target_path)
            .map_err(|e| format!("Failed to create file {:?}: {}", target_path, e))?;
        f.write_all(file.contents.as_bytes())
            .map_err(|e| format!("Failed to write file {:?}: {}", target_path, e))?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, write_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
