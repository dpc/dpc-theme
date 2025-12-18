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

pub const RED_HUE: f32 = 6.0;
pub const GREEN_HUE: f32 = 145.0;
pub const YELLOW_HUE: f32 = 93.0;
pub const BLUE_HUE: f32 = 264.0;
pub const MAGENTA_HUE: f32 = 300.0;
pub const CYAN_HUE: f32 = 210.0;

pub const CHROMA_MAX: f32 = 0.37;
pub const CHROMA_STD: f32 = 0.24;

type Color = Srgb<u8>;
struct Colors {
    foreground: Color,
    background: Color,
    bg_vvdark: Color,
    bg_vdark: Color,
    bg_dark: Color,
    bg_light: Color,
    bg_vlight: Color,
    bg_vvlight: Color,
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

fn generate_color(l: f32, c: f32, hue: f32) -> Color {
    let oklch = Oklch::new(l, c, hue);
    let srgb = Srgb::<f32>::from_color(oklch.clamp());
    Srgb::<u8>::from_format(srgb)
}

fn generate_color_pair(l: f32, c: f32, hue: f32) -> (Color, Color) {
    let normal = generate_color(l * 0.65, c, hue);
    let bright = generate_color(l, c - 0.02, hue);
    (normal, bright)
}

fn generate_colors() -> Colors {
    let (background, _) = generate_color_pair(0.29, 0.039, 221.11);
    let (_, foreground) = generate_color_pair(0.8, 0.0, 0.0);
    let (black, black_bright) = generate_color_pair(0.42, 0.03, CYAN_HUE);
    let (red, red_bright) = generate_color_pair(0.6, CHROMA_STD + 0.1, RED_HUE);
    let (green, green_bright) = generate_color_pair(0.8, CHROMA_STD, GREEN_HUE);
    let (yellow, yellow_bright) = generate_color_pair(0.85, CHROMA_STD, YELLOW_HUE);
    let (blue, blue_bright) = generate_color_pair(0.8, CHROMA_STD, BLUE_HUE);
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
        bg_vvdark: generate_color(0.20, 0.05, 219.6),
        bg_vdark: generate_color(0.274, 0.05, 219.6),
        bg_dark: generate_color(0.321, 0.053, 219.6),
        bg_light: generate_color(0.934, 0.031, 90.),
        bg_vlight: generate_color(0.977, 0.012, 90.),
        bg_vvlight: generate_color(1.037, 0.012, 90.),
    }
}

fn print_color(name: &str, normal: Color) {
    let color = anstyle::RgbColor(normal.red, normal.green, normal.blue).render_bg();

    print!("{}{:10}#{normal:X}{}", color, name, anstyle::Reset,);
}

fn println_color_pair(name: &str, normal: Color, bright: Color) {
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
    println!(
        r###"[colors]
ansi = [
    "#{:X}",
    "#{:X}",
    "#{:X}",
    "#{:X}",
    "#{:X}",
    "#{:X}",
    "#{:X}",
    "#{:X}",
]
background = "#{:X}"
brights = [
    "#{:X}",
    "#{:X}",
    "#{:X}",
    "#{:X}",
    "#{:X}",
    "#{:X}",
    "#{:X}",
    "#{:X}",
]
cursor_bg = "#{:X}"
cursor_border = "#{:X}"
cursor_fg = "#{:X}"
foreground = "#{:X}"
selection_bg = "#{:X}"
selection_fg = "#{:X}"

[metadata]
aliases = ["dpc"]
author = "dpc (http://github.com/dpc)"
name = "dpc"
origin_url = "https://github.com/dpc/dpc-theme"
wezterm_version = "wezterm 0-unstable-2025-01-24"
"###,
        colors.black,
        colors.red,
        colors.green,
        colors.yellow,
        colors.blue,
        colors.magenta,
        colors.cyan,
        colors.white,
        colors.background,
        colors.black_bright,
        colors.red_bright,
        colors.green_bright,
        colors.yellow_bright,
        colors.blue_bright,
        colors.magenta_bright,
        colors.cyan_bright,
        colors.white_bright,
        colors.white,
        colors.white,
        colors.black,
        colors.foreground,
        colors.white,
        colors.black,
    );
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
            println_color_pair("deft_fg", colors.foreground, colors.foreground);
            println_color_pair("deft_bg", colors.background, colors.background);
            print_color("bg_vvdark", colors.bg_vvdark);
            print!(" ");
            print_color("bg_vvlight", colors.bg_vvlight);
            println!();
            print_color("bg_vdark", colors.bg_vdark);
            print!(" ");
            print_color("bg_vlight", colors.bg_vlight);
            println!();
            print_color("bg_dark", colors.bg_dark);
            print!(" ");
            print_color("bg_light", colors.bg_light);
            println!();
            println_color_pair("black", colors.black, colors.black_bright);
            println_color_pair("red", colors.red, colors.red_bright);
            println_color_pair("green", colors.green, colors.green_bright);
            println_color_pair("yellow", colors.yellow, colors.yellow_bright);
            println_color_pair("blue", colors.blue, colors.blue_bright);
            println_color_pair("magenta", colors.magenta, colors.magenta_bright);
            println_color_pair("cyan", colors.cyan, colors.cyan_bright);
            println_color_pair("white", colors.white, colors.white_bright);
            println!();
        }
    }
}
