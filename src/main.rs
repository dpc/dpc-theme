use clap::{Parser, Subcommand};
use palette::{Clamp, FromColor, Oklch, Srgb};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate Wezterm color scheme
    Wezterm,
}

pub const RED_HUE: f32 = 0.0;
pub const GREEN_HUE: f32 = 145.0;
pub const YELLOW_HUE: f32 = 109.0;
pub const BLUE_HUE: f32 = 264.0;
pub const MAGENTA_HUE: f32 = 300.0;
pub const CYAN_HUE: f32 = 210.0;

pub const CHROMA_MAX: f32 = 0.37;
pub const CHROMA_STD: f32 = 0.2;

type Color = Srgb<u8>;
struct Colors {
    foreground: Color,
    background: Color,
    black: Color,
    red: Color,
    green: Color,
    yellow: Color,
    blue: Color,
    magenta: Color,
    cyan: Color,
    white: Color,
    black_bright: Color,
    red_bright: Color,
    green_bright: Color,
    yellow_bright: Color,
    blue_bright: Color,
    magenta_bright: Color,
    cyan_bright: Color,
    white_bright: Color,
}
fn generate_colors() -> Colors {
    let mut colors = Colors {
        foreground: Srgb::new(255, 255, 255),
        background: Srgb::new(0, 0, 0),
        black: Srgb::new(0, 0, 0),
        red: Srgb::new(0, 0, 0),
        green: Srgb::new(0, 0, 0),
        yellow: Srgb::new(0, 0, 0),
        blue: Srgb::new(0, 0, 0),
        magenta: Srgb::new(0, 0, 0),
        cyan: Srgb::new(0, 0, 0),
        white: Srgb::new(0, 0, 0),
        black_bright: Srgb::new(0, 0, 0),
        red_bright: Srgb::new(0, 0, 0),
        green_bright: Srgb::new(0, 0, 0),
        yellow_bright: Srgb::new(0, 0, 0),
        blue_bright: Srgb::new(0, 0, 0),
        magenta_bright: Srgb::new(0, 0, 0),
        cyan_bright: Srgb::new(0, 0, 0),
        white_bright: Srgb::new(0, 0, 0),
    };

    for (h_name, l, hue, c) in [
        ("bg", 0.30, 221.11, 0.039),
        ("fg", 0.8, 0.0, 0.0),
        ("black", 0.22, CYAN_HUE, 0.03),
        ("red", 0.6, RED_HUE, CHROMA_STD + 0.1),
        ("green", 0.8, GREEN_HUE, CHROMA_STD),
        ("yellow", 0.99, YELLOW_HUE, CHROMA_STD),
        ("blue", 0.9, BLUE_HUE, CHROMA_STD),
        ("magenta", 0.7, MAGENTA_HUE, CHROMA_STD),
        ("cyan", 0.8, CYAN_HUE, CHROMA_STD),
        ("white", 1.45, GREEN_HUE, 0.02),
    ] {
        for (l_name, l, c) in [("", l * 0.65, c), ("b.", l, c - 0.02)] {
            let oklch = Oklch::new(l, c, hue);
            let srgb = Srgb::<f32>::from_color(oklch.clamp());
            let srgb = Srgb::<u8>::from_format(srgb);
            match (l_name, h_name) {
                ("", "bg") => colors.background = srgb,
                ("b.", "fg") => colors.foreground = srgb,
                ("", "black") => colors.black = srgb,
                ("", "red") => colors.red = srgb,
                ("", "green") => colors.green = srgb,
                ("", "yellow") => colors.yellow = srgb,
                ("", "blue") => colors.blue = srgb,
                ("", "magenta") => colors.magenta = srgb,
                ("", "cyan") => colors.cyan = srgb,
                ("", "white") => colors.white = srgb,
                ("b.", "black") => colors.black_bright = srgb,
                ("b.", "red") => colors.red_bright = srgb,
                ("b.", "green") => colors.green_bright = srgb,
                ("b.", "yellow") => colors.yellow_bright = srgb,
                ("b.", "blue") => colors.blue_bright = srgb,
                ("b.", "magenta") => colors.magenta_bright = srgb,
                ("b.", "cyan") => colors.cyan_bright = srgb,
                ("b.", "white") => colors.white_bright = srgb,
                _ => {}
            }
        }
    }
    colors
}

fn print_color(name: &str, normal: Color, bright: Color) {
    let normal_bg = anstyle::RgbColor(normal.red, normal.green, normal.blue).render_bg();
    let normal_fg = anstyle::RgbColor(normal.red, normal.green, normal.blue).render_fg();
    let bright_bg = anstyle::RgbColor(bright.red, bright.green, bright.blue).render_bg();
    let bright_fg = anstyle::RgbColor(bright.red, bright.green, bright.blue).render_fg();

    println!(
        "{}{:10}#{normal:X}{} {}{:10}#{bright:X}{}",
        normal_bg,
        name,
        anstyle::Reset,
        bright_bg,
        format!("{}b", name),
        anstyle::Reset,
    );
    println!(
        "{}{:10}#{normal:X}{} {}{:10}#{bright:X}{}",
        normal_fg,
        name,
        anstyle::Reset,
        bright_fg,
        format!("{}b", name),
        anstyle::Reset,
    );
}

fn generate_wezterm_config(colors: Colors) {
    println!("[colors]");
    println!("ansi = [");
    println!("    \"#{:X}\",", colors.black);
    println!("    \"#{:X}\",", colors.red);
    println!("    \"#{:X}\",", colors.green);
    println!("    \"#{:X}\",", colors.yellow);
    println!("    \"#{:X}\",", colors.blue);
    println!("    \"#{:X}\",", colors.magenta);
    println!("    \"#{:X}\",", colors.cyan);
    println!("    \"#{:X}\",", colors.white);
    println!("]");
    println!("background = \"#{:X}\"", colors.background);
    println!("brights = [");
    println!("    \"#{:X}\",", colors.black_bright);
    println!("    \"#{:X}\",", colors.red_bright);
    println!("    \"#{:X}\",", colors.green_bright);
    println!("    \"#{:X}\",", colors.yellow_bright);
    println!("    \"#{:X}\",", colors.blue_bright);
    println!("    \"#{:X}\",", colors.magenta_bright);
    println!("    \"#{:X}\",", colors.cyan_bright);
    println!("    \"#{:X}\",", colors.white_bright);
    println!("]");
    println!("cursor_bg = \"#{:X}\"", colors.white);
    println!("cursor_border = \"#{:X}\"", colors.white);
    println!("cursor_fg = \"#{:X}\"", colors.black);
    println!("foreground = \"#{:X}\"", colors.foreground);
    println!("selection_bg = \"#{:X}\"", colors.white);
    println!("selection_fg = \"#{:X}\"", colors.black);
    println!();
    println!("[metadata]");
    println!("aliases = [\"dpc\"]");
    println!("author = \"dpc (http://github.com/dpc)\"");
    println!("name = \"dpc\"");
    println!("origin_url = \"https://github.com/dpc/dpc-theme\"");
    println!("wezterm_version = \"wezterm 0-unstable-2025-01-24\"");
    println!();
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Wezterm) => {
            let colors = generate_colors();
            generate_wezterm_config(colors);
        }
        None => {
            let colors = generate_colors();
            print_color("fg", colors.foreground, colors.foreground);
            print_color("bg", colors.background, colors.background);
            print_color("black", colors.black, colors.black_bright);
            print_color("red", colors.red, colors.red_bright);
            print_color("green", colors.green, colors.green_bright);
            print_color("yellow", colors.yellow, colors.yellow_bright);
            print_color("blue", colors.blue, colors.blue_bright);
            print_color("magenta", colors.magenta, colors.magenta_bright);
            print_color("cyan", colors.cyan, colors.cyan_bright);
            print_color("white", colors.white, colors.white_bright);
        }
    }
}
