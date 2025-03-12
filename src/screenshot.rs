use chrono::Local;
use std::env;
use std::path::PathBuf;
use std::process::{exit, Command};

pub fn get_screenshots_dir_path() -> PathBuf {
    if let Ok(default) = env::var("SCREENSHOOTER_DIR") {
        return PathBuf::from(default);
    }
    if let Ok(xdg) = env::var("XDG_PICTURES_DIR") {
        return PathBuf::from(xdg);
    } else {
        return PathBuf::from(env::var("HOME").unwrap());
    }
}

pub fn name_file() -> String {
    let time = Local::now().format("%Y-%m-%d_%I:%M:%p").to_string();
    let name = time + "_screenshooter.png";
    name
}

pub fn take_screenshot(region: &String, path: &String) {
    let grim = Command::new("grim")
        .args(["-g", region, path])
        .status()
        .expect("Failed to run grim");

    if !grim.success() {
        eprintln!("Failed to take screenshot");
        exit(1);
    }

    println!("Image saved to: {}", path);
}

pub fn only_clipboard_screenshot(region: &String) {
    let bash = Command::new("bash")
        .args(["-c", &format!("grim -g \"{}\" - | wl-copy", region)])
        .status()
        .expect("Failed to run bash command");

    if !bash.success() {
        eprintln!("Failed to take only clipboard screenshot");
        exit(1);
    }
}
