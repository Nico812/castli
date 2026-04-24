use crossterm::style::Color;

// COLOR SCHEME
pub const BLACK: Color = Color::Black;
pub const BLUE_1: Color = Color::Blue;
pub const BLUE_0: Color = Color::Rgb {
    r: 200,
    g: 200,
    b: 255,
};
pub const GREEN_0: Color = Color::Green;
pub const GREEN_1: Color = Color::Rgb { r: 0, g: 150, b: 0 };
pub const GREEN_2: Color = Color::Rgb { r: 0, g: 100, b: 0 };
pub const GREEN_3: Color = Color::Rgb { r: 0, g: 70, b: 0 };
pub const GREY_2: Color = Color::Rgb {
    r: 100,
    g: 100,
    b: 100,
};
pub const GREY_1: Color = Color::Rgb {
    r: 128,
    g: 128,
    b: 128,
};
pub const GREY_0: Color = Color::Rgb {
    r: 192,
    g: 192,
    b: 192,
};
pub const MAGENTA: Color = Color::Magenta;
pub const RED: Color = Color::Red;
pub const WHITE: Color = Color::White;
