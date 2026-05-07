#![allow(dead_code)]

use crossterm::style::Color;

pub const BLACK: Color = Color::Black;
pub const MAGENTA: Color = Color::Magenta;
pub const RED: Color = Color::Red;
pub const WHITE: Color = Color::White;
pub const GREEN: Color = Color::Green;

// DAY PALETTE (WARMER
pub const FG_DAY_BROWN: Color = Color::Rgb {
    r: 180,
    g: 110,
    b: 55,
};
pub const BK_DAY_BROWN: Color = Color::Rgb {
    r: 70,
    g: 45,
    b: 20,
};

pub const FG_DAY_BLUE_1: Color = Color::Rgb {
    r: 85,
    g: 115,
    b: 195,
};
pub const BK_DAY_BLUE_1: Color = Color::Rgb {
    r: 35,
    g: 48,
    b: 78,
};

pub const FG_DAY_BLUE_0: Color = Color::Rgb {
    r: 105,
    g: 135,
    b: 215,
};
pub const BK_DAY_BLUE_0: Color = Color::Rgb {
    r: 43,
    g: 56,
    b: 85,
};

pub const FG_DAY_GREEN_0: Color = Color::Rgb {
    r: 100,
    g: 195,
    b: 75,
};
pub const BK_DAY_GREEN_0: Color = Color::Rgb {
    r: 38,
    g: 78,
    b: 28,
};

pub const FG_DAY_GREEN_1: Color = Color::Rgb {
    r: 80,
    g: 155,
    b: 55,
};
pub const BK_DAY_GREEN_1: Color = Color::Rgb {
    r: 30,
    g: 62,
    b: 20,
};

pub const FG_DAY_GREEN_2: Color = Color::Rgb {
    r: 65,
    g: 105,
    b: 45,
};
pub const BK_DAY_GREEN_2: Color = Color::Rgb {
    r: 26,
    g: 42,
    b: 16,
};

pub const FG_DAY_GREEN_3: Color = Color::Rgb {
    r: 50,
    g: 72,
    b: 35,
};
pub const BK_DAY_GREEN_3: Color = Color::Rgb {
    r: 20,
    g: 28,
    b: 12,
};

pub const FG_DAY_GREY_2: Color = Color::Rgb {
    r: 135,
    g: 120,
    b: 105,
};
pub const BK_DAY_GREY_2: Color = Color::Rgb {
    r: 52,
    g: 46,
    b: 38,
};

pub const FG_DAY_GREY_1: Color = Color::Rgb {
    r: 175,
    g: 158,
    b: 140,
};
pub const BK_DAY_GREY_1: Color = Color::Rgb {
    r: 68,
    g: 60,
    b: 50,
};

pub const FG_DAY_GREY_0: Color = Color::Rgb {
    r: 225,
    g: 208,
    b: 190,
};
pub const BK_DAY_GREY_0: Color = Color::Rgb {
    r: 88,
    g: 80,
    b: 70,
};

pub const FG_DAY_WHITE: Color = Color::Rgb {
    r: 255,
    g: 248,
    b: 235,
};
pub const BK_DAY_WHITE: Color = Color::Rgb {
    r: 108,
    g: 98,
    b: 86,
};

// NIGHT PALETTE
pub const FG_NIGHT_BROWN: Color = Color::Rgb {
    r: 105,
    g: 75,
    b: 55,
};
pub const BK_NIGHT_BROWN: Color = Color::Rgb {
    r: 42,
    g: 30,
    b: 22,
};

pub const FG_NIGHT_BLUE_1: Color = Color::Rgb {
    r: 80,
    g: 90,
    b: 180,
};
pub const BK_NIGHT_BLUE_1: Color = Color::Rgb {
    r: 32,
    g: 36,
    b: 72,
};

pub const FG_NIGHT_BLUE_0: Color = Color::Rgb {
    r: 90,
    g: 100,
    b: 190,
};
pub const BK_NIGHT_BLUE_0: Color = Color::Rgb {
    r: 36,
    g: 40,
    b: 76,
};

pub const FG_NIGHT_GREEN_0: Color = Color::Rgb {
    r: 70,
    g: 160,
    b: 90,
};
pub const BK_NIGHT_GREEN_0: Color = Color::Rgb {
    r: 28,
    g: 64,
    b: 36,
};

pub const FG_NIGHT_GREEN_1: Color = Color::Rgb {
    r: 60,
    g: 130,
    b: 70,
};
pub const BK_NIGHT_GREEN_1: Color = Color::Rgb {
    r: 24,
    g: 52,
    b: 28,
};

pub const FG_NIGHT_GREEN_2: Color = Color::Rgb {
    r: 50,
    g: 90,
    b: 55,
};
pub const BK_NIGHT_GREEN_2: Color = Color::Rgb {
    r: 20,
    g: 36,
    b: 22,
};

pub const FG_NIGHT_GREEN_3: Color = Color::Rgb {
    r: 40,
    g: 65,
    b: 45,
};
pub const BK_NIGHT_GREEN_3: Color = Color::Rgb {
    r: 16,
    g: 26,
    b: 18,
};

pub const FG_NIGHT_GREY_2: Color = Color::Rgb {
    r: 75,
    g: 77,
    b: 90,
};
pub const BK_NIGHT_GREY_2: Color = Color::Rgb {
    r: 30,
    g: 31,
    b: 36,
};

pub const FG_NIGHT_GREY_1: Color = Color::Rgb {
    r: 100,
    g: 102,
    b: 115,
};
pub const BK_NIGHT_GREY_1: Color = Color::Rgb {
    r: 40,
    g: 41,
    b: 46,
};

pub const FG_NIGHT_GREY_0: Color = Color::Rgb {
    r: 145,
    g: 147,
    b: 160,
};
pub const BK_NIGHT_GREY_0: Color = Color::Rgb {
    r: 58,
    g: 59,
    b: 64,
};

pub const FG_NIGHT_WHITE: Color = Color::Rgb {
    r: 190,
    g: 195,
    b: 215,
};
pub const BK_NIGHT_WHITE: Color = Color::Rgb {
    r: 76,
    g: 78,
    b: 86,
};
