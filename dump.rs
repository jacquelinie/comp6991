        // Check indentation for loops
        // let curr_indentation = count_indentation(line);
        // let mut if_indentation = 0;
        // let mut while_indentation = 0;
        // let inputs: Vec<&str> = line.split_whitespace().collect();
        // if inputs[0] == "IF" {
        //     if_indentation = count_indentation(line);

        // }
        // if inputs[0] == "WHILE" {
        //     while_indentation = count_indentation(line);
        // }
        // // If loop
        // pub fn if_loop(curr_loop: Vec<&str>, image: &mut Image,
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
        // ================ TASK 3 ================
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

    // Stack to track conditional execution
    let mut condition_stack: Vec<bool> = Vec::new();

    // To store the lines for re-execution inside a loop
    let mut loop_stack: Vec<(i32, Vec<String>)> = Vec::new();

    let file_content = std::fs::read_to_string(&file_path).map_err(|_| eprintln!("File not found: {:?}", &file_path))?;
    let mut line_number: i32 = 1;
    let lines: Vec<&str> = file_content.lines().collect();

    while line_number < lines.len() as i32 {
        let line = lines[line_number as usize];

        // Skip empty lines or comments
        if line.trim().is_empty() || line.starts_with("//") {
            line_number += 1;
            continue;
        }

        let mut inputs: Vec<&str> = line.split_whitespace().collect();

        // Handle ']': End of a block
        if inputs[0] == "]" {
            if condition_stack.pop().is_none() {
                eprintln!("Error: Unknown Command ']' at line {}", line_number);
                process::exit(1);
            }

            // Check if we need to repeat the WHILE loop
            if let Some((_start_line, loop_commands)) = loop_stack.last_mut() {
                if evaluate_condition(&mut turtle, &variables, &mut inputs, &line_number, "WHILE") {
                    // Re-execute the stored loop commands
                    for loop_line in loop_commands.iter() {
                        execute_command(&mut turtle, &mut image, &mut variables, loop_line, &line_number)
                            .map_err(|e| eprintln!("{}", e))?;
                    }
                } else {
                    // Exit the loop if condition becomes false
                    loop_stack.pop();
                }
            }

            line_number += 1;
            continue;
        }

        // Handle 'IF EQ' conditions
        if inputs[0] == "IF" {
            condition_stack.push(evaluate_condition(&mut turtle, &variables, &mut inputs, &line_number, "IF"));
            line_number += 1;
            continue;
        }

        // Handle 'WHILE EQ' loop
        if inputs[0] == "WHILE" {
            let is_true = evaluate_condition(&mut turtle, &variables, &mut inputs, &line_number, "WHILE");
            condition_stack.push(is_true);

            // Store the starting point of the loop
            loop_stack.push((line_number, Vec::new()));
            line_number += 1;
            continue;
        }

        // If inside a WHILE loop, store the commands to re-execute later
        if !loop_stack.is_empty() {
            if let Some((_, loop_commands)) = loop_stack.last_mut() {
                loop_commands.push(line.to_string());
            }
        }

        // Skip command if any condition in the stack is false
        if condition_stack.contains(&false) {
            line_number += 1;
            continue;
        }

        // Execute Command
        if let Err(e) = execute_command(&mut turtle, &mut image, &mut variables, line, &line_number) {
            eprintln!("{}", e);
            process::exit(1);
        }

        line_number += 1;
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

// Evaluate the condition for IF EQ and WHILE EQ
fn evaluate_condition(turtle: &mut Turtle, variables: &HashMap<String, String>, inputs: &mut Vec<&str>, line_number: &i32, command: &str) -> bool {
    // Parse the condition (e.g., "WHILE EQ XCOR 50" or "IF EQ XCOR 50")
    error_extra_arguments(&inputs, 5);
    if inputs.len() < 5 {
        eprintln!("Error: Error on line {}: Invalid format", line_number);
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
