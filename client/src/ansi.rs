#![allow(dead_code)]

use crossterm::style::Color;

// COLOR SCHEME
pub const BLACK: Color = Color::Black;
pub const BLACK_BRIGHT: Color = Color::DarkGrey;
pub const BLUE: Color = Color::Blue;
pub const BLUE_BRIGHT: Color = Color::Rgb {
    r: 80,
    g: 80,
    b: 255,
};
pub const BROWN: Color = Color::Rgb {
    r: 139,
    g: 69,
    b: 19,
};
pub const CYAN: Color = Color::Cyan;
pub const GREEN_BRIGHT: Color = Color::Green;
pub const GREEN: Color = Color::Rgb {
    r: 30,
    g: 240,
    b: 30,
};
pub const GREEN_DARK: Color = Color::DarkGreen;
pub const GREEN_DARKER: Color = Color::Rgb { r: 0, g: 70, b: 0 };
pub const GREY: Color = Color::Grey;
pub const GREY_BRIGHT: Color = Color::White; // O Color::BrightWhite
pub const GREY_GREENISH: Color = Color::Rgb {
    r: 128,
    g: 138,
    b: 115,
};
pub const LIGHT_BROWN: Color = Color::Rgb {
    r: 205,
    g: 133,
    b: 63,
};
pub const MAGENTA: Color = Color::Magenta;
pub const RED: Color = Color::Red;
pub const WHITE: Color = Color::White;
pub const YELLOW: Color = Color::Yellow;
