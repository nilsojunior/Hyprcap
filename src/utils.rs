use std::process::{exit, Command};

pub fn move_cursor(x: &str, y: &str) {
    let hyprctl = Command::new("hyprctl")
        .args(["dispatch", "movecursor", x, y])
        .status()
        .expect("Failed to run hyprctl");

    if !hyprctl.success() {
        eprintln!("Failed to move cursor");
        exit(1);
    }
}

pub fn get_cursor_pos() -> String {
    let hyprctl = Command::new("hyprctl")
        .arg("cursorpos")
        .output()
        .expect("Failed to run hyprctl");

    if !hyprctl.status.success() {
        eprintln!("Failed to get cursor position");
        exit(1);
    }

    let hyprctl = String::from_utf8_lossy(&hyprctl.stdout).trim().to_string();
    println!("{}", hyprctl);

    // Remove the coma
    hyprctl.replace(",", "")
}

pub fn copy_to_clipboard(path: &String) {
    let copy = Command::new("bash")
        .args(["-c", &format!("wl-copy < {}", path)])
        .status()
        .expect("Failed to run wl-copy");

    if !copy.success() {
        eprintln!("Failed to copy image");
        exit(1);
    }
}
