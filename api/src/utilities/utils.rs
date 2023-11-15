pub enum ColorTypes {
    String(String),
}

pub fn convert_color_u64(color: ColorTypes) -> u32 {
    match color {
        ColorTypes::String(color_string) => {
            u32::from_str_radix(color_string.trim_start_matches("#"), 16).unwrap_or_else(|_|
                u32::from_str_radix("2B2D31", 16).unwrap()
            )
        }
    }
}
