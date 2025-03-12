use args::*;
use clap::Parser;
use grab::*;
use hyprshade::*;
use notify::*;
use screenshot::*;
use std::path::PathBuf;
use utils::*;

mod args;
mod grab;
mod hyprshade;
mod notify;
mod screenshot;
mod utils;

fn main() {
    let args = Cli::parse();

    let region = match args.mode.as_str() {
        "region" => Grab::region(),
        "window" => Grab::window(),
        "monitor" => Grab::monitor(),
        // "active" => select_active_window(),
        _ => {
            eprintln!("Mode does not exist");
            return;
        }
    };

    let name = if !args.filename.is_empty() {
        args.filename
    } else {
        Screenshot::name_file()
    };

    let dir = if let Some(dir) = &args.directory {
        if dir.is_dir() {
            dir.as_path()
        } else {
            eprintln!("{} does not exist or is not a directory", dir.display());
            return;
        }
    } else {
        &Screenshot::get_dir_path()
    };

    let path = dir.join(name);
    let path = path
        .to_str()
        .expect("Error converting Path to String")
        .to_string();

    if args.only_clipboard {
        if args.disable_shader {
            Screenshot::only_clipboard_without_shader(&region);
        } else {
            Screenshot::only_clipboard(&region);
        }
        if !args.silent {
            Notify::screenshot_only_clipboard();
        }
        return;
    }

    if args.disable_shader {
        Screenshot::save_without_shader(&region, &path);
    } else {
        Screenshot::save(&region, &path);
    }
    if !args.silent {
        Notify::screenshot(&path);
    }
    let cursorpos = get_cursor_pos();

    if args.move_cursor {
        move_cursor("5000", "5000");
    }
}
