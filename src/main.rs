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

    let cursorpos: Option<String> = if args.move_cursor {
        Some(get_cursor_pos())
    } else {
        None
    };

    if cursorpos.is_some() {
        move_cursor("1000", "1000");
    }

    let shader: Option<String> = if args.disable_shader {
        Some(Hyprshade::get_current_shader())
    } else {
        None
    };

    if shader.is_some() {
        Hyprshade::disable_shader();
    }

    if args.only_clipboard {
        Screenshot::only_clipboard(&region);
        if !args.silent {
            Notify::screenshot_only_clipboard();
        }
    } else {
        Screenshot::save(&region, &path);
        copy_to_clipboard(&path);
        if !args.silent {
            Notify::screenshot(&path);
        }
    }

    if cursorpos.is_some() {
        let cursorpos: String = cursorpos.expect("Cursorpos should be a String");
        let mut cursorpos = cursorpos.split_whitespace();

        let x = cursorpos.next().unwrap();
        let y = cursorpos.next().unwrap();
        move_cursor(x, y);
    }

    if shader.is_some() {
        let shader: String = shader.expect("Shader should be a String");
        Hyprshade::enable_shader(&shader);
    }
}
