use clap::Parser;
use hyprshade::*;
use screenshot::*;
use slurp::*;
use std::env;
use utils::*;

mod hyprshade;
mod screenshot;
mod slurp;
mod utils;

#[derive(Parser)]
#[command(version = "1.0", about = "Screenshot tool for Hyprland")]
struct Cli {
    #[arg(short = 'm', long)]
    mode: String,

    #[arg(short = 'c', long = "only-clipboard")]
    only_clipboard: bool,

    #[arg(short = 'p', long = "move-cursor")]
    move_cursor: bool,

    #[arg(short = 'n', long = "send-notification")]
    send_notification: bool,
}

fn main() {
    let args = Cli::parse();

    let region = match args.mode.as_str() {
        "region" => select_region(),
        "window" => select_window(),
        "monitor" => select_monitor(),
        _ => {
            eprintln!("Mode does not exist");
            return;
        }
    };

    if args.only_clipboard {
        only_clipboard_screenshot(&region);
    }

    if args.move_cursor {
        move_cursor();
    }

    if args.send_notification {}
}
