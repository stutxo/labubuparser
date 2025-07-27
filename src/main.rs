use image::{ImageBuffer, ImageReader, Rgba, imageops::FilterType};
use palette::{FromColor, Hsl, Srgb};
use rand::{Rng, SeedableRng, distr::Uniform, rngs::StdRng};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let in_path = "labewbuw-2.png";
    let tile_px: u32 = 28;
    let rng_seed: Option<u64> = None;

    let img = ImageReader::open(in_path)?.decode()?.to_rgba8();
    let (w, h) = img.dimensions();
    let grid_w = (w as f32 / tile_px as f32).round() as u32;
    let grid_h = (h as f32 / tile_px as f32).round() as u32;
    let tiny = image::imageops::resize(&img, grid_w, grid_h, FilterType::Nearest);
    let flat = tiny; // no clone needed

    let mut rng: StdRng = rng_seed
        .map(StdRng::seed_from_u64)
        .unwrap_or_else(|| StdRng::from_seed(rand::random()));
    let new_hue: f32 = rng.sample(Uniform::new(0.0, 360.0).unwrap());

    let (out_w, out_h) = flat.dimensions();
    let mut out: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(out_w, out_h);
    for y in 0..out_h {
        for x in 0..out_w {
            let p = flat.get_pixel(x, y);
            let a = p[3];
            if a == 0 {
                out.put_pixel(x, y, Rgba([0, 0, 0, 0]));
                continue;
            }
            let rgb = Srgb::new(p[0] as f32 / 255.0, p[1] as f32 / 255.0, p[2] as f32 / 255.0);
            let hsl: Hsl = Hsl::from_color(rgb);
            let min_sat = 0.6;
            let sat = if hsl.saturation < 0.05 { min_sat } else { hsl.saturation };
            let recol_f32 = Srgb::from_color(Hsl::new(palette::RgbHue::from_degrees(new_hue), sat, hsl.lightness));
            let mut recol_u8 = recol_f32.into_format::<u8>();
            // Preserve white teeth
            if p[0] > 240 && p[1] > 240 && p[2] > 240 {
                recol_u8.red = 255;
                recol_u8.green = 255;
                recol_u8.blue = 255;
            }

            out.put_pixel(x, y, Rgba([recol_u8.red, recol_u8.green, recol_u8.blue, a]));
        }
    }

    let out_path = Path::new("control_block_labubu.png");
    out.save(&out_path)?;
    Ok(())
}
