use unsvg::{Image, COLORS};
use std::process;

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
            Err(format!("Invalid color code: {}", color_code))
        }
    }
}

pub fn error_extra_arguments(commands: &Vec<&str>, num_commands: usize) {
    let extra_args = &commands[num_commands..];
    let extra_args_debug: Vec<String> = extra_args.iter().map(|arg| format!("{:?}", arg)).collect();
    let extra_args_str = extra_args_debug.join(", "); // Format Arguments
    eprintln!("Error: Extra arguments: [{}]", extra_args_str);
    process::exit(1);
}

pub fn execute_command(turtle: &mut Turtle, image: &mut Image, line: &str, line_number: &i32) -> Result<(), String> {
    let commands: Vec<&str> = line.split_whitespace().collect();
    if commands[0].starts_with("//") {
        return Ok(()); // Ignore the comment line and return early
    }
    // Not comment
    match commands[0] {
        "PENUP" => {
            if commands.len() > 1 {
                error_extra_arguments(&commands, 1);
            }
            turtle.pen_up();
        }
        "PENDOWN" => {
            if commands.len() > 1 {
                error_extra_arguments(&commands, 1);
            }
            turtle.pen_down();
        }
        "FORWARD" => {
            if commands.len() > 2 {
                error_extra_arguments(&commands, 2);
            }
            let distance_str = commands.get(1).ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            if distance_str.starts_with('"') {
                let distance: i32 = distance_str.trim_start_matches('"').parse().map_err(|_| format!("Error: Error on line {}: Drawing requires an integer argument", line_number))?;
                turtle.move_forward(distance, image);
            } else {
                eprintln!("Error: Error on line {}, Unknown command: {}", line_number, distance_str);
                process::exit(1);
            }
        }
        "BACK" => {
            if commands.len() > 2 {
                error_extra_arguments(&commands, 2);
            }
            let distance_str = commands.get(1).ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            if distance_str.starts_with('"') {
                let distance: i32 = distance_str.trim_start_matches('"').parse().map_err(|_| format!("Error: Error on line {}: Drawing requires an integer argument", line_number))?;
                turtle.move_back(distance, image);
            } else {
                eprintln!("Error: Error on line {}, Unknown command: {}", line_number, distance_str);
                process::exit(1);
            }
        }
        "LEFT" => {
            if commands.len() > 2 {
                error_extra_arguments(&commands, 2);
            }
            let distance_str = commands.get(1).ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            if distance_str.starts_with('"') {
                let distance: i32 = distance_str.trim_start_matches('"').parse().map_err(|_| format!("Error: Error on line {}: Drawing requires an integer argument", line_number))?;
                turtle.left(distance, image);
            } else {
                eprintln!("Error: Error on line {}, Unknown command: {}", line_number, distance_str);
                process::exit(1);
            }
        }
        "RIGHT" => {
            if commands.len() > 2 {
                error_extra_arguments(&commands, 2);
            }
            let distance_str = commands.get(1).ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            if distance_str.starts_with('"') {
                let distance: i32 = distance_str.trim_start_matches('"').parse().map_err(|_| format!("Error: Error on line {}: Drawing requires an integer argument", line_number))?;
                turtle.right(distance, image);
            } else {
                eprintln!("Error: Error on line {}, Unknown command: {}", line_number, distance_str);
                process::exit(1);
            }
        }
        "SETPENCOLOR" => {
            if commands.len() > 2 {
                error_extra_arguments(&commands, 2);
            }
            let color = commands.get(1).ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            if color.starts_with('"') {
                let color_code = color.trim_start_matches('"').parse().map_err(|_| format!("Error: Error on line {}: Invalid color: {}", line_number, color))?;
                turtle.set_pen_color(color_code)?;
            } else {
                eprintln!("Error: Error on line {}, Unknown command: {}", line_number, color);
                process::exit(1);
            }
        }
        "TURN" => {
            if commands.len() > 2 {
                error_extra_arguments(&commands, 2);
            }
            let degree_str = commands.get(1).ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
                        if degree_str.starts_with('"') {
                let degree: i32 = degree_str.trim_start_matches('"').parse().map_err(|_| format!("Error: Error on line {}: Turning requires an integer.", line_number))?;
                turtle.turn(degree);
            } else {
                eprintln!("Error: Error on line {}, Unknown command: {}", line_number, degree_str);
                process::exit(1);
            }
        }
        "SETHEADING" => {
            if commands.len() > 2 {
                error_extra_arguments(&commands, 2);
            }
            let degree_str = commands.get(1).ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            if degree_str.starts_with('"') {
                let degree: i32 = degree_str.trim_start_matches('"').parse().map_err(|_| format!("Error: Error on line {}: Setting heading requires an integer.", line_number))?;
                turtle.set_heading(degree);
            } else {
                eprintln!("Error: Error on line {}, Unknown command: {}", line_number, degree_str);
                process::exit(1);
            }
        }
        "SETX" => {
            if commands.len() > 2 {
                error_extra_arguments(&commands, 2);
            }
            let position_str = commands.get(1).ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            if position_str.starts_with('"') {
                let position: i32 = position_str.trim_start_matches('"').parse().map_err(|_| format!("Error: Error on line {}: Setting position requires an integer.", line_number))?;
                turtle.set_x(position);
            } else {
                eprintln!("Error: Error on line {}, Unknown command: {}", line_number, position_str);
                process::exit(1);
            }
        }
        "SETY" => {
            if commands.len() > 2 {
                error_extra_arguments(&commands, 2);
            }
            let position_str = commands.get(1).ok_or(format!("Error: Error on line {}: Empty line", line_number))?;
            if position_str.starts_with('"') {
                let position: i32 = position_str.trim_start_matches('"').parse().map_err(|_| format!("Error: Error on line {}: Setting position requires an integer.", line_number))?;
                turtle.set_y(position);
            } else {
                eprintln!("Error: Error on line {}, Unknown command: {}", line_number, position_str);
                process::exit(1);
            }
        }
        _ => return Err(format!("Error on line {}: Unknown command: {}", line_number, commands[0])),
    }
    Ok(())
}
