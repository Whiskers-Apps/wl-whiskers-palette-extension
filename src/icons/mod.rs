use whiskers_launcher_rs::api::extensions::get_extension_dir;

pub fn get_icon(icon: impl Into<String>) -> String {
    let mut path = get_extension_dir("lighttigerxiv/whiskers-palette").unwrap();
    path.push(format!("src/icons/{}.svg", icon.into()));

    path.into_os_string().into_string().unwrap()
}
