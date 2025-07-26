mod labubu_parser_lib;
use image::{Rgba, RgbaImage, imageops};

fn parse_hex_color(hex_str: &str) -> Option<[u8; 3]> {
    let hex_str = hex_str.strip_prefix('#').unwrap_or(hex_str);
    if hex_str.len() != 6 {
        return None;
    }
    let mut rgb = [0u8; 3];
    hex::decode_to_slice(hex_str, &mut rgb).ok()?;
    Some(rgb)
}
fn main() {
    const LABUBU_DESIGNS: &[&str] = &[
        "00001100000000110000.00013310000001331000.00133331000133331000.01333333101333333100.01333333311333333100.00111111111111111100.01333333333333333100.13334433333333443331.13340433333333404331.13333333333333333331.13333111111111133331.13331551551551513331.13331111111111113331.01333333333333333100.00133222222222223310.00011222222222221100.00000111111111110000",
        // You could add more poses here!
    ];

    let cat_id = "0000000000";

    match labubu_parser_lib::labubu_parser(cat_id, LABUBU_DESIGNS) {
        Ok(pixel_data) => {
            println!("Successfully parsed Cat ID: {}", cat_id);

            let height = pixel_data.len() as u32;
            let width = if height > 0 {
                pixel_data[0].len() as u32
            } else {
                0
            };

            if width == 0 || height == 0 {
                println!("Cannot create image from empty pixel data.");
                return;
            }

            let mut img: RgbaImage = RgbaImage::new(width, height);

            for (y, row) in pixel_data.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    let pixel = match cell {
                        Some(hex_color) => {
                            let rgb = parse_hex_color(hex_color).unwrap_or([0, 0, 0]);
                            Rgba([rgb[0], rgb[1], rgb[2], 255])
                        }
                        None => Rgba([0, 0, 0, 0]),
                    };
                    img.put_pixel(x as u32, y as u32, pixel);
                }
            }

            let scale_factor = 12;
            let scaled_img = imageops::resize(
                &img,
                width * scale_factor,
                height * scale_factor,
                imageops::FilterType::Nearest,
            );

            let output_filename = "labubu.png";
            scaled_img
                .save(output_filename)
                .expect("Failed to save image.");

            println!("✅ Successfully saved image as {}", output_filename);
        }
        Err(e) => {
            eprintln!("Error parsing catId: {}", e);
        }
    }
}
