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
    let bash = Command::new("bash")
        .args(["-c", "hyprctl clients -j | jq -r '.[] | select(.workspace.id != null) | .at,.size' | jq -s 'add' | jq '_nwise(4)' | jq -r '\"\\(.[0]),\\(.[1]) \\(.[2])x\\(.[3])\"' | slurp"])
        .output()
        .expect("Failed to run bash command");

    if !bash.status.success() {
        println!("Failed to select window");
        exit(1);
    }

    let bash = String::from_utf8_lossy(&bash.stdout).trim().to_string();
    bash
}
