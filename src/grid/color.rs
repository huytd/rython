use ratatui::style::Color as RColor;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
    Red,
    Cyan,
    Purple,
    Green,
    Blue,
    Yellow,
    Orange,
    Brown,
    Lightred,
    Darkgray,
    Gray,
    Lightgreen,
    Lightblue,
    Lightgray,
}

impl Color {
    pub fn from_name(name: &str) -> Result<Color, String> {
        match name.to_lowercase().as_str() {
            "black" => Ok(Color::Black),
            "white" => Ok(Color::White),
            "red" => Ok(Color::Red),
            "cyan" => Ok(Color::Cyan),
            "purple" => Ok(Color::Purple),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            "yellow" => Ok(Color::Yellow),
            "orange" => Ok(Color::Orange),
            "brown" => Ok(Color::Brown),
            "lightred" => Ok(Color::Lightred),
            "darkgray" => Ok(Color::Darkgray),
            "gray" => Ok(Color::Gray),
            "lightgreen" => Ok(Color::Lightgreen),
            "lightblue" => Ok(Color::Lightblue),
            "lightgray" => Ok(Color::Lightgray),
            _ => Err(format!("Unknown color name: '{}'", name)),
        }
    }

    /// Convert to ratatui Color using C64-inspired RGB values.
    pub fn to_ratatui(self) -> RColor {
        match self {
            Color::Black => RColor::Rgb(0x00, 0x00, 0x00),
            Color::White => RColor::Rgb(0xFF, 0xFF, 0xFF),
            Color::Red => RColor::Rgb(0xCC, 0x00, 0x00),
            Color::Cyan => RColor::Rgb(0x66, 0xCC, 0xCC),
            Color::Purple => RColor::Rgb(0xCC, 0x00, 0xCC),
            Color::Green => RColor::Rgb(0x00, 0xCC, 0x00),
            Color::Blue => RColor::Rgb(0x00, 0x00, 0xCC),
            Color::Yellow => RColor::Rgb(0xCC, 0xCC, 0x00),
            Color::Orange => RColor::Rgb(0xCC, 0x66, 0x00),
            Color::Brown => RColor::Rgb(0x66, 0x33, 0x00),
            Color::Lightred => RColor::Rgb(0xFF, 0x66, 0x66),
            Color::Darkgray => RColor::Rgb(0x66, 0x66, 0x66),
            Color::Gray => RColor::Rgb(0x99, 0x99, 0x99),
            Color::Lightgreen => RColor::Rgb(0x66, 0xFF, 0x66),
            Color::Lightblue => RColor::Rgb(0x66, 0x99, 0xFF),
            Color::Lightgray => RColor::Rgb(0xCC, 0xCC, 0xCC),
        }
    }
}
