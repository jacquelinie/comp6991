use unsvg::{Image, COLORS};
use std::process;
use std::collections::HashMap;

pub struct Turtle {
    pub x: i32,
    pub y: i32,
    pub heading: i32,
    pub pen_down: bool,
    pub pen_color: unsvg::Color, // Color code as defined in unsvg COLORS array
}

impl Turtle {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            x: width as i32 / 2,
            y: height as i32 / 2,
            heading: 0,
            pen_down: false,
            pen_color: COLORS[7], // Default white
        }
    }

    pub fn pen_up(&mut self) {
        println!("Pen up");
        self.pen_down = false;
    }

    pub fn pen_down(&mut self) {
        println!("Pen down");
        self.pen_down = true;
    }

    pub fn move_forward(&mut self, distance: i32, image: &mut Image) {
        println!("Moving forward by {}", distance);
        let (new_x, new_y) = unsvg::get_end_coordinates(self.x, self.y, self.heading, distance);
        if self.pen_down {
            if let Err(e) = image.draw_simple_line(self.x, self.y, self.heading, distance, self.pen_color) {
                println!("Error occurred when drawing image: {}", e);
            };
        }
        self.x = new_x;
        self.y = new_y;
        println!("Current coord: {new_x}, {new_y}");
    }

    pub fn move_back(&mut self, distance: i32, image: &mut Image) {
        self.move_forward(-distance, image);
    }

    pub fn turn(&mut self, degrees: i32) {
        println!("Turning by: {degrees}");
        self.heading = (self.heading + degrees) % 360;
    }

    pub fn set_heading(&mut self, degrees: i32) {
        println!("Setting heading: {degrees}");
        self.heading = (degrees) % 360;
    }

    pub fn set_x(&mut self, position: i32) {
        println!("Setting x: {position}");
        self.x = position;
    }

    pub fn set_y(&mut self, position: i32) {
        println!("Setting y: {position}");
        self.y = position;
    }

    pub fn left(&mut self, distance: i32, image: &mut Image) {
        println!("Traveling left: {distance}");
        let curr_heading = self.heading;
        self.heading = (self.heading - 90) % 360;
        self.move_forward(distance, image);
        self.heading = curr_heading;
    }

    pub fn right(&mut self, distance: i32, image: &mut Image) {
        println!("Traveling right: {distance}");
        let curr_heading = self.heading;
        self.heading = (self.heading + 90) % 360;
        self.move_forward(distance, image);
        self.heading = curr_heading;
    }

    pub fn set_pen_color(&mut self, color_code: usize) -> Result<(), String> {
        println!("Setting pen color: {}", color_code);
        if color_code < 16 {
            self.pen_color = COLORS[color_code];
            Ok(())
        } else {
            Err(format!("Invalid color code: {}", color_code)) // TODO: Fix errors to return Err(())
        }
    }
}

pub fn error_extra_arguments(inputs: &Vec<&str>, num_inputs: usize) {
    if inputs.len() <= num_inputs {
        return;
    }
    let extra_args = &inputs[num_inputs..];
    let extra_args_debug: Vec<String> = extra_args.iter().map(|arg| format!("{:?}", arg)).collect();
    let extra_args_str = extra_args_debug.join(", "); // Format Arguments
    eprintln!("Error: Extra arguments: [{}]", extra_args_str);
    process::exit(1);
}

pub fn execute_command(turtle: &mut Turtle, image: &mut Image, variables: &mut HashMap<String, i32>, line: &str, line_number: &i32) -> Result<(), String> {
    let inputs: Vec<&str> = line.split_whitespace().collect();
    let mut arguments = Vec::new();
    // Parse Args
    for input in &inputs[1..] {
        // " variable
        if input.starts_with('"') {
            arguments.push(input.trim_start_matches('"').to_string());

        // : variable
        } else if input.starts_with(':') {
            let var_name = input.trim_start_matches(':');
            let arg = *variables.get(var_name).ok_or(format!("Error on line {}: Could not find variable: {}", line_number, var_name))?;
            arguments.push(arg.to_string());

        } else {
            return Err(format!("Error on line {}, Unknown command: {}", line_number, input));
        }
    }

    // Not comment
    match inputs[0] {
        // ============= TASK 1 =============
        "PENUP" => { // Just pen up
            error_extra_arguments(&inputs, 1);
            turtle.pen_up();
        }
        "PENDOWN" => { // Just pen down
            error_extra_arguments(&inputs, 1);
            turtle.pen_down();
        }
        "FORWARD" => { // Forward dist
            error_extra_arguments(&inputs, 2);
            let distance_str = arguments.get(0).ok_or(format!("Error on line {}: Empty line", line_number))?;
            let distance: i32 = distance_str.parse().map_err(|_| format!("Error on line {}: Drawing requires an integer argument", line_number))?;
            turtle.move_forward(distance, image);
        }
        "BACK" => { // Back dist
            error_extra_arguments(&inputs, 2);
            let distance_str = arguments.get(0).ok_or(format!("Error on line {}: Empty line", line_number))?;
            let distance: i32 = distance_str.parse().map_err(|_| format!("Error on line {}: Drawing requires an integer argument", line_number))?;
            turtle.move_back(distance, image);
        }
        "LEFT" => { // Left dist
            error_extra_arguments(&inputs, 2);
            let distance_str = arguments.get(0).ok_or(format!("Error on line {}: Empty line", line_number))?;
            let distance: i32 = distance_str.parse().map_err(|_| format!("Error on line {}: Drawing requires an integer argument", line_number))?;
            turtle.left(distance, image);
        }
        "RIGHT" => { // Right dist
            error_extra_arguments(&inputs, 2);
            let distance_str = arguments.get(0).ok_or(format!("Error on line {}: Empty line", line_number))?;
            let distance: i32 = distance_str.parse().map_err(|_| format!("Error on line {}: Drawing requires an integer argument", line_number))?;
            turtle.right(distance, image);
        }
        "SETPENCOLOR" => { // Setpencolor color
            error_extra_arguments(&inputs, 2);
            let color = arguments.get(0).ok_or(format!("Error on line {}: Empty line", line_number))?;
            let color_code = color.parse().map_err(|_| format!("Error on line {}: Invalid color: {}", line_number, color))?;
            turtle.set_pen_color(color_code)?; // TODO: if let Err =
        }
        "TURN" => { // Turn degrees
            error_extra_arguments(&inputs, 2);
            let degree_str = arguments.get(0).ok_or(format!("Error on line {}: Empty line", line_number))?;
            let degree: i32 = degree_str.parse().map_err(|_| format!("Error on line {}: Turning requires an integer.", line_number))?;
            turtle.turn(degree);
        }
        "SETHEADING" => { // Setheading degrees
            error_extra_arguments(&inputs, 2);
            let degree_str = arguments.get(0).ok_or(format!("Error on line {}: Empty line", line_number))?;
            let degree: i32 = degree_str.parse().map_err(|_| format!("Error on line {}: Setting heading requires an integer.", line_number))?;
            turtle.set_heading(degree);
        }
        "SETX" => { // Setx x
            error_extra_arguments(&inputs, 2);
            let position_str = arguments.get(0).ok_or(format!("Error on line {}: Empty line", line_number))?;
            let position: i32 = position_str.parse().map_err(|_| format!("Error on line {}: Setting position requires an integer.", line_number))?;
            turtle.set_x(position);
        }
        "SETY" => { // Sety y
            error_extra_arguments(&inputs, 2);
            let position_str = arguments.get(0).ok_or(format!("Error on line {}: Empty line", line_number))?;
            let position: i32 = position_str.parse().map_err(|_| format!("Error on line {}: Setting position requires an integer.", line_number))?;
            turtle.set_y(position);
        }
        // ================ TASK 2 ================
        "MAKE" => { // Make var_name value
            error_extra_arguments(&inputs, 3);
            let var_name_str = arguments.get(0).ok_or(format!("Error on line {}: Empty line", line_number))?;
            let var_val_str = arguments.get(1).ok_or(format!("Error on line {}: Empty line", line_number))?;
            let var_val: i32 = var_val_str.parse().map_err(|_| format!("Error on line {}: Making variable requires a value.", line_number))?;
            variables.insert(var_name_str.to_string(), var_val);
            println!("Made variable {}: {}", var_name_str, var_val);
        }
        "ADDASSIGN" => { // AddAssign v1 v2
            error_extra_arguments(&inputs, 3);
            // let var_name = inputs[1].trim_start_matches(':');
            // let existing_value = variables.get_mut(var_name).ok_or(format!("Error on line {}: Variable {} not found", line_number, var_name))?;

            // let add_value_str = inputs[2];
            // let add_value = if add_value_str.starts_with(':') {
            //     let add_var_name = add_value_str.trim_start_matches(':');
            //     variables.get(add_var_name).ok_or(format!("Error on line {}: Variable {} not found", line_number, add_var_name))?;
            // } else {
            //     return Err(format!("Error on line {}: ADDASSIGN requires a number", line_number))?;
            // };

            // *existing_value += add_value;
            // println!("Added {} to {}, new value: {}", add_value, var_name, existing_value);
        }

        _ => return Err(format!("Error on line {}: Unknown command: {}", line_number, inputs[0])),
    }
    Ok(())
}
