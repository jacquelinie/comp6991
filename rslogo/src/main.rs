use clap::Parser;
use unsvg::Image;
use std::collections::HashMap;
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

// Count indentations at the start of the line for loops
fn count_indentation(line: &str) -> usize {
    line.chars()
        .take_while(|ch| ch.is_whitespace())
        .count()
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
    let mut variables: HashMap<String, String> = HashMap::new();

    // ========= ASSIGNMENT =========
    // Parse File
    let file_content = std::fs::read_to_string(&file_path).map_err(|_| eprintln!("File not found: {:?}", &file_path))?;
    let mut line_number = 0;
    for line in file_content.lines(){
        // let curr_indentation = count_indentation(line);
        // let mut if_indentation = 0;
        // let mut while_indentation = 0;
        // Skip empty lines or comments
        if line.trim().is_empty() || line.starts_with("//")  {
            continue;
        }
        // if line.starts_with("IF") {
        //     inputs.drain(0..1);
        //     if_indentation = count_indentation(line);
        // }
        // if line.starts_with("WHILE") {
        //     inputs.drain(0..1);
        //     while_indentation = count_indentation(line);
        // }
        // // If loop
        // pub fn if_loop(&mut self, curr_loop: Vec<&str>, image: &mut Image,
        //     variables: &mut HashMap<String, String>, line_number: &i32) {
        //     println!("If Looping");
        //     for line in curr_loop[1..].iter() {
        //         let line_num = line_number + 1;
        //         if let Err(e) = execute_command(self, image, variables, line, &line_num) {
        //             eprintln!("Error: {}", e);
        //             process::exit(1);
        //         }
        //     }
        // }
        // // ================ TASK 3 ================
        // "IF" => { // IF EQ v1 v2
        //     error_extra_arguments(&inputs, 4);
        //     let v1_str = arguments.get(0).ok_or(format!("Error on line {}: Empty line", line_number))?;
        //     let v2_str = arguments.get(1).ok_or(format!("Error on line {}: Empty line", line_number))?;
        //     let v1: i32 = v1_str.parse().map_err(|_| format!("Error on line {}: If statement requires a value.", line_number))?;
        //     let v2: i32 = v2_str.parse().map_err(|_| format!("Error on line {}: If statement requires a value.", line_number))?;

        //     if v1 == v2 {
        //         let mut curr_loop: Vec<&str> = Vec::new();
        //         while inputs[0] != "[" && curr_indentation != if_indentation {
        //             curr_loop.push(line);
        //         }
        //         turtle.if_loop(curr_loop, image, variables, line_number);
        //     }
        // }

        // "WHILE" => { // IF EQ v1 v2

        // }
        line_number += 1;
        if let Err(e) = execute_command(&mut turtle, &mut image, &mut variables, line, &line_number) {
            eprintln!("Error: {}", e);
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
