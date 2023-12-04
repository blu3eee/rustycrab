use csscolorparser::Color as CssColor;

#[derive(Debug, PartialEq)]
pub enum ColorResolvables {
    ColorInt(u32),
    HexString(String),
    Text(String),
    Red,
    Green,
    Blue,
    Yellow,
}

impl ColorResolvables {
    pub fn as_u32(&self) -> u32 {
        match self {
            ColorResolvables::ColorInt(number) => *number,
            ColorResolvables::HexString(hex) => {
                u32::from_str_radix(hex.trim_start_matches("#"), 16).unwrap_or_default()
            }
            ColorResolvables::Text(text) => {
                let color = text.parse::<CssColor>().or_else(|_| {
                    // If direct parsing fails, try with a "#" prefix
                    format!("#{}", text).parse::<CssColor>()
                });

                // Convert the parsed color to a hex string if successful
                color.map_or(0x118ab2, |c|
                    u32
                        ::from_str_radix(c.to_hex_string().trim_start_matches("#"), 16)
                        .unwrap_or_default()
                )
            }
            ColorResolvables::Red => 0xef476f, // Direct hex values
            ColorResolvables::Green => 0x06d6a0,
            ColorResolvables::Blue => 0x118ab2,
            ColorResolvables::Yellow => 0xffd166,
        }
    }
}
