use clap::Parser;
use unsvg::Image;
use std::collections::HashMap;
use std::process;
mod turtle;
use turtle::{Turtle, execute_command, parse_args, error_extra_arguments};

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
// fn count_indentation(line: &str) -> usize {
//     line.chars()
//         .take_while(|ch| ch.is_whitespace())
//         .count()
// }

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

    // Stack to track conditional execution
    let mut condition_stack: Vec<bool> = Vec::new();

    for line in file_content.lines(){
        // Skip empty lines or comments
        if line.trim().is_empty() || line.starts_with("//")  {
            continue;
        }

        // Increment line
        line_number += 1;
        let mut inputs: Vec<&str> = line.split_whitespace().collect();

        // Handle ']': End of a block
        if inputs[0] == "]" {
            if condition_stack.pop().is_none() {
                eprintln!("Error: Mismatched ']' at line {}", line_number);
                process::exit(1);
            }
            continue;
        }

        // Handle 'IF EQ' conditions
        if inputs[0] == "IF" {
            println!("Adding IF bool");
            condition_stack.push(evaluate_condition(&mut turtle, &variables, &mut inputs, &line_number, "IF"));
            println!("Curr Stack: {:?}", condition_stack);
            continue;
        }

        // Handle 'WHILE EQ' loop
        if inputs[0] == "WHILE" {
            println!("Evaluatin in WHILE LOOP");
            let is_true = evaluate_condition(&mut turtle, &variables, &mut inputs, &line_number, "WHILE");
            condition_stack.push(is_true);

            // Store the starting point of the loop
            loop_stack.push((line_number, Vec::new()));
            line_number += 1;
            continue;
        }

        // Skip command if any condition in the stack is false
        if condition_stack.contains(&false) {
            continue;
        }

        // Execute Command
        if let Err(e) = execute_command(&mut turtle, &mut image, &mut variables, line, &line_number) {
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
    // Check that loops are closed
    if !condition_stack.is_empty() {
        eprintln!("Error: Stack not empty at end of program");
        return Err(());
    }
    Ok(())
}

// Evaluate the condition for IF EQ
fn evaluate_condition(turtle: &mut Turtle, variables: &HashMap<String, String>, inputs: &mut Vec<&str>, line_number: &i32, command: &str) -> bool {
    // Parse the condition (e.g., "IF EQ XCOR 50")
    error_extra_arguments(&inputs, 5);
    if inputs.len() < 5 {
        eprintln!("Error: Error on line {}: Empty line", line_number);
        process::exit(1);
    }
    inputs.pop(); // remove "]"
    let arguments = match parse_args(&inputs[2..], command, line_number, turtle, variables) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    return arguments.get(0) == arguments.get(1);
}
