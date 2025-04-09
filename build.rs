use chrono::Utc;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_info.rs");

    let timestamp = Utc::now().format("%Y%m%d%H").to_string();
    let content = format!("pub const BUILD_TIMESTAMP: &str = \"{}\";", timestamp);

    let args: Vec<String> = env::args().collect();
    for arg in args {
        println!("{}", arg);
    }

    fs::write(dest_path, content).unwrap();
}
