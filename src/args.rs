use crate::name_file;
use crate::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(version = "1.0", about = "Screenshot tool for Hyprland")]
pub struct Cli {
    #[arg(short = 'm', long, help = "Specify mode [region, window, monitor]")]
    pub mode: String,

    #[arg(short = 'f', long = "filename", default_value_t = name_file(), help = "Specify filename")]
    pub filename: String,

    #[arg(
        short = 'd',
        long = "output-directory",
        help = "Specify directory to save the screenshot"
    )]
    pub directory: Option<PathBuf>,

    #[arg(
        short = 'c',
        long = "only-clipboard",
        help = "Only copy the screenshot to the clipboard (without saving)"
    )]
    pub only_clipboard: bool,

    #[arg(
        short = 'p',
        long = "move-cursor",
        help = "Move cursor out of the screen before taking screenshot"
    )]
    pub move_cursor: bool,

    #[arg(short = 's', long = "silent", help = "Don't send notification")]
    pub silent: bool,

    #[arg(
        short = 't',
        long = "disable-shader",
        help = "Disable current shader for the screenshot"
    )]
    pub disable_shader: bool,
}
