use crate::Hyprshade;
use chrono::Local;
use std::env;
use std::path::PathBuf;
use std::process::{exit, Command};

pub struct Screenshot;

impl Screenshot {
    pub fn get_dir_path() -> PathBuf {
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

    pub fn save(region: &String, path: &String) {
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

    pub fn save_without_shader(region: &String, path: &String) {
        let shader = Hyprshade::get_current_shader();
        Hyprshade::disable_shader();
        Screenshot::save(&region, &path);
        Hyprshade::enable_shader(&shader);
    }

    pub fn only_clipboard(region: &String) {
        let bash = Command::new("bash")
            .args(["-c", &format!("grim -g \"{}\" - | wl-copy", region)])
            .status()
            .expect("Failed to run bash command");

        if !bash.success() {
            eprintln!("Failed to take only clipboard screenshot");
            exit(1);
        }
    }

    pub fn only_clipboard_without_shader(region: &String) {
        let shader = Hyprshade::get_current_shader();
        Hyprshade::disable_shader();
        Screenshot::only_clipboard(&region);
        Hyprshade::enable_shader(&shader);
    }
}
