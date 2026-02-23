use std::process::Command;

pub fn set_raw_mode() {
    Command::new("stty")
        .arg("raw")
        .arg("-echo")
        .status()
        .expect("Failed to set terminal to raw mode");
}

pub fn reset_mode() {
    Command::new("stty")
        .arg("sane")
        .status()
        .expect("Failed to reset terminal mode");
}

pub fn hide_cursor() {
    print!("\x1b[?25l");
}

pub fn clear_screen() {
    let _ = Command::new("clear").status();
}

pub fn login() -> String {
    let mut input = String::new();
    println!("Login:");
    if std::io::stdin().read_line(&mut input).is_err() {
        eprintln!("Failed to read login input");
        return String::new();
    }
    input.trim().to_string()
}
