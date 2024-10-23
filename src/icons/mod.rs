use std::path::PathBuf;

use whiskers_launcher_core::features::extensions::get_extension_dir;

pub fn get_icon(icon: impl Into<String>) -> PathBuf {
    let mut path = get_extension_dir("lighttigerxiv/whiskers-palette").unwrap();
    path.push(format!("src/icons/{}.svg", icon.into()));
    path
}
