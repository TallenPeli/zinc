use std::env;
use std::fs::File;
use std::sync::OnceLock;
use std::io::prelude::*;
use colored::Colorize;

mod tokenizer;
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
                println!("Usage: zinc [options] file\nOptions:\n
                    \t-h,\t--help\t\t\tDisplay this information.\n
                    \t-v,\t--version\t\tPrint the version of ZINC\n
                    \t--vb,\t--verbose\t\tPrint verbose logs.\n
                    \t");
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
                if arg != NAME {
                    input_file_str = arg.to_string();
                }
            }
        }
 
        if input_file_str != "" {
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

            tokenizer::tokenize(src);
        } else {
            zlog::err(&format!("{}: fatal error: No input file(s). Type --help for usage.", NAME.green()));
        }
    }
    else {
        zlog::err(&format!("{}: fatal error: Incorrect Usage. Type --help for usage.", NAME.green()));
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
    file.read_to_string(output_string).expect("zinc: fatal error: Failed to read file.");

    Ok(())
}

#[cfg(test)]
mod tests {
}
