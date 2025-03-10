use std::process::{exit, Command};

pub fn select_region() -> String {
    let slurp = Command::new("slurp").output().expect("Failed to run slurp");

    if !slurp.status.success() {
        eprintln!("Failed to capture region");
    }

    let slurp = String::from_utf8_lossy(&slurp.stdout).trim().to_string();

    if slurp.is_empty() {
        eprintln!("No region was selected");
        exit(1);
    }
    slurp
}

pub fn select_monitor() -> String {
    let slurp = Command::new("slurp")
        .arg("-o")
        .output()
        .expect("Ffailed to run slurp");

    if !slurp.status.success() {
        eprintln!("Failed to capture region");
    }

    let slurp = String::from_utf8_lossy(&slurp.stdout).trim().to_string();

    if slurp.is_empty() {
        eprintln!("No region was selected");
        exit(1);
    }
    slurp
}

pub fn select_window() -> String {
    let script_path = "~/personal/screenshot/select-window.sh";
    let bash = Command::new("bash")
        .args(["-c", script_path])
        .output()
        .expect("Failed to run bash commands");

    if !bash.status.success() {
        eprintln!("Failed to select window");
        exit(1);
    }

    let bash = String::from_utf8_lossy(&bash.stdout).trim().to_string();
    bash
}
