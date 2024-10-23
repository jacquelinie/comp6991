use std::collections::{HashMap, VecDeque};
use std::process;
use unsvg::Image;
mod turtle;
use turtle::Turtle;


// If there are extra arguments, return error
pub fn error_extra_arguments(
    inputs: &mut VecDeque<&str>,
    arguments: &Vec<String>,
    num_inputs: usize,
) {
    println!("{:?}", arguments);
    // Check for math in inputs
    if arguments.len() <= num_inputs {
        return;
    }
    inputs.pop_front(); // Remove command

    // Get extra arguments and print
    let extra_args: Vec<&str> = inputs.drain(num_inputs..).collect();
    let extra_args_debug: Vec<String> = extra_args.iter().map(|arg| format!("{:?}", arg)).collect();
    let extra_args_str = extra_args_debug.join(", "); // Format Arguments
    eprintln!("Error: Error: Extra arguments: [{}]", extra_args_str);
    process::exit(1);
}

// Match Queries and return value
pub fn parse_queries(turtle: &mut Turtle, input: &str) -> i32 {
    match input {
        "XCOR" => turtle.x,
        "YCOR" => turtle.y,
        "HEADING" => turtle.heading,
        "COLOR" => turtle.pen_color_code,
        _ => 0,
    }
}

// Parse all math related expressions (+, -, *, /, GT, LT, EQ, NE, AND, OR)
pub fn parse_math(
    turtle: &mut Turtle,
    instruction: &str,
    inputs: &mut VecDeque<&str>,
    command: &str,
    line_number: &i32,
    variables: &HashMap<String, String>,
    inputs_i: &mut i32,
) -> Result<String, String> {
    println!("Math inputs: {:?}", inputs);
    let math_args = parse_args(
        inputs,
        command,
        line_number,
        turtle,
        variables,
        true,
        inputs_i,
    )?;
    println!("Curr math: {} {:?}", instruction, math_args);
    let v1_str = math_args
        .first()
        .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
    let v2_str = math_args
        .get(1)
        .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;

    // Check for comparison
    let math_comparisons = ["EQ", "NE", "AND", "OR"];
    if math_comparisons.contains(&instruction) {
        let result = match instruction {
            "EQ" => v1_str == v2_str,
            "NE" => v1_str != v2_str,
            "AND" => {
                let v1_bool = v1_str
                    .parse::<bool>()
                    .map_err(|_| format!("Invalid boolean string: {}", v1_str))?;
                let v2_bool = v1_str
                    .parse::<bool>()
                    .map_err(|_| format!("Invalid boolean string: {}", v1_str))?;
                return Ok((v1_bool && v2_bool).to_string());
            }
            "OR" => {
                let v1_bool = v1_str
                    .parse::<bool>()
                    .map_err(|_| format!("Invalid boolean string: {}", v1_str))?;
                let v2_bool = v1_str
                    .parse::<bool>()
                    .map_err(|_| format!("Invalid boolean string: {}", v1_str))?;
                return Ok((v1_bool || v2_bool).to_string());
            }
            _ => {
                return Err(format!(
                    "Error: Error on line {}, Unknown operator: {}",
                    line_number, instruction
                ))
            }
        };
        return Ok(result.to_string());
    }

    // Check for
    let math_operations = ["+", "-", "*", "/", "GT", "LT"];
    if math_operations.contains(&instruction) {
        let v1: i32 = v1_str.parse().map_err(|_| {
            format!(
                "Error: Error on line {}: Math requires a value.",
                line_number
            )
        })?;
        let v2: i32 = v2_str.parse().map_err(|_| {
            format!(
                "Error: Error on line {}: Math requires a value.",
                line_number
            )
        })?;
        if instruction == "GT" {
            return Ok((v1 > v2).to_string());
        }
        if instruction == "LT" {
            return Ok((v1 < v2).to_string());
        }
        let result = match instruction {
            "+" => v1 + v2,
            "-" => v1 - v2,
            "*" => v1 * v2,
            "/" => {
                // Check for division by zero
                if v2 == 0 {
                    return Err(format!(
                        "Error: Error on line {}: Cannot divide by zero",
                        line_number
                    ));
                }
                return Ok((v1 / v2).to_string());
            }
            _ => {
                return Err(format!(
                    "Error: Error on line {}, Unknown operator: {}",
                    line_number, instruction
                ))
            }
        };
        return Ok(result.to_string());
    }
    Err(format!(
        "Error: Error on line {}, Unknown operator: {}",
        line_number, instruction
    ))
}

// Parse each arguments from inputs
pub fn parse_args(
    inputs: &mut VecDeque<&str>,
    command: &str,
    line_number: &i32,
    turtle: &mut Turtle,
    variables: &HashMap<String, String>,
    parsing_math: bool,
    inputs_i: &mut i32,
) -> Result<Vec<String>, String> {
    let mut arguments = Vec::new();

    // While input stack is empty, continue parsing
    while !inputs.is_empty() {
        // Check that math is fulfilled:
        if parsing_math && arguments.len() == 2 {
            return Ok(arguments);
        }

        let input: &str = inputs
            .pop_front()
            .expect("Expected a &str input, but inputs is empty.");
        // Math
        let operators = ["*", "-", "+", "/", "GT", "LT", "OR", "AND", "EQ", "NE"];
        if operators.contains(&input) {
            // Handle math expressions
            println!("inserting inputs: {:?}", inputs);
            let result: String = parse_math(
                turtle,
                input,
                inputs,
                command,
                line_number,
                variables,
                inputs_i,
            )?;
            arguments.push(result);
        }
        // Queries - (XCOR, YCOR, HEADING, COLOR)
        else if input == "XCOR" || input == "YCOR" || input == "HEADING" || input == "COLOR" {
            if command == "ADDASSIGN" && *inputs_i == 0 {
                arguments.push(input.to_string());
                arguments.push(parse_queries(turtle, input).to_string());
            } else {
                arguments.push(parse_queries(turtle, input).to_string());
            }
        }
        // " variable
        else if input.starts_with('"') {
            let mut var_name = input.trim_start_matches('"');
            if var_name == "TRUE" {
                var_name = "true";
            } else if var_name == "false" {
                var_name = "false";
            }

            // ADDASSIGN requires getting value of variable if possible
            if command == "ADDASSIGN" && *inputs_i == 0 {
                let arg = variables
                    .get(var_name)
                    .ok_or(format!(
                        "Error on line {}: Could not find variable: {}",
                        line_number, var_name
                    ))?
                    .clone();
                arguments.push(var_name.to_string());
                arguments.push(arg.to_string());
            } else {
                arguments.push(var_name.to_string());
            }
        }
        // : variable
        else if input.starts_with(':') {
            let var_name = input.trim_start_matches(':');
            let mut value = variables
                .get(var_name)
                .ok_or(format!(
                    "Error: Error on line {}: Could not find variable: {}",
                    line_number, var_name
                ))?
                .clone();
            // Parse value, check for true and false
            println!("Curr :variable value: {}", value);
            if value.parse::<i32>().is_err() {
                if value == "TRUE" || value == "true" {
                    value = true.to_string();
                } else if value == "FALSE" || value == "false" {
                    value = false.to_string();
                } else {
                    value = variables
                        .get(&value)
                        .ok_or(format!(
                            "Error: Error on line {}: Could not find variable: {}",
                            line_number, var_name
                        ))?
                        .clone();
                }
            }
            // Push value into arguments
            if command == "MAKE" && *inputs_i == 0 {
                arguments.push(var_name.to_string());
            } else if command == "ADDASSIGN" && *inputs_i == 0 {
                arguments.push(var_name.to_string());
                arguments.push(value);
            } else {
                arguments.push(value);
            }
        } else {
            return Err(format!(
                "Error: Error on line {}, Unknown command: {}",
                line_number, input
            ));
        }
        // Increment inputs_i
        *inputs_i += 1;
    }
    Ok(arguments)
}

// Execute command line arguments
pub fn execute_command(
    turtle: &mut Turtle,
    image: &mut Image,
    variables: &mut HashMap<String, String>,
    line: &str,
    line_number: &i32,
) -> Result<(), String> {
    // Initiate inputs, og_inputs for extra argument printing and command
    let mut inputs: VecDeque<&str> = line.split_whitespace().collect();
    let mut og_inputs: VecDeque<&str> = line.split_whitespace().collect();
    let command = inputs[0];
    inputs.drain(0..1);

    // Get arguments
    let arguments = parse_args(
        &mut inputs,
        command,
        line_number,
        turtle,
        variables,
        false,
        &mut 0,
    )?;

    // Not comment
    match command {
        // ============= TASK 1 =============
        "PENUP" => {
            // Just pen up
            error_extra_arguments(&mut og_inputs, &arguments, 0);
            turtle.pen_up();
        }
        "PENDOWN" => {
            // Just pen down
            error_extra_arguments(&mut og_inputs, &arguments, 0);
            turtle.pen_down();
        }
        "FORWARD" => {
            // Forward dist
            println!("{:?}", arguments);
            error_extra_arguments(&mut og_inputs, &arguments, 1);
            let distance_str = arguments
                .first()
                .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            let distance: i32 = distance_str.parse().map_err(|_| {
                format!(
                    "Error: Error on line {}: Drawing requires an integer argument",
                    line_number
                )
            })?;
            turtle.move_forward(distance, image);
        }
        "BACK" => {
            // Back dist
            error_extra_arguments(&mut og_inputs, &arguments, 1);
            let distance_str = arguments
                .first()
                .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            let distance: i32 = distance_str.parse().map_err(|_| {
                format!(
                    "Error: Error on line {}: Drawing requires an integer argument",
                    line_number
                )
            })?;
            turtle.move_back(distance, image);
        }
        "LEFT" => {
            // Left dist
            error_extra_arguments(&mut og_inputs, &arguments, 1);
            let distance_str = arguments
                .first()
                .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            let distance: i32 = distance_str.parse().map_err(|_| {
                format!(
                    "Error: Error on line {}: Drawing requires an integer argument",
                    line_number
                )
            })?;
            turtle.left(distance, image);
        }
        "RIGHT" => {
            // Right dist
            error_extra_arguments(&mut og_inputs, &arguments, 1);
            let distance_str = arguments
                .first()
                .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            let distance: i32 = distance_str.parse().map_err(|_| {
                format!(
                    "Error: Error on line {}: Drawing requires an integer argument",
                    line_number
                )
            })?;
            turtle.right(distance, image);
        }
        "SETPENCOLOR" => {
            // Setpencolor color
            error_extra_arguments(&mut og_inputs, &arguments, 1);
            let color = arguments
                .first()
                .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            let color_code: usize = color.parse().map_err(|_| {
                format!(
                    "Error: Error on line {}: Invalid color: {}",
                    line_number, color
                )
            })?;
            turtle.pen_color_code = color_code.try_into().unwrap_or_else(|_| {
                panic!("The usize value is too large to fit into an i32");
            });
            turtle.set_pen_color(color_code)?; // TODO: if let Err =
        }
        "TURN" => {
            // Turn degrees
            error_extra_arguments(&mut og_inputs, &arguments, 1);
            let degree_str = arguments
                .first()
                .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            let degree: i32 = degree_str.parse().map_err(|_| {
                format!(
                    "Error: Error on line {}: Turning requires an integer.",
                    line_number
                )
            })?;
            turtle.turn(degree);
        }
        "SETHEADING" => {
            // Setheading degrees
            error_extra_arguments(&mut og_inputs, &arguments, 1);
            let degree_str = arguments
                .first()
                .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            let degree: i32 = degree_str.parse().map_err(|_| {
                format!(
                    "Error: Error on line {}: Setting heading requires an integer.",
                    line_number
                )
            })?;
            turtle.set_heading(degree);
        }
        "SETX" => {
            // Setx x
            error_extra_arguments(&mut og_inputs, &arguments, 1);
            let position_str = arguments
                .first()
                .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            let position: i32 = position_str.parse().map_err(|_| {
                format!(
                    "Error: Error on line {}: Setting position requires an integer.",
                    line_number
                )
            })?;
            turtle.set_x(position);
        }
        "SETY" => {
            // Sety y
            error_extra_arguments(&mut og_inputs, &arguments, 1);
            let position_str = arguments
                .first()
                .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            let position: i32 = position_str.parse().map_err(|_| {
                format!(
                    "Error: Error on line {}: Setting position requires an integer.",
                    line_number
                )
            })?;
            turtle.set_y(position);
        }
        // ================ TASK 2 ================
        "MAKE" => {
            // Make var_name value
            error_extra_arguments(&mut og_inputs, &arguments, 2);
            let var_name_str = arguments
                .first()
                .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            let var_val_str = arguments
                .get(1)
                .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            turtle.make(var_name_str, var_val_str, variables);
        }
        "ADDASSIGN" => {
            // AddAssign v1 v2 | ADDASSIGN "forwardDist :dist | arguments = [val_name, v1, v2]
            error_extra_arguments(&mut og_inputs, &arguments, 3);
            let var_name_str = arguments
                .first()
                .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            let v1_str = arguments
                .get(1)
                .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            let v2_str = arguments
                .get(2)
                .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            let v1: i32 = v1_str.parse().map_err(|_| {
                format!(
                    "Error: Error on line {}: Making variable requires a value.",
                    line_number
                )
            })?;
            let v2: i32 = v2_str.parse().map_err(|_| {
                format!(
                    "Error: Error on line {}: Making variable requires a value.",
                    line_number
                )
            })?;
            turtle.make(var_name_str, &(v1 + v2).to_string(), variables);
        }
        _ => {
            return Err(format!(
                "Error: Error on line {}: Unknown command: {}",
                line_number, command
            ))
        }
    }
    Ok(())
}
