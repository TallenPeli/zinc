use crate::IS_VERBOSE;
use colored::Colorize;

/// # Verbose Log
///
/// Prints a zinc standard log only if [`IS_VERBOSE`] is set to `true`.
///
/// # Arguments
///
/// * `message` - A string slice (`&str`) containing the log to print.
///
/// # Example
/// ```
/// zlog::verbose("This is a log only shown when `IS_VERBOSE` is true.");
/// ```
pub fn verbose(message: &str) {
    if *IS_VERBOSE.get_or_init(|| false) {
        println!("{}\t{message}", "[VERBOSE]".green());
    }
}

/// # Error Log
///
/// Prints a zinc standard error message.
///
/// # Arguments
///
/// * `message` - A string slice (&str) containing the error message to print.
///
/// # Example
/// ```
/// zlog::err("This is an error message");
/// ```
pub fn err(message: &str) {
    println!("{}\t{message}", "[ERROR]".red());
}

/// # Warning Log
///
/// Prints a zinc standard warning message.
///
/// # Arguments
///
/// * `message` - A string slice (&str) containing the warning message to print.
///
/// # Example
/// ```
/// zlog::warn("This is an error message");
/// ```
pub fn warn(message: &str) {
    println!("{}\t{message}", "[WARNING]".yellow());
}
