mod mooncat_parser_lib;
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
        "0000011000000011000000.0004144100000144100000.0004141410001414100000.0012441410001414410000.0012211441014411410000.0012213041014031410000.0012213041014031410000.0012213041014031410000.0012444411111444410000.0012444444444444410000.0011144444444444110000.0124444444444444441000.1444444444444444444100.4444433344444333444410.4443300033333000344410.4430000000000000034410.4430030030003003034441.4300003000000030003441.4300000400000400003441.4300004100000140003441.4330004403330440033441.4300004400300440003441.4300000000000000003441.4430000000000000034410.4433000000000000034410.4443300000000000344410.1124433000000033441100.0011144333333344110000.0124214444144441441000.1441141111111114144100.4441144414141444144410.4412444444144444414410.4412444444144444414410.0212444444144444414030.0124444414141444441000.0124444414141444441000.0124444444144444441000.0124444444144444441000.0124444414141444441000.0014044441114444010000",
    ];

    let cat_id = "0000000000";

    match mooncat_parser_lib::mooncat_parser(cat_id, LABUBU_DESIGNS) {
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

            let output_filename = "mooncat.png";
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
