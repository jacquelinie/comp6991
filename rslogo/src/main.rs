use clap::Parser;
use std::collections::{HashMap, VecDeque};
use std::process;
use unsvg::Image;
mod turtle;
use turtle::{error_extra_arguments, execute_command, parse_args, Turtle};

/// A struct that defines the command-line arguments using `clap`. This includes:
/// * `file_path`: Path to a text file containing the instructions for the turtle.
/// * `image_path`: Path to an SVG or PNG image where the turtle will draw.
/// * `height`: The height of the image.
/// * `width`: The width of the image.
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

/// The main function that parses the command-line arguments and processes
/// the turtle commands from the file. It initializes the image and turtle,
/// processes the file line by line, and handles conditionals and loops.
/// The program will save the final image in either PNG or SVG format depending
/// on the `image_path` file extension.
///
/// # Returns
/// * `Result<(), ()>` - Returns `Ok(())` if the program runs successfully.
/// * Err(String) - Returns an error when there are invalid arguments, commands or extra arguments.
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
    let file_content = std::fs::read_to_string(&file_path)
        .map_err(|_| eprintln!("File not found: {:?}", &file_path))?;
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
        if line.trim().is_empty() || line.starts_with("//") {
            continue;
        }

        // Increment line
        line_number += 1;
        // Get inputs from each line
        let mut inputs: VecDeque<&str> = line.split_whitespace().collect();

        // Handle ']': End of a loop
        if inputs[0] == "]" {
            // Close loop in condition_stack
            if let Some(execute_loop) = condition_stack.pop() {
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
            match evaluate_condition(&mut turtle, &variables, &mut inputs, &line_number, "IF") {
                Ok(if_bool) => {
                    condition_stack.push(if_bool);
                    loop_stack.push((line_number, line_increment, "IF".to_string()));
                }
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1); // Alternatively, return Err(()) or handle the error appropriately.
                }
            }
            continue;
        }

        // Handle 'WHILE EQ' loop
        if inputs[0] == "WHILE" {
            match evaluate_condition(&mut turtle, &variables, &mut inputs, &line_number, "WHILE") {
                Ok(while_bool) => {
                    condition_stack.push(while_bool);
                    // Store the starting point of the loop
                    loop_stack.push((line_number, line_increment, "WHILE".to_string()));
                }
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1); // Alternatively, return Err(()) or handle the error appropriately.
                }
            }
            continue;
        }

        // Skip command if any condition in the stack is false
        if condition_stack.contains(&false) {
            continue;
        }

        // Execute Command
        if let Err(e) = execute_command(&mut turtle, &mut image, &mut variables, line, &line_number)
        {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    // Parse image
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

/// Evaluates a condition for the `IF` and `WHILE` commands.
///
/// # Arguments
/// * `turtle` - A mutable reference to the `Turtle` object.
/// * `variables` - A hashmap of variables and their values.
/// * `inputs` - A mutable deque of input strings representing the condition.
/// * `line_number` - The current line number, used for error reporting.
/// * `command` - The command being processed (`IF` or `WHILE`).
///
/// # Returns
/// * `bool` - The result of the evaluated condition.
/// * Err(String) - If there is not enough arguments or too many arguments, or invalid arguments.
fn evaluate_condition(
    turtle: &mut Turtle,
    variables: &HashMap<String, String>,
    inputs: &mut VecDeque<&str>,
    line_number: &i32,
    command: &str,
) -> Result<bool, String> {
    // Check for bool
    // (e.g., "IF :VARIABLE [")
    let comparisons = ["EQ", "NE", "AND", "OR"];

    // Doesn't contain a comparison => evaluate for bool
    if !inputs.iter().any(|&input| comparisons.contains(&input)) {
        inputs.pop_front();
        inputs.pop_back();
        let arguments = parse_args(
            inputs,
            command,
            line_number,
            turtle,
            variables,
            false,
            &mut 0,
        )
        .map_err(|e| format!("Error parsing arguments: {}", e))?;

        let first_arg = arguments
            .first()
            .ok_or_else(|| format!("Error: No argument provided on line {}", line_number))?;

        // Check if the argument is a valid boolean
        match first_arg.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(format!(
                "Error: Invalid boolean value '{}' on line {}",
                first_arg, line_number
            )),
        }
    } else {
        // Parse the condition (e.g., "IF EQ XCOR 50 [")
        if inputs.len() < 5 {
            return Err(format!("Error on line {}: Not enough inputs", line_number));
        }

        let instructions: Vec<&str> = inputs.drain(0..2).collect(); // remove "IF / WHILE  EQ / AND / OR"
        inputs.pop_back(); // remove "["
        let arguments = parse_args(
            inputs,
            command,
            line_number,
            turtle,
            variables,
            false,
            &mut 0,
        )
        .map_err(|e| format!("Error parsing arguments: {}", e))?;
        error_extra_arguments(inputs, &arguments, 5);

        let v1 = arguments.first();
        let v2 = arguments.get(1);

        // Check the comparison
        if instructions.contains(&"OR") {
            // OR
            let v1_bool = parse_bool(v1, line_number)?;
            let v2_bool = parse_bool(v2, line_number)?;
            Ok(v1_bool || v2_bool)
        } else if instructions.contains(&"AND") {
            // AND
            let v1_bool = parse_bool(v1, line_number)?;
            let v2_bool = parse_bool(v2, line_number)?;
            Ok(v1_bool && v2_bool)
        } else if instructions.contains(&"NE") {
            // NE
            Ok(v1 != v2)
        } else if instructions.contains(&"EQ") {
            // EQ
            Ok(v1 == v2)
        } else {
            Err(format!(
                "Error: Invalid comparison operator on line {}",
                line_number
            ))
        }
    }
}

/// Helper function to parse booleans and raise errors if invalid
///
/// Arguments:
/// value - The string to be parsed
/// line_number - The line number where the value appears on
///
/// Returns:
/// * Result<bool, String> - If the value parses as a bool
/// * Err(String) - If the value is not a bool
fn parse_bool(value: Option<&String>, line_number: &i32) -> Result<bool, String> {
    match value {
        Some(s) if s == "true" => Ok(true),
        Some(s) if s == "false" => Ok(false),
        Some(s) => Err(format!(
            "Error: Invalid boolean value '{}' on line {}",
            s, line_number
        )),
        None => Err(format!(
            "Error: Missing boolean value on line {}",
            line_number
        )),
    }
}
