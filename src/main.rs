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
pub const YELLOW_HUE: f32 = 103.0;
pub const BLUE_HUE: f32 = 256.0;
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
        ("black", 0.0, 0.0, 0.0),
        ("red", 0.5, RED_HUE, CHROMA_MAX),
        ("green", 0.5, GREEN_HUE, CHROMA_MAX),
        ("yellow", 0.5, YELLOW_HUE, CHROMA_MAX),
        ("blue", 0.5, BLUE_HUE, CHROMA_MAX),
        ("magenta", 0.5, MAGENTA_HUE, CHROMA_MAX),
        ("cyan", 0.5, CYAN_HUE, CHROMA_MAX),
        ("white", 0.9, 0.0, 0.0),
    ] {
        for (l_name, l) in [("", l), ("b.", l + 0.3)] {
            let oklch = Oklch::new(l, c, hue);
            let srgb = Srgb::<f32>::from_color(oklch.clamp());
            let srgb = Srgb::<u8>::from_format(srgb);
            match (l_name, h_name) {
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

fn print_color(name: &str, color: Color) {
    println!(
        "{}{:10}#{color:X}{}",
        anstyle::RgbColor(color.red, color.green, color.blue).render_bg(),
        name,
        anstyle::Reset,
    );
    println!(
        "{}{:10}#{color:X}{}",
        anstyle::RgbColor(color.red, color.green, color.blue).render_fg(),
        name,
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
            print_color("fg", colors.foreground);
            print_color("bg", colors.background);
            print_color("black", colors.black);
            print_color("red", colors.red);
            print_color("green", colors.green);
            print_color("yellow", colors.yellow);
            print_color("blue", colors.blue);
            print_color("magenta", colors.magenta);
            print_color("cyan", colors.cyan);
            print_color("white", colors.white);
            print_color("blackb", colors.black_bright);
            print_color("redb", colors.red_bright);
            print_color("greenb", colors.green_bright);
            print_color("yellowb", colors.yellow_bright);
            print_color("blueb", colors.blue_bright);
            print_color("magentab", colors.magenta_bright);
            print_color("cyanb", colors.cyan_bright);
            print_color("whiteb", colors.white_bright);
        }
    }
}
