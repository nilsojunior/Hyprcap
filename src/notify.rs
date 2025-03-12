use std::process::Command;

pub struct Notify;

impl Notify {
    pub fn screenshot(path: &String) {
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

    pub fn screenshot_only_clipboard() {
        let notification = Command::new("notify-send")
            .args(["Screenshot saved", &format!("Image copied to clipboard")])
            .status()
            .expect("Failed to run notify-send");

        if !notification.success() {
            eprintln!("Failed to send notification");
        }
    }
}
