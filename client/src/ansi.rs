// === ANSI COLOR MACROS ===

macro_rules! FG_BLACK {
    () => {
        "\x1b[30m"
    };
}
macro_rules! BG_BLACK {
    () => {
        "\x1b[40m"
    };
}
macro_rules! FG_RED {
    () => {
        "\x1b[31m"
    };
}
macro_rules! BG_RED {
    () => {
        "\x1b[41m"
    };
}
macro_rules! FG_GREEN {
    () => {
        "\x1b[32m"
    };
}
macro_rules! BG_GREEN {
    () => {
        "\x1b[42m"
    };
}
macro_rules! FG_YELLOW {
    () => {
        "\x1b[33m"
    };
}
macro_rules! BG_YELLOW {
    () => {
        "\x1b[43m"
    };
}
macro_rules! FG_BLUE {
    () => {
        "\x1b[34m"
    };
}
macro_rules! BG_BLUE {
    () => {
        "\x1b[44m"
    };
}
macro_rules! FG_MAGENTA {
    () => {
        "\x1b[35m"
    };
}
macro_rules! BG_MAGENTA {
    () => {
        "\x1b[45m"
    };
}
macro_rules! FG_CYAN {
    () => {
        "\x1b[36m"
    };
}
macro_rules! BG_CYAN {
    () => {
        "\x1b[46m"
    };
}
macro_rules! FG_WHITE {
    () => {
        "\x1b[37m"
    };
}
macro_rules! BG_WHITE {
    () => {
        "\x1b[47m"
    };
}

macro_rules! FG_BRIGHT_BLACK {
    () => {
        "\x1b[90m"
    };
}
macro_rules! BG_BRIGHT_BLACK {
    () => {
        "\x1b[100m"
    };
}
macro_rules! FG_BRIGHT_RED {
    () => {
        "\x1b[91m"
    };
}
macro_rules! BG_BRIGHT_RED {
    () => {
        "\x1b[101m"
    };
}
macro_rules! FG_BRIGHT_GREEN {
    () => {
        "\x1b[92m"
    };
}
macro_rules! BG_BRIGHT_GREEN {
    () => {
        "\x1b[102m"
    };
}
macro_rules! FG_BRIGHT_YELLOW {
    () => {
        "\x1b[93m"
    };
}
macro_rules! BG_BRIGHT_YELLOW {
    () => {
        "\x1b[103m"
    };
}
macro_rules! FG_BRIGHT_BLUE {
    () => {
        "\x1b[94m"
    };
}
macro_rules! BG_BRIGHT_BLUE {
    () => {
        "\x1b[104m"
    };
}
macro_rules! FG_BRIGHT_MAGENTA {
    () => {
        "\x1b[95m"
    };
}
macro_rules! BG_BRIGHT_MAGENTA {
    () => {
        "\x1b[105m"
    };
}
macro_rules! FG_BRIGHT_CYAN {
    () => {
        "\x1b[96m"
    };
}
macro_rules! BG_BRIGHT_CYAN {
    () => {
        "\x1b[106m"
    };
}
macro_rules! FG_BRIGHT_WHITE {
    () => {
        "\x1b[97m"
    };
}
macro_rules! BG_BRIGHT_WHITE {
    () => {
        "\x1b[107m"
    };
}

// Reset / Default
pub const RESET_COLOR: &str = "\x1b[0m";
pub const BLOCK: &str = "▀";

// Game ansi elements. Variants are used when the full terminal character block is occupied by the
// game element (example: two grass tiles one on top of the other)
pub const ERR_COLOR: (&str, &str) = (FG_MAGENTA!(), BG_MAGENTA!());
pub const ERR_VARIANT: &str = "?";

pub const GRASS_COLOR: (&str, &str) = (FG_GREEN!(), BG_GREEN!());
pub const GRASS_VARIANTS: (&str, &str) = (
    concat!(BG_GREEN!(), FG_BRIGHT_GREEN!(), "\""),
    concat!(BG_GREEN!(), " "),
);

pub const WATER_COLOR: (&str, &str) = (FG_BLUE!(), BG_BLUE!());
pub const WATER_VARIANTS: (&str, &str) = (
    concat!(BG_BLUE!(), FG_BRIGHT_BLUE!(), "~"),
    concat!(BG_BLUE!(), " "),
);
