use clap::Parser;
use unsvg::Image;
use std::process;
mod turtle;
use turtle::{Turtle, execute_command};

/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    let mut image = Image::new(width, height);
    let mut turtle = Turtle::new(width, height);

    // ========= TASK 1 =========
    // Parse File
    let file_content = std::fs::read_to_string(&file_path).map_err(|_| eprintln!("File not found: {:?}", &file_path))?;
    let mut line_number = 0;
    for line in file_content.lines(){
        line_number += 1;
        // Skip empty lines or lines that are just whitespace
        if line.trim().is_empty() {
            continue;
        }
        if let Err(e) = execute_command(&mut turtle, &mut image, line, &line_number) {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    match image_path.extension().and_then(|s| s.to_str()) {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving svg: {e}");
                return Err(());
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving png: {e}");
                return Err(());
            }
        }
        _ => {
            eprintln!("File extension not supported");
            return Err(());
        }
    }

    Ok(())
}
