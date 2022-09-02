use std::io::Write;

pub const TRACE: u32 = 5;
pub const DEBUG: u32 = 4;
pub const INFO: u32 = 3;
pub const WARNING: u32 = 2;
pub const ERROR: u32 = 1;
pub const FATAL: u32 = 0;

const LOG_PATH_VAR: &str = "SE_LOG_PATH";
const LOG_LEVEL_VAR: &str = "SE_LOG_LEVEL";

/// Initialize the logger with settings
/// ### Arguments
///
/// - `path` - Path to save log files to. Can be formated according to:
/// 
/// <https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers>
///
/// #### Example
/// `log_%F_%H-%M-%S.log` expands to `log_2022-09-02_06-27-44.log`
///
/// - `level` - Log level:
///     - `TRACE` - 5
///     - `DEBUG` - 4
///     - `INFO` - 3
///     - `WARRNING` - 2
///     - `ERROR` - 1
///     - `FATAL` - 0
/// 
/// ### Notes
/// `%D`, `%x`, `%R`, `%T`, `%X`, `%r`, `%+` should not
/// be used as they contain `/` or `:` which are disallowed in filenames.
/// 
/// If a path is invalid, the default will be used: `unnamed.log`
pub fn init(path: &str, level: u32) {
    set_log_path(&current_time_fmt(path));
    set_log_level(level);
}

/// Log a generic message
pub fn log(message: &str) {
    println!("{message}");
    let mut f = match std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(get_log_path())
    {
        Ok(f) => f,
        Err(e) => {
            println!("Logger: Failed to open file: {e}");
            return;
        }
    };
    match f.write((message.to_string() + "\n").as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            println!("Logger: Failed to write to file: {e}");
            return;
        }
    }
}

/// Log a trace message
pub fn trace(message: &str) {
    log_with_level(message, TRACE);
}
/// Log a debug message
pub fn debug(message: &str) {
    log_with_level(message, DEBUG);
}
/// Log an info message
pub fn info(message: &str) {
    log_with_level(message, INFO);
}
/// Log a warning message
pub fn warning(message: &str) {
    log_with_level(message, WARNING);
}
/// Log an error message
pub fn error(message: &str) {
    log_with_level(message, ERROR);
}
/// Log a fatal message
pub fn fatal(message: &str) {
    log_with_level(message, FATAL);
}

fn log_with_level(message: &str, level: u32) {
    if get_log_level() >= level {
        log(&format!(
            "[{}] [{}] [{}] {}",
            current_time_fmt("%T"),
            level_to_string(level),
            match std::thread::current().name() {
                Some(s) => s,
                None => "unnamed thread",
            },
            message
        ))
    }
}

fn get_log_level() -> u32 {
    match std::env::var(LOG_LEVEL_VAR) {
        Ok(s) => match s.parse::<u32>() {
            Ok(v) => v,
            Err(_) => INFO,
        },
        Err(_) => INFO,
    }
}
fn set_log_level(level: u32) {
    if level >= FATAL && level <= TRACE {
        std::env::set_var(LOG_LEVEL_VAR, level.to_string());
    }
}
fn get_log_path() -> String {
    match std::env::var(LOG_PATH_VAR) {
        Ok(s) => s,
        Err(_) => "unnamed.log".to_string(),
    }
}
fn set_log_path(path: &str) {
    std::env::set_var(LOG_PATH_VAR, path);
}

fn level_to_string(level: u32) -> String {
    match level {
        TRACE => "TRACE",
        DEBUG => "DEBUG",
        INFO => "INFO",
        WARNING => "WARNING",
        ERROR => "ERROR",
        FATAL => "FATAL",
        _ => "",
    }
    .to_string()
}
fn current_time_fmt(fmt: &str) -> String {
    chrono::Local::now().format(fmt).to_string()
}
