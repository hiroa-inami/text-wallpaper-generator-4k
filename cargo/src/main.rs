use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use rand::seq::IteratorRandom;
use rand::Rng;
use rusttype::{Font, Scale};
use serde::Deserialize;
use std::fs;
use toml;
use ab_glyph;
use std::collections::HashMap;
use std::path::Path;

#[derive(Deserialize)]
struct ColorRange {
    g: (u8, u8),
    r: (u8, u8),
    b: (u8, u8),
}

#[derive(Deserialize)]
struct Settings {
    color_range: ColorRange,
    texts: Vec<String>,
    num_wallpapers_per_text: u32,
    width: u32,
    height: u32,
}

fn main() {
    // 1. Load Settings
    let settings = load_settings("settings.toml");

    // 2. Load Fonts
    let fonts = load_fonts_with_data("fonts");

    // 3. Generate Wallpapers
    for text in &settings.texts {
        for i in 0..settings.num_wallpapers_per_text {
            // a. Generate random color
            let color = generate_random_color(&settings.color_range);
            let width = settings.width;
            let height = settings.height;
            // b. Choose random font
            let (font_path, (font_data, font)) = fonts.iter().choose(&mut rand::thread_rng()).unwrap();
            
            let font_name = Path::new(font_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown_font");

            // c. Create image
            let mut image = RgbImage::new(width, height);
            for x in 0..width {
                for y in 0..height {
                    image.put_pixel(x, y, color);
                }
            }

            // d. Calculate font scale and draw text
            let scale = calculate_font_scale(&font, &text, width, height);
            let text_width = text_width(&font, scale, &text);
            let text_height = text_height(&font, scale);

            let x = (width.saturating_sub(text_width)) / 2;
            let y = (height.saturating_sub(text_height)) / 2;
            let px_scale = ab_glyph::PxScale::from(scale.x);
            let font: ab_glyph::FontRef<'_> = ab_glyph::FontRef::try_from_slice(font_data).unwrap();

            draw_text_mut(&mut image, Rgb([255,255,255]), x.try_into().unwrap(), y.try_into().unwrap(), px_scale, &font, &text);

            // e. Save the image
            let filename = format!("wallpaper_{}_{}_{}.png", text, font_name, i + 1);
            println!("Generating: {}", filename);
            image.save(filename).expect("Failed to save image");
        }
    }
}

fn load_settings(filename: &str) -> Settings {
    let contents = fs::read_to_string(filename).expect("Failed to read settings file");
    toml::from_str(&contents).expect("Failed to parse settings file")
}

fn load_fonts_with_data(fonts_dir: &str) -> HashMap<String, (Vec<u8>, Font<'static>)> {
    let mut fonts = HashMap::new();
    for entry in walkdir::WalkDir::new(fonts_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            !e.file_type().is_dir()
                && e.path()
                    .extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("ttf"))
        })
    {
        let font_data = fs::read(entry.path()).expect("Failed to read font file");
        let font = Font::try_from_vec(font_data.clone()).expect("Failed to load font");
        fonts.insert(
            entry.path().display().to_string(),
            (font_data, font),
        );
    }
    fonts
}


fn generate_random_color(color_range: &ColorRange) -> Rgb<u8> {
    let mut rng = rand::thread_rng();
    Rgb([
        rng.gen_range(color_range.r.0..=color_range.r.1),
        rng.gen_range(color_range.g.0..=color_range.g.1),
        rng.gen_range(color_range.b.0..=color_range.b.1),
    ])
}

fn calculate_font_scale(font: &Font, text: &str, width: u32, height: u32) -> Scale {
    let mut scale = Scale::uniform(100.0); // Start with an arbitrary scale
    let padding_x = (width as f32 * 0.1) as u32; // 10% padding on each side
    let padding_y = (height as f32 * 0.1) as u32; // 10% padding on top and bottom

    loop {
        let text_width = text_width(font, scale, text);
        let text_height = text_height(font, scale);

        if text_width > width - padding_x || text_height > height - padding_y {
            scale = Scale::uniform(scale.x * 0.95);
            break;
        }
        scale = Scale::uniform(scale.x * 1.05); // Increase scale by 5%
    }
    scale
}

fn text_width(font: &Font, scale: Scale, text: &str) -> u32 {
    let glyphs_width: f32 = font
        .layout(text, scale, rusttype::Point { x: 0.0, y: 0.0 })
        .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .last()
        .unwrap_or(0.0);

    glyphs_width.round() as u32
}

fn text_height(font: &Font, scale: Scale) -> u32 {
    let v_metrics = font.v_metrics(scale);
    (v_metrics.ascent - v_metrics.descent).round() as u32
}
