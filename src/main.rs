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
fn generate_color_pair(l: f32, c: f32, hue: f32) -> (Color, Color) {
    let normal = {
        let oklch = Oklch::new(l * 0.65, c, hue);
        let srgb = Srgb::<f32>::from_color(oklch.clamp());
        Srgb::<u8>::from_format(srgb)
    };
    
    let bright = {
        let oklch = Oklch::new(l, c - 0.02, hue);
        let srgb = Srgb::<f32>::from_color(oklch.clamp());
        Srgb::<u8>::from_format(srgb)
    };
    
    (normal, bright)
}

fn generate_colors() -> Colors {
    let (background, _) = generate_color_pair(0.30, 0.039, 221.11);
    let (_, foreground) = generate_color_pair(0.8, 0.0, 0.0);
    let (black, black_bright) = generate_color_pair(0.42, 0.03, CYAN_HUE);
    let (red, red_bright) = generate_color_pair(0.6, CHROMA_STD + 0.1, RED_HUE);
    let (green, green_bright) = generate_color_pair(0.8, CHROMA_STD, GREEN_HUE);
    let (yellow, yellow_bright) = generate_color_pair(0.99, CHROMA_STD, YELLOW_HUE);
    let (blue, blue_bright) = generate_color_pair(0.9, CHROMA_STD, BLUE_HUE);
    let (magenta, magenta_bright) = generate_color_pair(0.7, CHROMA_STD, MAGENTA_HUE);
    let (cyan, cyan_bright) = generate_color_pair(0.8, CHROMA_STD, CYAN_HUE);
    let (white, white_bright) = generate_color_pair(1.45, 0.02, GREEN_HUE);

    Colors {
        foreground,
        background,
        black,
        red,
        green,
        yellow,
        blue,
        magenta,
        cyan,
        white,
        black_bright,
        red_bright,
        green_bright,
        yellow_bright,
        blue_bright,
        magenta_bright,
        cyan_bright,
        white_bright,
    }
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
