use clap::Parser;
use unsvg::Image;
use std::collections::{HashMap, VecDeque};
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

// Main Function
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
    let lines: Vec<&str> = file_content.lines().collect();
    let mut line_number: i32 = 0;
    let mut line_increment = 0; // Need line increment because empty lines don't count as 'line_number', but need to continue in the file

    // Stack to track conditional execution
    let mut condition_stack: Vec<bool> = Vec::new();
    let mut loop_stack: Vec<(i32, usize, String)> = Vec::new();

    // Loop and parse lines
    while line_increment < lines.len() {
        let line = lines[line_increment];
        line_increment += 1;
        // Skip empty lines or comments
        if line.trim().is_empty() || line.starts_with("//")  {
            continue;
        }

        // Increment line
        line_number += 1;
        // Get inputs from each line
        let mut inputs: VecDeque<&str> = line.split_whitespace().collect();

        // Handle ']': End of a loop
        if inputs[0] == "]" {
            println!("Checking loops...");
            // Close loop in condition_stack
            if let Some (execute_loop) = condition_stack.pop() {
                // Check loop identity
                if let Some((line_n, line_i, text)) = loop_stack.pop() {
                // If while and execute_loop == true, jump back to check
                    if text == *"WHILE" && execute_loop {
                        line_number = line_n - 1;
                        line_increment = line_i - 1;
                    }
                } else {
                    eprintln!("Error: Mismatched ']' at line {}", line_number);
                    process::exit(1);
            }
            } else {
                eprintln!("Error: Mismatched ']' at line {}", line_number);
                process::exit(1);
            }
            continue;
        }

        // Handle 'IF EQ' conditions
        if inputs[0] == "IF" {
            let if_bool = evaluate_condition(&mut turtle, &variables, &mut inputs, &line_number, "IF");
            println!("Adding IF bool: {if_bool}");
            condition_stack.push(if_bool);
            loop_stack.push((line_number, line_increment, "IF".to_string()));
            continue;
        }

        // Handle 'WHILE EQ' loop
        if inputs[0] == "WHILE" {
            let while_bool = evaluate_condition(&mut turtle, &variables, &mut inputs, &line_number, "WHILE");
            println!("Adding WHILE bool: {while_bool}");
            condition_stack.push(while_bool);

            // Store the starting point of the loop
            loop_stack.push((line_number, line_increment, "WHILE".to_string()));
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
        process::exit(1);
    }
    Ok(())
}

// Evaluate the condition for IF and WHILE
fn evaluate_condition(turtle: &mut Turtle, variables: &HashMap<String, String>, inputs: &mut VecDeque<&str>, line_number: &i32, command: &str) -> bool {
    // Check for bool
    // (e.g., "IF :VARIABLE [")
    println!("CURR INPUTS: {:?}", inputs);
    let comparisons = ["EQ", "NE", "AND", "OR"];

    // Doesn't contain a comparison => eval for bool
    if !inputs.iter().any(|&input| comparisons.contains(&input)) {
        inputs.pop_front();
        inputs.pop_back();
        let arguments = match parse_args(inputs, command, line_number, turtle, variables, false, &mut 0) {
            Ok(args) => args,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
        println!("SMALL ARGUMENTS: {:?}", arguments);
        return arguments.first().map(|s| s == "true").unwrap_or(false);
    }

    // Parse the condition
    // (e.g., "IF EQ XCOR 50 [")
    if inputs.len() < 5 {
        eprintln!("Error: Error on line {}: Empty line", line_number);
        process::exit(1);
    }
    let instructions: Vec<&str> = inputs.drain(0..2).collect(); // remove "IF / WHILE  EQ / AND / OR"
    inputs.pop_back(); // remove "["
    let arguments = match parse_args(inputs, command, line_number, turtle, variables, false, &mut 0) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    error_extra_arguments(inputs, &arguments, 5);
    let v1 = arguments.first();
    let v2 = arguments.get(1);

    // Check the equality
    if instructions.contains(&"OR") {
        // OR
        v1.map(|s| s == "true").unwrap_or(false) || v2.map(|s| s == "true").unwrap_or(false)
    } else if instructions.contains(&"AND") {
        // AND
        v1.map(|s| s == "true").unwrap_or(false) && v2.map(|s| s == "true").unwrap_or(false)
    } else if instructions.contains (&"NE") {
        // NE
        v1 != v2
    } else {
        // EQ
        v1 == v2
    }

}
