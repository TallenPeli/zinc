use std::env;
use std::fs::File;
use std::sync::OnceLock;
use std::io::prelude::*;
use colored::Colorize;

// mod tokenizer;
mod zlog;

const VERSION: &str = "0.0.1-pre";
const BUILD_ID: &str = "20250320";
const NAME: &str = "zinc";

static IS_VERBOSE: OnceLock<bool> = OnceLock::new();

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut src: String = String::new();

    if args.len() >= 2 {
        let mut input_file_str: String = String::new();

        for arg in &args {
            if arg == "--help" || arg == "-h" {
                println!("Usage: zinc [options] file\nOptions:\n\t-h, --help\t\t\tDisplay this information.\n\t--help=");
                return Ok(())
            } else if arg == "--version" || arg == "-v" {
                println!("zinc (ZINC) {VERSION} {BUILD_ID}\nCopyright (C) 2025 TallenPeli\nThis is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.");
                return Ok(())
            } else if arg == "--verbose" || arg == "--vb" {
                IS_VERBOSE.set(true).expect("Failed to set the `verbose` flag to true");
                zlog::verbose("Flag `verbose` set to `true`");
            } else if arg.starts_with('-') {
                zlog::warn(&format!("Unknown argument `{}`", arg));
            } else {
                input_file_str = arg.to_string();
            }
        }
 
        let src_path: String;
        let cwd = std::env::current_dir().unwrap();

        // Handle different ways to handle file input
        if args[1].starts_with("./") {
            src_path = cwd.display().to_string() + &input_file_str[1..];
        } else if !args[1].starts_with('/') {
            src_path = cwd.display().to_string() + "/" + &input_file_str;
        } else {
            src_path = cwd.display().to_string() + &input_file_str;
        }
        
        zlog::verbose(&format!("Absolute source file path: {}", src_path.to_string().blue()));

        match read_file_to_string(&src_path, &mut src) {
            Ok(()) => {},
            Err(e) => {
                zlog::err(&format!("Failed to read file contents to string: {}", e));
                return Err(e)
            }
        }
    }
    else {
        zlog::err(&format!("{}: fatal error: Incorrect Usage. Type --help for usage.\nTerminated.", NAME.green()));
    }

    // tokenizer::tokenize(src);
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
    file.read_to_string(output_string).expect("zinc: fatal error: Failed to read file.");

    Ok(())
}

#[cfg(test)]
mod tests {
}
