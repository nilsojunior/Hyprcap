use chrono::Local;
use hyprshade::{disable_shader, enable_shader, get_current_shader};
use slurp::{select_monitor, select_region, select_window};
use std::process::{exit, Command};

mod hyprshade;
mod slurp;

fn main() {
    // let region = select_region();
    // let region = select_monitor();
    let name = name_file();
    let region = select_window();
    move_cursor();
    take_screenshot(&region, &name);
}

fn take_screenshot(region: &String, name: &String) {
    let grim = Command::new("grim")
        .args(["-g", region, name])
        .status()
        .expect("Failed to run grim");

    if grim.success() {
        println!("{} saved to: ", name);
    } else {
        eprintln!("Failed to take screenshot");
        exit(1);
    }
}

fn name_file() -> String {
    let time = Local::now().format("%Y-%m-%d_%I:%M:%p").to_string();
    let name = format!("{}_screenshooter.png", time);
    name
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
