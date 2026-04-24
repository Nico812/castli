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

// DAY PALETTE
const DAY_BLUE_1: Color = Color::Rgb {
    r: 110,
    g: 110,
    b: 190,
};
const DAY_BLUE_0: Color = Color::Rgb {
    r: 120,
    g: 120,
    b: 190,
};
const DAY_GREEN_0: Color = Color::Rgb {
    r: 100,
    g: 190,
    b: 100,
};
const DAY_GREEN_1: Color = Color::Rgb {
    r: 80,
    g: 150,
    b: 80,
};
const DAY_GREEN_2: Color = Color::Rgb {
    r: 55,
    g: 95,
    b: 55,
};
const DAY_GREEN_3: Color = Color::Rgb {
    r: 45,
    g: 70,
    b: 45,
};
const DAY_GREY_2: Color = Color::Rgb {
    r: 75,
    g: 77,
    b: 90,
};
const DAY_GREY_1: Color = Color::Rgb {
    r: 100,
    g: 102,
    b: 115,
};
const DAY_GREY_0: Color = Color::Rgb {
    r: 145,
    g: 147,
    b: 160,
};
const DAY_WHITE: Color = Color::Rgb {
    r: 200,
    g: 202,
    b: 220,
};

// NIGHT PALETTE
const NIGHT_BLUE_1: Color = Color::Rgb {
    r: 70,
    g: 80,
    b: 130,
};
const NIGHT_BLUE_0: Color = Color::Rgb {
    r: 75,
    g: 85,
    b: 135,
};
const NIGHT_GREEN_0: Color = Color::Rgb {
    r: 80,
    g: 110,
    b: 80,
};
const NIGHT_GREEN_1: Color = Color::Rgb {
    r: 65,
    g: 90,
    b: 65,
};
const NIGHT_GREEN_2: Color = Color::Rgb {
    r: 50,
    g: 70,
    b: 50,
};
const NIGHT_GREEN_3: Color = Color::Rgb {
    r: 35,
    g: 50,
    b: 35,
};
const NIGHT_GREY_2: Color = Color::Rgb {
    r: 65,
    g: 65,
    b: 70,
};
const NIGHT_GREY_1: Color = Color::Rgb {
    r: 90,
    g: 90,
    b: 95,
};
const NIGHT_GREY_0: Color = Color::Rgb {
    r: 120,
    g: 120,
    b: 130,
};
const NIGHT_WHITE: Color = Color::Rgb {
    r: 165,
    g: 165,
    b: 175,
};

pub const BLUE_1: DynamicColor = DynamicColor::new(DAY_BLUE_1, NIGHT_BLUE_1);
pub const BLUE_0: DynamicColor = DynamicColor::new(DAY_BLUE_0, NIGHT_BLUE_0);
pub const GREEN_0: DynamicColor = DynamicColor::new(DAY_GREEN_0, NIGHT_GREEN_0);
pub const GREEN_1: DynamicColor = DynamicColor::new(DAY_GREEN_1, NIGHT_GREEN_1);
pub const GREEN_2: DynamicColor = DynamicColor::new(DAY_GREEN_2, NIGHT_GREEN_2);
pub const GREEN_3: DynamicColor = DynamicColor::new(DAY_GREEN_3, NIGHT_GREEN_3);
pub const GREY_2: DynamicColor = DynamicColor::new(DAY_GREY_2, NIGHT_GREY_2);
pub const GREY_1: DynamicColor = DynamicColor::new(DAY_GREY_1, NIGHT_GREY_1);
pub const GREY_0: DynamicColor = DynamicColor::new(DAY_GREY_0, NIGHT_GREY_0);
pub const WHITE: DynamicColor = DynamicColor::new(DAY_WHITE, NIGHT_WHITE);

// Those remain the same
pub const BLACK: DynamicColor = DynamicColor::new(Color::Black, Color::Black);
pub const MAGENTA: DynamicColor = DynamicColor::new(Color::Magenta, Color::Magenta);
pub const RED: DynamicColor = DynamicColor::new(Color::Red, Color::Red);
