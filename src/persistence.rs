use std::{fs, path::Path};

pub fn home() -> String {
    let dir = dirs::home_dir();
    dir.map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn persistence_file() -> String {
    home() + "/.local/share/rsstig/read.json"
}

pub fn load_persistence() -> Vec<String> {
    fs::read_to_string(persistence_file())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or(Vec::new())
}

pub fn save_persistence(read_entries: Vec<String>) -> bool {
    match serde_json::to_string_pretty(&read_entries) {
        Ok(json) => {
            let path = persistence_file();
            if let Some(parent) = Path::new(&path).parent() {
                let _ = fs::create_dir_all(parent);
            }
            fs::write(&path, json).is_ok()
        },
        Err(_) => false,
    }
}
