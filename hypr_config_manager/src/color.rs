use regex::Regex;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color {
    pub value: String,
    pub rgba: (u8, u8, u8, u8),
}

impl Color {
    pub fn new(value: &str) -> Self {
        let rgba = Self::parse_to_rgba(value);
        Self {
            value: value.to_string(),
            rgba,
        }
    }

    fn parse_to_rgba(color_str: &str) -> (u8, u8, u8, u8) {
        if color_str.is_empty() {
            return (0, 0, 0, 255);
        }

        let color_str = color_str.trim();

        if let Some(hex_val) = color_str.strip_prefix('#') {
            if hex_val.len() == 6
                && let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&hex_val[0..2], 16),
                    u8::from_str_radix(&hex_val[2..4], 16),
                    u8::from_str_radix(&hex_val[4..6], 16),
                )
            {
                return (r, g, b, 255);
            } else if hex_val.len() == 8
                && let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
                    u8::from_str_radix(&hex_val[0..2], 16),
                    u8::from_str_radix(&hex_val[2..4], 16),
                    u8::from_str_radix(&hex_val[4..6], 16),
                    u8::from_str_radix(&hex_val[6..8], 16),
                )
            {
                return (r, g, b, a);
            } else if hex_val.len() == 3
                && let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&format!("{}{}", &hex_val[0..1], &hex_val[0..1]), 16),
                    u8::from_str_radix(&format!("{}{}", &hex_val[1..2], &hex_val[1..2]), 16),
                    u8::from_str_radix(&format!("{}{}", &hex_val[2..3], &hex_val[2..3]), 16),
                )
            {
                return (r, g, b, 255);
            } else if hex_val.len() == 4
                && let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
                    u8::from_str_radix(&format!("{}{}", &hex_val[0..1], &hex_val[0..1]), 16),
                    u8::from_str_radix(&format!("{}{}", &hex_val[1..2], &hex_val[1..2]), 16),
                    u8::from_str_radix(&format!("{}{}", &hex_val[2..3], &hex_val[2..3]), 16),
                    u8::from_str_radix(&format!("{}{}", &hex_val[3..4], &hex_val[3..4]), 16),
                )
            {
                return (r, g, b, a);
            }
        }

        if let Some(stripped) = color_str.strip_prefix("0x") {
            if stripped.len() == 8 {
                if let Ok(val) = u32::from_str_radix(stripped, 16) {
                    let a = ((val >> 24) & 0xFF) as u8;
                    let r = ((val >> 16) & 0xFF) as u8;
                    let g = ((val >> 8) & 0xFF) as u8;
                    let b = (val & 0xFF) as u8;
                    return (r, g, b, a);
                }
            } else if stripped.len() == 6
                && let Ok(val) = u32::from_str_radix(stripped, 16)
            {
                let r = ((val >> 16) & 0xFF) as u8;
                let g = ((val >> 8) & 0xFF) as u8;
                let b = (val & 0xFF) as u8;
                return (r, g, b, 255);
            }
        }

        // Handle rgba function syntax
        if let Ok(re) = Regex::new(r"rgba?\((\d+),\s*(\d+),\s*(\d+)(?:,\s*([\d.]+))?\)")
            && let Some(caps) = re.captures(color_str)
            && let (Ok(r), Ok(g), Ok(b)) = (
                caps[1].parse::<u8>(),
                caps[2].parse::<u8>(),
                caps[3].parse::<u8>(),
            )
        {
            let mut a = 255;
            if let Some(m) = caps.get(4)
                && let Ok(a_float) = m.as_str().parse::<f32>()
            {
                if a_float <= 1.0 {
                    a = (a_float * 255.0) as u8;
                } else {
                    a = a_float as u8;
                }
            }
            return (r, g, b, a);
        }

        (0, 0, 0, 255)
    }

    pub fn r(&self) -> u8 {
        self.rgba.0
    }
    pub fn g(&self) -> u8 {
        self.rgba.1
    }
    pub fn b(&self) -> u8 {
        self.rgba.2
    }
    pub fn a(&self) -> u8 {
        self.rgba.3
    }

    pub fn hypr(&self) -> String {
        format!(
            "0x{:02x}{:02x}{:02x}{:02x}",
            self.a(),
            self.r(),
            self.g(),
            self.b()
        )
    }

    pub fn css(&self) -> String {
        let alpha = (self.a() as f32) / 255.0;
        format!(
            "rgba({}, {}, {}, {:.2})",
            self.r(),
            self.g(),
            self.b(),
            alpha
        )
    }

    pub fn rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.r(), self.g(), self.b())
    }

    pub fn hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }

    pub fn hexa(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            self.r(),
            self.g(),
            self.b(),
            self.a()
        )
    }

    pub fn csv(&self) -> String {
        format!("{},{},{}", self.r(), self.g(), self.b())
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
