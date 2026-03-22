#![allow(dead_code)]

// === ANSI COLORS ===
// Foreground colors
pub const FG_BLACK: &str = "\x1b[30m";
pub const FG_BLACK_BRIGHT: &str = "\x1b[90m";
pub const FG_BLUE: &str = "\x1b[34m";
pub const FG_BLUE_BRIGHT: &str = "\x1b[94m";
pub const FG_BROWN: &str = "\x1b[38;2;139;69;19m";
pub const FG_CYAN: &str = "\x1b[36m";
pub const FG_CYAN_BRIGHT: &str = "\x1b[96m";
pub const FG_GREEN: &str = "\x1b[32m";
pub const FG_GREEN_BRIGHT: &str = "\x1b[92m";
pub const FG_GREEN_DARK: &str = "\x1b[38;2;0;100;0m";
pub const FG_GREEN_DARKER: &str = "\x1b[38;2;0;70;0m";
pub const FG_GREY: &str = "\x1b[38;2;128;128;128m";
pub const FG_GREY_BRIGHT: &str = "\x1b[38;2;192;192;192m";
pub const FG_GREY_GREENISH: &str = "\x1b[38;2;128;138;115m";
pub const FG_LIGHT_BROWN: &str = "\x1b[38;2;205;133;63m";
pub const FG_MAGENTA: &str = "\x1b[35m";
pub const FG_MAGENTA_BRIGHT: &str = "\x1b[95m";
pub const FG_RED: &str = "\x1b[31m";
pub const FG_RED_BRIGHT: &str = "\x1b[91m";
pub const FG_WHITE: &str = "\x1b[37m";
pub const FG_YELLOW: &str = "\x1b[33m";
pub const FG_YELLOW_BRIGHT: &str = "\x1b[93m";

// Background colors
pub const BG_BLACK: &str = "\x1b[40m";
pub const BG_BLACK_BRIGHT: &str = "\x1b[100m";
pub const BG_BLUE: &str = "\x1b[44m";
pub const BG_BLUE_BRIGHT: &str = "\x1b[104m";
pub const BG_BROWN: &str = "\x1b[48;2;139;69;19m";
pub const BG_CYAN: &str = "\x1b[46m";
pub const BG_CYAN_BRIGHT: &str = "\x1b[106m";
pub const BG_GREEN: &str = "\x1b[42m";
pub const BG_GREEN_BRIGHT: &str = "\x1b[102m";
pub const BG_GREEN_DARK: &str = "\x1b[48;2;0;100;0m";
pub const BG_GREEN_DARKER: &str = "\x1b[48;2;0;70;0m";
pub const BG_GREY: &str = "\x1b[48;2;128;128;128m";
pub const BG_GREY_BRIGHT: &str = "\x1b[48;2;192;192;192m";
pub const BG_GREY_GREENISH: &str = "\x1b[48;2;128;138;115m";
pub const BG_LIGHT_BROWN: &str = "\x1b[48;2;205;133;63m";
pub const BG_MAGENTA: &str = "\x1b[45m";
pub const BG_MAGENTA_BRIGHT: &str = "\x1b[105m";
pub const BG_RED: &str = "\x1b[41m";
pub const BG_RED_BRIGHT: &str = "\x1b[101m";
pub const BG_WHITE: &str = "\x1b[47m";
pub const BG_YELLOW: &str = "\x1b[43m";
pub const BG_YELLOW_BRIGHT: &str = "\x1b[103m";

// === ANSI ELEMENTS AND CODES ===
pub const RESET_COLOR: &str = "\x1b[0m";
pub const BLOCK: char = '▀';
