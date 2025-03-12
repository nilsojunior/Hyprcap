use std::process::{exit, Command};

pub fn move_cursor() {
    let wlrctl = Command::new("wlrctl")
        .args(["pointer", "move", "5000", "5000"])
        .status()
        .expect("Failed to run wlrctl");

    if !wlrctl.success() {
        eprintln!("Failed to move cursor");
        exit(1);
    }
}

pub fn send_notification(path: &String) {
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
pub fn send_clipboard_notification() {
    let notification = Command::new("notify-send")
        .args(["Screenshot saved", &format!("Image copied to clipboard")])
        .status()
        .expect("Failed to run notify-send");

    if !notification.success() {
        eprintln!("Failed to send notification");
    }
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
