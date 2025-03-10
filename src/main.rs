use chrono::Local;
use hyprshade::{disable_shader, enable_shader, get_current_shader};
use slurp::{select_monitor, select_region, select_window};
use std::env;
use std::process::{exit, Command};

mod hyprshade;
mod slurp;

fn main() {
    // let region = select_region();
    // let region = select_monitor();
    let name = name_file();
    let region = select_region();
    move_cursor();
    take_screenshot(&region, &name);
    send_notification(&name);
    // copy_to_clipboard(&name);
}

fn take_screenshot(region: &String, path: &String) {
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

fn only_clipboard_screenshot(region: &String) {
    let bash = Command::new("bash")
        .args(["-c", &format!("grim -g \"{}\" - | wl-copy", region)])
        .status()
        .expect("Failed to run bash command");

    if !bash.success() {
        eprintln!("Failed to take only clipboard screenshot");
        exit(1);
    }
}

fn move_cursor() {
    let wlrctl = Command::new("wlrctl")
        .args(["pointer", "move", "5000", "5000"])
        .status()
        .expect("Failed to run wlrctl");

    if !wlrctl.success() {
        eprintln!("Failed to move cursor");
        exit(1);
    }
}

fn copy_to_clipboard(file: &String) {
    let copy = Command::new("bash")
        .args(["-c", &format!("wl-copy < {}", file)])
        .status()
        .expect("Failed to run wl-copy");

    if !copy.success() {
        eprintln!("Failed to copy image");
        exit(1);
    }
}

fn get_screenshots_dir_path() -> String {
    if let Ok(default) = env::var("SCREENSHOOTER_DIR") {
        return default;
    }
    if let Ok(xdg) = env::var("XDG_PICTURES_DIR") {
        return xdg;
    } else {
        return env::var("HOME").unwrap();
    }
}

fn name_file() -> String {
    let time = Local::now().format("%Y-%m-%d_%I:%M:%p").to_string();

    let dir_path = get_screenshots_dir_path();
    let name = format!("{}/{}_screenshooter.png", dir_path, time);
    name
}

fn send_notification(path: &String) {
    let notification = Command::new("notify-send")
        .args([
            "-i",
            path,
            "Screenshot saved",
            &format!("Image saved to {}", path),
        ])
        .status()
        .expect("Failed to run notify-send");

    if !notification.success() {
        eprintln!("Failed to send notification");
    }
}
