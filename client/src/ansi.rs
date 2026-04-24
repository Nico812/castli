use crossterm::style::Color;

#[derive(Copy, Clone, PartialEq)]
pub struct DynamicColor {
    day: Color,
    night: Color,
}

impl DynamicColor {
    pub const fn new(day: Color, night: Color) -> Self {
        Self { day, night }
    }

    pub fn get(&self, night: bool) -> Color {
        match night {
            false => self.day,
            true => self.night,
        }
    }
}

pub const BLACK: Color = Color::Black;
pub const MAGENTA: Color = Color::Magenta;
pub const RED: Color = Color::Red;
pub const WHITE: Color = Color::White;
pub const GREEN: Color = Color::Green;

// DAY PALETTE
pub const DAY_BLUE_1: Color = Color::Rgb {
    r: 110,
    g: 110,
    b: 190,
};
pub const DAY_BLUE_0: Color = Color::Rgb {
    r: 120,
    g: 120,
    b: 190,
};
pub const DAY_GREEN_0: Color = Color::Rgb {
    r: 100,
    g: 190,
    b: 100,
};
pub const DAY_GREEN_1: Color = Color::Rgb {
    r: 80,
    g: 150,
    b: 80,
};
pub const DAY_GREEN_2: Color = Color::Rgb {
    r: 55,
    g: 95,
    b: 55,
};
pub const DAY_GREEN_3: Color = Color::Rgb {
    r: 45,
    g: 70,
    b: 45,
};
pub const DAY_GREY_2: Color = Color::Rgb {
    r: 75,
    g: 77,
    b: 90,
};
pub const DAY_GREY_1: Color = Color::Rgb {
    r: 100,
    g: 102,
    b: 115,
};
pub const DAY_GREY_0: Color = Color::Rgb {
    r: 145,
    g: 147,
    b: 160,
};
pub const DAY_WHITE: Color = Color::Rgb {
    r: 200,
    g: 202,
    b: 220,
};

// NIGHT PALETTE
pub const NIGHT_BLUE_1: Color = Color::Rgb {
    r: 70,
    g: 80,
    b: 130,
};
pub const NIGHT_BLUE_0: Color = Color::Rgb {
    r: 75,
    g: 85,
    b: 135,
};
pub const NIGHT_GREEN_0: Color = Color::Rgb {
    r: 80,
    g: 110,
    b: 80,
};
pub const NIGHT_GREEN_1: Color = Color::Rgb {
    r: 65,
    g: 90,
    b: 65,
};
pub const NIGHT_GREEN_2: Color = Color::Rgb {
    r: 50,
    g: 70,
    b: 50,
};
pub const NIGHT_GREEN_3: Color = Color::Rgb {
    r: 35,
    g: 50,
    b: 35,
};
pub const NIGHT_GREY_2: Color = Color::Rgb {
    r: 65,
    g: 65,
    b: 70,
};
pub const NIGHT_GREY_1: Color = Color::Rgb {
    r: 90,
    g: 90,
    b: 95,
};
pub const NIGHT_GREY_0: Color = Color::Rgb {
    r: 120,
    g: 120,
    b: 130,
};
pub const NIGHT_WHITE: Color = Color::Rgb {
    r: 165,
    g: 165,
    b: 175,
};
