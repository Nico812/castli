pub const FG_BLACK: &str = "\x1b[30m";
pub const FG_RED: &str = "\x1b[31m";
pub const FG_GREEN: &str = "\x1b[32m";
pub const FG_YELLOW: &str = "\x1b[33m";
pub const FG_BLUE: &str = "\x1b[34m";
pub const FG_MAGENTA: &str = "\x1b[35m";
pub const FG_WHITE: &str = "\x1b[37m";

pub const BG_BLACK: &str = "\x1b[40m";
pub const BG_GREEN: &str = "\x1b[42m";
pub const BG_BLUE: &str = "\x1b[44m";

pub const FG_BRIGHT_GREEN: &str = "\x1b[92m";
pub const FG_BRIGHT_BLUE: &str = "\x1b[94m";

pub const BG_BRIGHT_MAGENTA: &str = "\x1b[105m";

pub const RESET_COLOR: &str = "\x1b[0m";
pub const BLOCK: char = '\u{2580}';

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TermCell {
    pub ch: char,
    pub fg: &'static str,
    pub bg: &'static str,
}

impl TermCell {
    pub const fn new(ch: char, fg: &'static str, bg: &'static str) -> Self {
        Self { ch, fg, bg }
    }

    pub fn write_to(&self, buf: &mut String) {
        buf.push_str(self.fg);
        buf.push_str(self.bg);
        buf.push(self.ch);
    }
}
