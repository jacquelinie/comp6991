use unsvg::Image;

pub struct Turtle {
    pub x: f32,
    pub y: f32,
    pub heading: f32,
    pub pen_down: bool,
    pub pen_color: usize, // Color code as defined in unsvg COLORS array
}

impl Turtle {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            x: width as f32 / 2.0,
            y: height as f32 / 2.0,
            heading: 0.0,
            pen_down: false,
            pen_color: 0,
        }
    }

    pub fn pen_up(&mut self) {
        self.pen_down = false;
    }

    pub fn pen_down(&mut self) {
        self.pen_down = true;
    }

    pub fn move_forward(&mut self, distance: f32, image: &mut Image) {
        let (new_x, new_y) = unsvg::get_end_coordinates(self.x, self.y, self.heading, distance);
        if self.pen_down {
            image.draw_simple_line(self.x, self.y, new_x, new_y, self.pen_color);
        }
        self.x = new_x;
        self.y = new_y;
    }

    pub fn move_back(&mut self, distance: f32, image: &mut Image) {
        self.move_forward(-distance, image);
    }

    pub fn turn_left(&mut self, degrees: f32) {
        self.heading += 270.0;
    }

    pub fn turn_right(&mut self, degrees: f32) {
        self.heading += 90.0;
    }

    pub fn set_pen_color(&mut self, color_code: usize) -> Result<(), String> {
        if color_code < 16 {
            self.pen_color = color_code;
            Ok(())
        } else {
            Err(format!("Invalid color code: {}", color_code))
        }
    }
}

pub fn execute_command(turtle: &mut Turtle, image: &mut Image, line: &str) -> Result<(), String> {
    let commands: Vec<&str> = line.split_whitespace().collect();
    match commands[0] {
        "PENUP" => turtle.pen_up(),
        "PENDOWN" => turtle.pen_down(),
        "FORWARD" => {
            let distance: f32 = commands.get(1).ok_or("Missing input: distance")?.parse().map_err(|_| "Invalid distance")?;
            turtle.move_forward(distance, image);
        }
        "BACK" => {
            let distance: f32 = commands.get(1).ok_or("Missing input: distance")?.parse().map_err(|_| "Invalid distance")?;
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
