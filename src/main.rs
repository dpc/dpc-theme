use palette::{Clamp, FromColor, Oklch, Srgb};

pub const RED_HUE: f32 = 0.0;
pub const GREEN_HUE: f32 = 145.0;
pub const YELLOW_HUE: f32 = 103.0;
pub const BLUE_HUE: f32 = 256.0;
pub const MAGENTA_HUE: f32 = 300.0;
pub const CYAN_HUE: f32 = 210.0;

pub const CHROMA_MAX: f32 = 0.37;
pub const CHROMA_STD: f32 = 0.2;
fn main() {
    for (h_name, l, hue, c) in [
        ("black", 0.22, CYAN_HUE, 0.03),
        ("red", 0.7, RED_HUE, CHROMA_STD),
        ("green", 0.8, GREEN_HUE, CHROMA_STD),
        ("yellow", 0.99, YELLOW_HUE, CHROMA_STD),
        ("blue", 0.8, BLUE_HUE, CHROMA_STD),
        ("magenta", 0.7, MAGENTA_HUE, CHROMA_STD),
        ("cyan", 0.8, CYAN_HUE, CHROMA_STD),
        ("white", 1.3, GREEN_HUE, 0.02),
    ] {
        for (l_name, l, c) in [("", l * 0.65, c), ("b.", l, c - 0.02)] {
            let oklch = Oklch::new(l, c, hue);
            let srgb = Srgb::<f32>::from_color(oklch.clamp());
            let srgb = Srgb::<u8>::from_format(srgb);
            print!(
                "{}{:10}#{srgb:X}({l:.2} {c:.2} {hue:06.2}){}",
                anstyle::RgbColor(srgb.red, srgb.green, srgb.blue).render_bg(),
                format!("{}{}", l_name, h_name),
                anstyle::Reset,
            );
            println!(
                "{}{:10}#{srgb:X}({l:.2} {c:.2} {hue:06.2}){}",
                anstyle::RgbColor(srgb.red, srgb.green, srgb.blue).render_fg(),
                format!("{}{}", l_name, h_name),
                anstyle::Reset,
            );
        }
    }
}
