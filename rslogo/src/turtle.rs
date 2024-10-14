use unsvg::{Image, COLORS};

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

    pub fn turn_left(&mut self, _: i32) {
        self.heading += 270;
    }

    pub fn turn_right(&mut self, _: i32) {
        self.heading += 90;
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

pub fn execute_command(turtle: &mut Turtle, image: &mut Image, line: &str) -> Result<(), String> {
    let commands: Vec<&str> = line.split_whitespace().collect();
    if commands[0].starts_with("//") {
        return Ok(()); // Ignore the comment line and return early
    }
    // Not comment
    match commands[0] {
        "PENUP" => turtle.pen_up(),
        "PENDOWN" => turtle.pen_down(),
        "FORWARD" => {
            let distance_str = commands.get(1).ok_or("Missing input: distance")?; // .parse().map_err(|_| format!("Invalid distance: {}", commands.get(1)))?;
            let distance: i32 = distance_str.trim_start_matches('"').parse().map_err(|_| format!("Invalid distance: {}", distance_str))?;
            turtle.move_forward(distance, image);
        }
        "BACK" => {
            let distance_str = commands.get(1).ok_or("Missing input: distance")?;
            let distance: i32 = distance_str.trim_start_matches('"').parse().map_err(|_| "Invalid distance: {}, distance")?;
            turtle.move_back(distance, image);
        }
        "SETPENCOLOR" => {
            let color: usize = commands.get(1).ok_or("Missing input: color code")?.parse().map_err(|_| "Invalid color code")?;
            turtle.set_pen_color(color)?;
        }
        _ => return Err(format!("Unknown command: {}, please choose from: PENUP, PENDOWN, FORWARD, BACK, SETPENCOLOR", commands[0])),
    }
    Ok(())
}
