use gtk4::gdk;
use regex::Regex;

pub struct ColorUtils;

impl ColorUtils {
    pub fn is_valid_color_string(s: &str) -> bool {
        let s = s.trim();
        if s.is_empty() {
            return false;
        }
        if s.starts_with("0x")
            || s.starts_with("#")
            || s.starts_with("rgba(")
            || s.starts_with("rgb(")
        {
            return true;
        }
        // Raw hex (AARRGGBB or RRGGBB)
        if (s.len() == 6 || s.len() == 8) && s.chars().all(|c| c.is_ascii_hexdigit()) {
            return true;
        }
        false
    }

    pub fn split_hypr_color_string(s: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current_part = String::new();
        let mut paren_depth: i32 = 0;

        for char in s.chars() {
            if char == ' ' && paren_depth == 0 {
                if !current_part.is_empty() {
                    parts.push(current_part.clone());
                    current_part.clear();
                }
            } else {
                if char == '(' {
                    paren_depth += 1;
                } else if char == ')' {
                    paren_depth = paren_depth.saturating_sub(1);
                }
                current_part.push(char);
            }
        }

        if !current_part.is_empty() {
            parts.push(current_part);
        }

        parts
    }

    pub fn rgba_to_gdkrgba(color_str: &str) -> gdk::RGBA {
        if color_str.is_empty() {
            return gdk::RGBA::BLACK;
        }

        let color_str = color_str.trim();

        // Handle 0xaarrggbb (Hyprland Hex)
        if color_str.starts_with("0x")
            && let Ok(val) = u64::from_str_radix(&color_str[2..], 16)
        {
            if color_str.len() == 10 {
                // 0xaarrggbb
                let a = ((val >> 24) & 0xFF) as f32 / 255.0;
                let r = ((val >> 16) & 0xFF) as f32 / 255.0;
                let g = ((val >> 8) & 0xFF) as f32 / 255.0;
                let b = (val & 0xFF) as f32 / 255.0;
                return gdk::RGBA::new(r, g, b, a);
            } else if color_str.len() == 8 {
                // 0xrrggbb
                let r = ((val >> 16) & 0xFF) as f32 / 255.0;
                let g = ((val >> 8) & 0xFF) as f32 / 255.0;
                let b = (val & 0xFF) as f32 / 255.0;
                return gdk::RGBA::new(r, g, b, 1.0);
            }
        }

        // Handle rgba(RRGGBBAA) - Hyprland Hex
        let rgba_hex_regex = Regex::new(
            r"rgba\(\s*([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})\s*\)",
        )
        .unwrap();
        if let Some(caps) = rgba_hex_regex.captures(color_str) {
            let r = u8::from_str_radix(&caps[1], 16).unwrap_or(0) as f32 / 255.0;
            let g = u8::from_str_radix(&caps[2], 16).unwrap_or(0) as f32 / 255.0;
            let b = u8::from_str_radix(&caps[3], 16).unwrap_or(0) as f32 / 255.0;
            let a = u8::from_str_radix(&caps[4], 16).unwrap_or(0) as f32 / 255.0;
            return gdk::RGBA::new(r, g, b, a);
        }

        // Handle rgb(RRGGBB)
        let rgb_hex_regex =
            Regex::new(r"rgb\(\s*([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})\s*\)").unwrap();
        if let Some(caps) = rgb_hex_regex.captures(color_str) {
            let r = u8::from_str_radix(&caps[1], 16).unwrap_or(0) as f32 / 255.0;
            let g = u8::from_str_radix(&caps[2], 16).unwrap_or(0) as f32 / 255.0;
            let b = u8::from_str_radix(&caps[3], 16).unwrap_or(0) as f32 / 255.0;
            return gdk::RGBA::new(r, g, b, 1.0);
        }

        // Handle CSS rgba(r, g, b, a) or rgb(r, g, b)
        let css_rgba_regex =
            Regex::new(r"rgba?\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)(?:\s*,\s*([\d.]+))?\s*\)")
                .unwrap();
        if let Some(caps) = css_rgba_regex.captures(color_str) {
            let r = caps[1].parse::<u8>().unwrap_or(0) as f32 / 255.0;
            let g = caps[2].parse::<u8>().unwrap_or(0) as f32 / 255.0;
            let b = caps[3].parse::<u8>().unwrap_or(0) as f32 / 255.0;
            let a = caps
                .get(4)
                .map_or(1.0, |m| m.as_str().parse::<f32>().unwrap_or(1.0));
            let a = if a > 1.0 { a / 255.0 } else { a };
            return gdk::RGBA::new(r, g, b, a);
        }

        // Handle #RRGGBB or #RRGGBBAA
        if let Some(hex_str) = color_str.strip_prefix("#")
            && let Ok(val) = u64::from_str_radix(hex_str, 16)
        {
            if hex_str.len() == 8 {
                let r = ((val >> 24) & 0xFF) as f32 / 255.0;
                let g = ((val >> 16) & 0xFF) as f32 / 255.0;
                let b = ((val >> 8) & 0xFF) as f32 / 255.0;
                let a = (val & 0xFF) as f32 / 255.0;
                return gdk::RGBA::new(r, g, b, a);
            } else if hex_str.len() == 6 {
                let r = ((val >> 16) & 0xFF) as f32 / 255.0;
                let g = ((val >> 8) & 0xFF) as f32 / 255.0;
                let b = (val & 0xFF) as f32 / 255.0;
                return gdk::RGBA::new(r, g, b, 1.0);
            }
        }

        // Handle raw hex (AARRGGBB or RRGGBB)
        if color_str.chars().all(|c| c.is_ascii_hexdigit())
            && let Ok(val) = u64::from_str_radix(color_str, 16)
        {
            if color_str.len() == 8 {
                let a = ((val >> 24) & 0xFF) as f32 / 255.0;
                let r = ((val >> 16) & 0xFF) as f32 / 255.0;
                let g = ((val >> 8) & 0xFF) as f32 / 255.0;
                let b = (val & 0xFF) as f32 / 255.0;
                return gdk::RGBA::new(r, g, b, a);
            } else if color_str.len() == 6 {
                let r = ((val >> 16) & 0xFF) as f32 / 255.0;
                let g = ((val >> 8) & 0xFF) as f32 / 255.0;
                let b = (val & 0xFF) as f32 / 255.0;
                return gdk::RGBA::new(r, g, b, 1.0);
            }
        }

        // Multi-part
        if color_str.contains(' ') {
            let parts = Self::split_hypr_color_string(color_str);
            for part in parts {
                if Self::is_valid_color_string(&part) {
                    return Self::rgba_to_gdkrgba(&part);
                }
            }
        }

        gdk::RGBA::BLACK
    }

    pub fn gdkrgba_to_rgba(gdk_rgba: &gdk::RGBA) -> String {
        let r = (gdk_rgba.red() * 255.0) as u8;
        let g = (gdk_rgba.green() * 255.0) as u8;
        let b = (gdk_rgba.blue() * 255.0) as u8;
        let a = (gdk_rgba.alpha() * 255.0) as u8;
        format!("rgba({:02x}{:02x}{:02x}{:02x})", r, g, b, a)
    }
}
