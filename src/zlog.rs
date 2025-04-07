use crate::Settings;
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
/// let settings = Settings {
///     ...
///     is_verbose: true,
///     is_no_color: false,
/// };
/// zlog::verbose("This is a log only shown when the --verbose flag is passed.", &settings);
/// ```
pub fn verbose(message: &str, settings: &Settings) {
    if settings.is_verbose {
        if settings.is_no_color {
            println!("[VERBOSE] {}", message);
        } else {
            println!("{} {}", "[VERBOSE]".purple(), message.purple());
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
/// let settings = Settings {
///     ...
///     is_verbose: true,
///     is_no_color: false,
/// };
/// zlog::err("This is an error message", &settings);
/// ```
pub fn err(message: &str, settings: &Settings) {
    if settings.is_verbose {
        if settings.is_no_color {
            println!("[ERROR] {}", message);
        } else {
            println!("{} {}", "[ERROR]".red(), message.red());
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
/// let settings = Settings {
///     ...
///     is_verbose: true,
///     is_no_color: false,
/// };
/// zlog::warn("This is an error message", &settings);
/// ```
pub fn warn(message: &str, settings: &Settings) {
    if settings.is_verbose {
        if settings.is_no_color {
            println!("[WARNING] {}", message);
        } else {
            println!("{} {}", "[WARNING]".yellow(), message.yellow());
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
/// let settings = Settings {
///     ...
///     is_verbose: true,
///     is_no_color: false,
/// };
/// zlog::log("This is a log message", &settings);
/// ```
pub fn log(message: &str, settings: &Settings) {
    if settings.is_verbose {
        if settings.is_no_color {
            println!("[LOG] {}", message);
        } else {
            println!("{} {}", "[LOG]".green(), message.green());
        }
    }
}
