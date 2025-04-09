use crate::CSettings;
use colored::Colorize;

/// # Verbose Log
///
/// Prints a zinc standard log only if [`IS_VERBOSE`] is set to `true`.
///
/// # Arguments
///
/// * `message` - A string slice (`&str`) containing the log to print.
/// * `settings` - A settings struct containing information about the flags passed to the compiler
///
/// # Example
/// ```
/// let c_settings = CSettings {
///     ...
///     is_verbose: true,
///     is_no_color: false,
/// };
/// zlog::verbose("This is a log only shown when the --verbose flag is passed.", &c_settings);
/// ```
pub fn verbose(message: &str, c_settings: &CSettings) {
    if c_settings.is_verbose {
        if c_settings.is_no_color {
            println!("[VERBOSE] {}", message);
        } else {
            println!("{} {}", "[VERBOSE]".purple(), message);
        }
    }
}

/// # Error Log
///
/// Prints a zinc standard error message.
///
/// # Arguments
///
/// * `message` - A string slice (&str) containing the error message to print.
/// * `settings` - A settings struct containing information about the flags passed to the compiler
///
/// # Example
/// ```
/// let c_settings = CSettings {
///     ...
///     is_verbose: true,
///     is_no_color: false,
/// };
/// zlog::err("This is an error message", &c_settings);
/// ```
pub fn err(message: &str, c_settings: &CSettings) {
    if c_settings.is_verbose {
        if c_settings.is_no_color {
            println!("[ERROR] {}", message);
        } else {
            println!("{} {}", "[ERROR]".red(), message);
        }
    }
}

/// # Warning Log
///
/// Prints a zinc standard warning message.
///
/// # Arguments
///
/// * `message` - A string slice (&str) containing the warning message to print.
/// * `settings` - A settings struct containing information about the flags passed to the compiler
///
/// # Example
/// ```
/// let c_settings = CSettings {
///     ...
///     is_verbose: true,
///     is_no_color: false,
/// };
/// zlog::warn("This is an error message", &c_settings);
/// ```
pub fn warn(message: &str, c_settings: &CSettings) {
    if c_settings.is_verbose {
        if c_settings.is_no_color {
            println!("[WARNING] {}", message);
        } else {
            println!("{} {}", "[WARNING]".yellow(), message);
        }
    }
}

/// # Log
///
/// Prints a zinc standard log message.
///
/// # Arguments
///
/// * `message` - A string slice (&str) containing the log message to print.
/// * `settings` - A settings struct containing information about the flags passed to the compiler
///
/// # Example
/// ```
/// let c_settings = CSettings {
///     ...
///     is_verbose: true,
///     is_no_color: false,
/// };
/// zlog::log("This is a log message", &c_settings);
/// ```
pub fn log(message: &str, c_settings: &CSettings) {
    if c_settings.is_verbose {
        if c_settings.is_no_color {
            println!("[LOG] {}", message);
        } else {
            println!("{} {}", "[LOG]".green(), message);
        }
    }
}
