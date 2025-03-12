use std::process::{exit, Command};

pub struct Hyprshade;

impl Hyprshade {
    pub fn get_current_shader() -> String {
        let hyprshade = Command::new("hyprshade")
            .arg("current")
            .output()
            .expect("Failed to run hyprshade");

        if !hyprshade.status.success() {
            eprintln!("Failed to get current shader");
            exit(1);
        }

        let hyprshade = String::from_utf8_lossy(&hyprshade.stdout)
            .trim()
            .to_string();

        hyprshade
    }

    pub fn disable_shader() {
        let hyprshade = Command::new("hyprshade")
            .arg("off")
            .status()
            .expect("Failed to run hyprshade");

        if !hyprshade.success() {
            eprintln!("Failed to disable shader");
            exit(1);
        }
    }

    pub fn enable_shader(shader: &String) {
        let hyprshade = Command::new("hyprshade")
            .args(["on", shader])
            .status()
            .expect("Failed to run hyprshade");

        if !hyprshade.success() {
            eprintln!("Failed to disable shader");
            exit(1);
        }
    }
}
