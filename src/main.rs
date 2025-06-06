use colored::Colorize;
use std::env;
use std::fs::File;
use std::io::prelude::*;
include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

mod parser;
mod tokenizer;
mod zlog;

const VERSION: &str = "0.0.1-dev";
const BUILD_ID: &str = BUILD_TIMESTAMP;
const NAME: &str = "zinc";

// Compiler settings
#[derive(Clone, Default)]
struct CSettings {
    is_verbose: bool,
    is_print_tokens: bool,
    is_no_color: bool,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut src: String = String::new();
    let mut c_settings: CSettings = CSettings::default();

    if args.len() >= 2 {
        let mut input_file_str: String = String::new();

        for arg in &args {
            if arg == "--help" || arg == "-h" {
                println!(
                    "Usage: zinc [options] file\nOptions:\n\t-h,\t--help\t\t\tDisplay this information.\n\t-v,\t--version\t\tPrint the version of ZINC\n\t--vb,\t--verbose\t\tPrint verbose logs.\n\t--pt,\t--print-tokens\t\tPrints the output of the tokenizer.\n\t--no-color\t\t\tDisable color output."
                );
                return Ok(());
            } else if arg == "--version" || arg == "-v" {
                println!("zinc (ZINC) {VERSION} {BUILD_ID}");
                return Ok(());
            } else if arg == "--verbose" || arg == "--vb" {
                c_settings.is_verbose = true;
            } else if arg == "--print-tokens" || arg == "--pt" {
                c_settings.is_print_tokens = true;
            } else if arg == "--no-color" || arg == "--nc" {
                c_settings.is_no_color = true;
            } else if arg.starts_with('-') {
                zlog::warn(&format!("Unknown argument `{}`", arg), &c_settings);
            } else {
                if arg != NAME {
                    input_file_str = arg.to_string();
                }
            }
        }

        if c_settings.is_verbose {
            zlog::verbose("Running in verbose mode.", &c_settings);
            if c_settings.is_no_color {
                zlog::verbose("Running without color output.", &c_settings);
            }
            if c_settings.is_print_tokens {
                zlog::verbose("Printing generated tokens set to `true`", &c_settings);
            }
        }

        if input_file_str != "" {
            let src_path: String;
            let cwd = std::env::current_dir().unwrap();

            // Handle different ways to handle file input
            if args[1].starts_with("./") {
                src_path = cwd.display().to_string() + &input_file_str[1..];
            } else if args[1].starts_with('/') {
                src_path = cwd.display().to_string() + &input_file_str;
            } else {
                src_path = cwd.display().to_string() + "/" + &input_file_str;
            }

            zlog::verbose(
                &format!("Absolute source file path: {}", src_path.to_string()),
                &c_settings,
            );

            match read_file_to_string(&src_path, &mut src) {
                Ok(()) => {}
                Err(e) => {
                    zlog::err(
                        &format!("Failed to read file contents to string due to error {}", e),
                        &c_settings,
                    );
                    return Err(e);
                }
            }

            let mut tokenizer: tokenizer::Tokenizer = tokenizer::Tokenizer::new(src, &c_settings);
            match tokenizer.tokenize() {
                Ok(tokens) => {
                    if c_settings.is_print_tokens {
                        // Print the tokens out
                        let all_token_string: String = tokens
                            .iter()
                            .map(|token| format!("{:#?}", token))
                            .collect::<Vec<String>>()
                            .join("\n");
                        zlog::log(&all_token_string, &c_settings);
                    }
                }
                Err(e) => {
                    zlog::err(
                        &format!("Failed to tokenize source file contents due to error {}", e),
                        &c_settings,
                    );
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
                }
            }
        } else {
            zlog::err(
                &format!(
                    "{}: fatal error: No input file(s). Type --help for usage.",
                    NAME.green()
                ),
                &c_settings,
            );
        }
    } else {
        zlog::err(
            &format!(
                "{}: fatal error: Incorrect Usage. Type --help for usage.",
                NAME.green()
            ),
            &c_settings,
        );
    }

    Ok(())
}

/// # Read File to String
///
/// Reads the contents of a file into a specified string.
///
/// # Arguments
///
/// * `path` - A string slice containing the **absolute** path to a file.
/// * `output_string` - A string where the file contents will be placed.
///
/// # Example
///
/// ```
/// read_file_to_string("/home/zinc/code/zinc/src/main.zc".to_string(), src_contents);
/// ```
fn read_file_to_string(path: &str, output_string: &mut String) -> std::io::Result<()> {
    let mut file: File = File::open(path)?;
    file.read_to_string(output_string)
        .expect("zinc: fatal error: Failed to read file.");

    Ok(())
}

#[cfg(test)]
mod tests {}
