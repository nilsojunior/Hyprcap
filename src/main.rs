use arguments::*;
use clap::Parser;
use hyprshade::*;
use screenshot::*;
use slurp::*;
use std::path::PathBuf;
use utils::*;

mod arguments;
mod hyprshade;
mod screenshot;
mod slurp;
mod utils;

fn main() {
    let args = Cli::parse();

    let region = match args.mode.as_str() {
        "region" => select_region(),
        "window" => select_window(),
        "monitor" => select_monitor(),
        "active" => select_active_window(),
        _ => {
            eprintln!("Mode does not exist");
            return;
        }
    };

    if args.only_clipboard {
        only_clipboard_screenshot(&region);
        if !args.silent {
            send_clipboard_notification();
        }
    } else {
        let name = if !args.filename.is_empty() {
            args.filename
        } else {
            name_file()
        };

        let dir = if let Some(dir) = &args.directory {
            if dir.is_dir() {
                dir.as_path()
            } else {
                eprintln!("{} does not exist or is not a directory", dir.display());
                return;
            }
        } else {
            &get_screenshots_dir_path()
        };

        let path = dir.join(name);
        println!("{}", path.display());
        let path = path
            .to_str()
            .expect("Error converting Path to String")
            .to_string();

        take_screenshot(&region, &path);
        if !args.silent {
            send_notification(&path);
        }
    }

    if args.move_cursor {
        move_cursor();
    }
}
