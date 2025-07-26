/// Converts an RGB color tuple to a HSL color tuple.
/// $H$ is in the range $[0, 360)$, and $S, L$ are in the range $[0, 1]$.
fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r_norm = r as f32 / 255.0;
    let g_norm = g as f32 / 255.0;
    let b_norm = b as f32 / 255.0;

    let c_max = r_norm.max(g_norm).max(b_norm);
    let c_min = r_norm.min(g_norm).min(b_norm);
    let delta = c_max - c_min;

    let mut h = if delta == 0.0 {
        0.0
    } else if c_max == r_norm {
        60.0 * (((g_norm - b_norm) / delta) % 6.0)
    } else if c_max == g_norm {
        60.0 * ((b_norm - r_norm) / delta + 2.0)
    } else {
        // c_max == b_norm
        60.0 * ((r_norm - g_norm) / delta + 4.0)
    };

    if h < 0.0 {
        h += 360.0;
    }

    let l = (c_max + c_min) / 2.0;

    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    (h, s, l)
}

/// Converts a HSL color tuple to an RGB color tuple (0-255).
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r_p, g_p, b_p) = if (0.0..60.0).contains(&h) {
        (c, x, 0.0)
    } else if (60.0..120.0).contains(&h) {
        (x, c, 0.0)
    } else if (120.0..180.0).contains(&h) {
        (0.0, c, x)
    } else if (180.0..240.0).contains(&h) {
        (0.0, x, c)
    } else if (240.0..300.0).contains(&h) {
        (x, 0.0, c)
    } else {
        // 300.0..360.0
        (c, 0.0, x)
    };

    let r = ((r_p + m) * 255.0).round() as u8;
    let g = ((g_p + m) * 255.0).round() as u8;
    let b = ((b_p + m) * 255.0).round() as u8;

    (r, g, b)
}

/// Converts an RGB color tuple to a CSS hex string.
fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

/// Generates the 5-color palette from a base RGB color.
fn derive_palette(r: u8, g: u8, b: u8, invert: bool) -> Vec<Option<String>> {
    let (h, _, _) = rgb_to_hsl(r, g, b);
    let hx = h;
    let hy = (h + 320.0) % 360.0;

    let c1 = rgb_to_hex(
        hsl_to_rgb(hx, 1.0, 0.1).0,
        hsl_to_rgb(hx, 1.0, 0.1).1,
        hsl_to_rgb(hx, 1.0, 0.1).2,
    );

    let (c2, c3, c4, c5) = if invert {
        (
            rgb_to_hex(
                hsl_to_rgb(hx, 1.0, 0.7).0,
                hsl_to_rgb(hx, 1.0, 0.7).1,
                hsl_to_rgb(hx, 1.0, 0.7).2,
            ), // c2
            rgb_to_hex(
                hsl_to_rgb(hy, 1.0, 0.8).0,
                hsl_to_rgb(hy, 1.0, 0.8).1,
                hsl_to_rgb(hy, 1.0, 0.8).2,
            ), // c3
            rgb_to_hex(
                hsl_to_rgb(hx, 1.0, 0.2).0,
                hsl_to_rgb(hx, 1.0, 0.2).1,
                hsl_to_rgb(hx, 1.0, 0.2).2,
            ), // c4
            rgb_to_hex(
                hsl_to_rgb(hx, 1.0, 0.45).0,
                hsl_to_rgb(hx, 1.0, 0.45).1,
                hsl_to_rgb(hx, 1.0, 0.45).2,
            ), // c5
        )
    } else {
        (
            rgb_to_hex(
                hsl_to_rgb(hx, 1.0, 0.2).0,
                hsl_to_rgb(hx, 1.0, 0.2).1,
                hsl_to_rgb(hx, 1.0, 0.2).2,
            ), // c2
            rgb_to_hex(
                hsl_to_rgb(hx, 1.0, 0.45).0,
                hsl_to_rgb(hx, 1.0, 0.45).1,
                hsl_to_rgb(hx, 1.0, 0.45).2,
            ), // c3
            rgb_to_hex(
                hsl_to_rgb(hx, 1.0, 0.7).0,
                hsl_to_rgb(hx, 1.0, 0.7).1,
                hsl_to_rgb(hx, 1.0, 0.7).2,
            ), // c4
            rgb_to_hex(
                hsl_to_rgb(hy, 1.0, 0.8).0,
                hsl_to_rgb(hy, 1.0, 0.8).1,
                hsl_to_rgb(hy, 1.0, 0.8).2,
            ), // c5
        )
    };

    vec![None, Some(c1), Some(c2), Some(c3), Some(c4), Some(c5)]
}

/// Parses a MoonCat's ID into a 2D array of pixel colors.
///
/// # Arguments
///
/// * `cat_id` - The 5-byte hexadecimal ID string for the MoonCat (e.g., "0x00c4202241").
/// * `designs` - A slice of strings, where each string is a pixel map for a cat design.
///
/// # Returns
///
/// A `Result` containing either the pixel data (`Vec<Vec<Option<String>>>`) on success,
/// or an error message string on failure. The pixel data is a 2D array where each
/// element is an `Option<String>` representing a hex color code (or `None` for transparent).
pub fn mooncat_parser(cat_id: &str, designs: &[&str]) -> Result<Vec<Vec<Option<String>>>, String> {
    let cat_id_trimmed = cat_id.strip_prefix("0x").unwrap_or(cat_id);

    let bytes = hex::decode(cat_id_trimmed).map_err(|e| format!("Invalid hex in catId: {}", e))?;

    if bytes.len() < 5 {
        return Err("catId must be at least 5 bytes (10 hex chars) long.".to_string());
    }

    let genesis = bytes[0] != 0;
    let k = bytes[1];
    let r = bytes[2];
    let g = bytes[3];
    let b = bytes[4];

    let invert = k >= 128;
    let k_norm = (k % 128) as usize;

    let design_str = designs.get(k_norm).ok_or_else(|| {
        format!(
            "Design index {} is out of bounds for designs array.",
            k_norm
        )
    })?;

    // Determine the color palette
    let colors: Vec<Option<String>> = if genesis {
        // This logic is equivalent to: (k % 2 === 0 && invert || k % 2 === 1 && !invert)
        if (k % 2 != 0) != invert {
            vec![
                None,
                Some("#555555".to_string()),
                Some("#d3d3d3".to_string()),
                Some("#ffffff".to_string()),
                Some("#aaaaaa".to_string()),
                Some("#ff9999".to_string()),
            ]
        } else {
            vec![
                None,
                Some("#555555".to_string()),
                Some("#222222".to_string()),
                Some("#111111".to_string()),
                Some("#bbbbbb".to_string()),
                Some("#ff9999".to_string()),
            ]
        }
    } else {
        derive_palette(r, g, b, invert)
    };

    // Map the design characters to the color palette
    let pixel_data = design_str
        .split('.')
        .map(|row_str| {
            row_str
                .chars()
                .map(|cell_char| {
                    let index = cell_char.to_digit(10).unwrap_or(0) as usize;
                    // Safely get the color from the palette, cloning it.
                    // Returns None if index is out of bounds or if the color is None.
                    colors.get(index).and_then(|opt_color| opt_color.clone())
                })
                .collect::<Vec<Option<String>>>()
        })
        .collect::<Vec<Vec<Option<String>>>>();

    Ok(pixel_data)
}
