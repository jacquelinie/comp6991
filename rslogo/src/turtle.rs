use std::collections::{HashMap, VecDeque};
use std::process;
use unsvg::{Image, COLORS};

// Turtle struct to store information
pub struct Turtle {
    pub x: i32,
    pub y: i32,
    pub heading: i32,
    pub pen_down: bool,
    pub pen_color_code: i32,
    pub pen_color: unsvg::Color, // Color code as defined in unsvg COLORS array
}

// Initialise turtle
impl Turtle {
    /// Creates a new `Turtle` starting at the center of the screen with a default heading and color.
    ///
    /// # Parameters:
    /// - `width`: The width of the drawing area.
    /// - `height`: The height of the drawing area.
    ///
    /// # Returns:
    /// A new Turtle instance.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            x: width as i32 / 2,
            y: height as i32 / 2,
            heading: 0,
            pen_down: false,
            pen_color_code: 7,
            pen_color: COLORS[7], // Default white
        }
    }

    // Make pen up
    pub fn pen_up(&mut self) {
        self.pen_down = false;
    }

    // Make pen down
    pub fn pen_down(&mut self) {
        self.pen_down = true;
    }

    /// Moves the turtle forward by a given distance.
    ///
    /// # Parameters:
    /// - `distance`: The distance to move the turtle.
    /// - `image`: The image on which the turtle is drawing.
    ///
    /// # Errors:
    /// Returns an error if drawing the line on the image fails.
    pub fn move_forward(&mut self, distance: i32, image: &mut Image) {
        let (new_x, new_y) = unsvg::get_end_coordinates(self.x, self.y, self.heading, distance);
        if self.pen_down {
            if let Err(e) =
                image.draw_simple_line(self.x, self.y, self.heading, distance, self.pen_color)
            {
                eprintln!("Error: Error occurred when drawing: {}", e);
                process::exit(1);
            };
        }
        self.x = new_x;
        self.y = new_y;
    }

    /// Moves the turtle backward by a given distance.
    ///
    /// # Parameters:
    /// - `distance`: The distance to move the turtle backward.
    /// - `image`: The image on which the turtle is drawing.
    ///
    /// # Errors:
    /// Returns an error if drawing the line on the image fails.
    pub fn move_back(&mut self, distance: i32, image: &mut Image) {
        self.move_forward(-distance, image);
    }

    /// Rotates the turtle by a given number of degrees.
    ///
    /// # Parameters:
    /// - `degrees`: The number of degrees to turn the turtle.
    pub fn turn(&mut self, degrees: i32) {
        self.heading = (self.heading + degrees) % 360;
    }

    /// Sets the turtle's heading to a specific value.
    ///
    /// # Parameters:
    /// - `degrees`: The heading to set, in degrees.
    pub fn set_heading(&mut self, degrees: i32) {
        self.heading = (degrees) % 360;
    }

    /// Sets the turtle's X-coordinate.
    ///
    /// # Parameters:
    /// - `position`: The X-coordinate to set.
    pub fn set_x(&mut self, position: i32) {
        self.x = position;
    }

    /// Sets the turtle's Y-coordinate.
    ///
    /// # Parameters:
    /// - `position`: The Y-coordinate to set.
    pub fn set_y(&mut self, position: i32) {
        self.y = position;
    }

    /// Moves the turtle left by a given distance.
    ///
    /// # Parameters:
    /// - `distance`: The distance to move left.
    /// - `image`: The image on which the turtle is drawing.
    ///
    /// # Errors:
    /// Returns an error if drawing the line on the image fails.
    pub fn left(&mut self, distance: i32, image: &mut Image) {
        let curr_heading = self.heading;
        self.heading = (self.heading - 90) % 360;
        self.move_forward(distance, image);
        self.heading = curr_heading;
    }

    /// Moves the turtle right by a given distance.
    ///
    /// # Parameters:
    /// - `distance`: The distance to move right.
    /// - `image`: The image on which the turtle is drawing.
    ///
    /// # Errors:
    /// Returns an error if drawing the line on the image fails.
    pub fn right(&mut self, distance: i32, image: &mut Image) {
        let curr_heading = self.heading;
        self.heading = (self.heading + 90) % 360;
        self.move_forward(distance, image);
        self.heading = curr_heading;
    }

    /// Sets the turtle's pen color using a color code.
    ///
    /// # Parameters:
    /// - `color_code`: The color code (0-15) to set the pen color to.
    ///
    /// # Returns:
    /// - `Ok(())` if the color is set successfully.
    /// - `Err(String)` if the color code is invalid.
    pub fn set_pen_color(&mut self, color_code: usize) -> Result<(), String> {
        if color_code < 16 {
            self.pen_color = COLORS[color_code];
            Ok(())
        } else {
            Err(format!("Error: Invalid color code: {}", color_code))
        }
    }

    /// Creates or updates a variable with a given value.
    ///
    /// # Parameters:
    /// - `var_name`: The name of the variable to create or update.
    /// - `var_val`: The value to assign to the variable.
    /// - `variables`: The hash map storing all variables.
    pub fn make(&mut self, var_name: &str, var_val: &str, variables: &mut HashMap<String, String>) {
        // Check for make variable variable case
        if var_val.parse::<i32>().is_err() {
            variables.insert(var_val.to_string(), var_name.to_string());
        }
        variables.insert(var_name.to_string(), var_val.to_string());
    }
}

/// Handles the case when extra arguments are provided for a command.
///
/// # Parameters:
/// - `inputs`: The deque of inputs provided.
/// - `arguments`: The list of parsed arguments.
/// - `num_inputs`: The expected number of inputs.
pub fn error_extra_arguments(inputs: &mut VecDeque<&str>, arguments: &[String], num_inputs: usize) {
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

/// Parse all queries involving XCOR, YCOR, HEADING and COLOR.
///
/// # Parameters:
/// - `turtle`: The turtle instance
/// - `input`: The inputted query
pub fn parse_queries(turtle: &mut Turtle, input: &str) -> i32 {
    match input {
        "XCOR" => turtle.x,
        "YCOR" => turtle.y,
        "HEADING" => turtle.heading,
        "COLOR" => turtle.pen_color_code,
        _ => 0,
    }
}

/// Parses and evaluates mathematical expressions.
///
/// # Arguments
/// * `turtle` - A mutable reference to the `Turtle` object.
/// * `instruction` - The math operation or comparison to be executed.
/// * `inputs` - A mutable deque of input strings.
/// * `command` - The command currently being executed.
/// * `line_number` - Reference to the current line number for error reporting.
/// * `variables` - A hashmap of variables and their values.
/// * `inputs_i` - A mutable reference to the current input index.
///
/// # Returns
/// * `Result<String, String>` - The result of the mathematical operation or a string error.
/// * Err(String) - If there is an invalid bool, missing arguments, invalid arguments or unknown operators.
pub fn parse_math(
    turtle: &mut Turtle,
    instruction: &str,
    inputs: &mut VecDeque<&str>,
    command: &str,
    line_number: &i32,
    variables: &HashMap<String, String>,
    inputs_i: &mut i32,
) -> Result<String, String> {
    let math_args = parse_args(
        inputs,
        command,
        line_number,
        turtle,
        variables,
        true,
        inputs_i,
    )?;
    let v1_str = math_args
        .first()
        .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
    let v2_str = math_args
        .get(1)
        .ok_or(format!("Error: Error on line {}: Empty line", line_number))?;

    // Check for comparisons
    let math_comparisons = ["EQ", "NE", "AND", "OR"];
    if math_comparisons.contains(&instruction) {
        let result = match instruction {
            "EQ" => v1_str == v2_str,
            "NE" => v1_str != v2_str,
            "AND" => {
                // AND bools
                let v1_bool = v1_str
                    .parse::<bool>()
                    .map_err(|_| format!("Invalid boolean string: {}", v1_str))?;
                let v2_bool = v1_str
                    .parse::<bool>()
                    .map_err(|_| format!("Invalid boolean string: {}", v1_str))?;
                return Ok((v1_bool && v2_bool).to_string());
            }
            "OR" => {
                // OR bools
                let v1_bool: bool = v1_str
                    .parse::<bool>()
                    .map_err(|_| format!("Invalid boolean string: {}", v1_str))?;
                let v2_bool: bool = v1_str
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

    // Check for normal math operations
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

/// Parses command-line arguments from the input.
///
/// # Arguments
/// * `inputs` - A mutable deque of input strings to be parsed.
/// * `command` - The command currently being executed.
/// * `line_number` - Reference to the current line number for error reporting.
/// * `turtle` - A mutable reference to the `Turtle` object.
/// * `variables` - A hashmap of variables and their values.
/// * `parsing_math` - A flag to indicate if the function is parsing math expressions.
/// * `inputs_i` - A mutable reference to the current input index.
///
/// # Returns
/// * `Result<Vec<String>, String>` - A vector of parsed arguments or an error string.
/// * Err(String) - If there are invalid arguments, missing arguments or unknown operators.
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
            if value.parse::<i32>().is_err() {
                if value == "TRUE" || value == "true" {
                    value = true.to_string();
                } else if value == "FALSE" || value == "false" {
                    value = false.to_string();
                // Get value from variables
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

/// Executes a command based on parsed input.
///
/// # Arguments
/// * `turtle` - A mutable reference to the `Turtle` object.
/// * `image` - A mutable reference to the image where the turtle draws.
/// * `variables` - A mutable hashmap of variables and their values.
/// * `line` - The line of code being executed.
/// * `line_number` - Reference to the current line number for error reporting.
///
/// # Returns
/// * `Result<(), String>` - Ok if the command is executed successfully, otherwise an error string.
///
/// # Errors
/// *
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
            turtle.set_pen_color(color_code)?;
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
            // Make var_name value | Make hello 5
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

            // Parse value strings
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

            // Overwrite old variable
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

// =========== Unit Tests to test turtle functionality ============
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use unsvg::Image;

    #[test]
    fn test_turtle_initialization() {
        let width = 800;
        let height = 600;
        let turtle = Turtle::new(width, height);
        assert_eq!(turtle.x, width as i32 / 2);
        assert_eq!(turtle.y, height as i32 / 2);
        assert_eq!(turtle.heading, 0);
        assert!(!turtle.pen_down);
        assert_eq!(turtle.pen_color_code, 7);
    }

    #[test]
    fn test_move_forward_pen_down() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        turtle.pen_down = true;
        turtle.move_forward(100, &mut image);

        assert_eq!(turtle.x, 100);
        assert_eq!(turtle.y, 0);
    }

    #[test]
    fn test_turn() {
        let mut turtle = Turtle::new(200, 200);
        turtle.turn(90);
        assert_eq!(turtle.heading, 90);

        turtle.turn(270);
        assert_eq!(turtle.heading, 0);
    }

    #[test]
    fn test_set_pen_color() {
        let mut turtle = Turtle::new(200, 200);
        assert!(turtle.set_pen_color(3).is_ok());
        assert!(turtle.set_pen_color(16).is_err());
    }

    #[test]
    fn test_make_variable() {
        let mut turtle = Turtle::new(200, 200);
        let mut variables = HashMap::new();

        turtle.make("x", "10", &mut variables);
        assert_eq!(variables.get("x"), Some(&"10".to_string()));

        turtle.make("y", "20", &mut variables);
        assert_eq!(variables.get("y"), Some(&"20".to_string()));
    }

    // =========== Test for commands ============
    #[test]
    fn test_execute_command_penup() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        assert!(execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "PENUP",
            &line_number
        )
        .is_ok());
        assert!(!turtle.pen_down);
    }

    #[test]
    fn test_execute_command_pendown() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        assert!(execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "PENDOWN",
            &line_number
        )
        .is_ok());
        assert!(turtle.pen_down);
    }

    #[test]
    fn test_execute_command_forward() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        assert!(execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "FORWARD \"50",
            &line_number
        )
        .is_ok());
        assert_eq!(turtle.x, 100);
        assert_eq!(turtle.y, 50);
    }

    #[test]
    fn test_execute_command_back() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        assert_eq!(turtle.y, 100);
        assert!(execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "BACK \"25",
            &line_number
        )
        .is_ok());
        assert_eq!(turtle.y, 125);
    }

    #[test]
    fn test_execute_command_left() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        assert_eq!(turtle.x, 100);
        assert!(execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "LEFT \"50",
            &line_number
        )
        .is_ok());
        assert_eq!(turtle.x, 50);
    }

    #[test]
    fn test_execute_command_right() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        assert_eq!(turtle.x, 100);
        assert!(execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "RIGHT \"50",
            &line_number
        )
        .is_ok());
        assert_eq!(turtle.x, 150)
    }

    #[test]
    fn test_execute_command_setpencolor() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        assert!(execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "SETPENCOLOR \"1",
            &line_number
        )
        .is_ok());
        assert_eq!(turtle.pen_color_code, 1);
    }

    #[test]
    fn test_execute_command_turn() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        assert!(execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "TURN \"90",
            &line_number
        )
        .is_ok());
        assert_eq!(turtle.heading, 90);
    }

    #[test]
    fn test_execute_command_setheading() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        assert!(execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "SETHEADING \"180",
            &line_number
        )
        .is_ok());
        assert_eq!(turtle.heading, 180);
    }

    #[test]
    fn test_execute_command_setx() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        assert!(execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "SETX \"100",
            &line_number
        )
        .is_ok());
        assert_eq!(turtle.x, 100);
    }

    #[test]
    fn test_execute_command_sety() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        assert!(execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "SETY \"100",
            &line_number
        )
        .is_ok());
        assert_eq!(turtle.y, 100);
    }

    #[test]
    fn test_execute_command_make() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        assert!(execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "MAKE \"x \"10",
            &line_number
        )
        .is_ok());
        assert_eq!(variables.get("x"), Some(&"10".to_string()));
    }

    #[test]
    fn test_execute_command_addassign() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        turtle.make("a", "5", &mut variables);

        assert!(execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "ADDASSIGN \"a \"50",
            &line_number
        )
        .is_ok());
        assert_eq!(variables.get("a"), Some(&"55".to_string()));
    }

    #[test]
    fn test_execute_command_invalid_command() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        let result = execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "FORWARD INVALIDCOMMAND",
            &line_number,
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Error: Error on line 0, Unknown command: INVALIDCOMMAND"
        );
    }

    #[test]
    fn test_execute_command_missing_command() {
        let mut turtle = Turtle::new(200, 200);
        let mut image = Image::new(200, 200);
        let mut variables = HashMap::new();
        let line_number = 0;

        let result = execute_command(
            &mut turtle,
            &mut image,
            &mut variables,
            "BACK",
            &line_number,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Error: Error on line 0: Empty line");
    }
}
