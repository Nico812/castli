use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

static LOGGER: OnceLock<Mutex<BufWriter<File>>> = OnceLock::new();
static START: OnceLock<Instant> = OnceLock::new();

pub fn init() {
    let Some(path) = std::env::var("CASTLI_LOG").ok() else {
        return;
    };
    let file = match File::create(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open log file {path}: {e}");
            return;
        }
    };
    START.get_or_init(Instant::now);
    let _ = LOGGER.set(Mutex::new(BufWriter::new(file)));
}

pub fn write(args: fmt::Arguments<'_>) {
    let Some(logger) = LOGGER.get() else { return };
    let Ok(mut writer) = logger.lock() else {
        return;
    };
    let elapsed = START.get().map_or(0.0, |s| s.elapsed().as_secs_f64());
    let _ = write!(writer, "[{elapsed:>10.3}] ");
    let _ = writer.write_fmt(args);
    let _ = writeln!(writer);
    let _ = writer.flush();
}
