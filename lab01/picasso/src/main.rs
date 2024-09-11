use bmp::{consts, open};

fn main() {
    // Get args, skipping filename then collect into a Vec
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    // Check if bmp ok
    for arg in args {
        let result = open(arg.clone());
        println!("===== {arg} =====");
        match result {
            // Print image if image is ok
            Ok(image) => {
                for (x, y) in image.coordinates() {
                    let pixel = image.get_pixel(x, y);

                    // R = Red, G = Lime, B = Blue, W = White
                    match pixel {
                        consts::RED => print!("R "),
                        consts::LIME => print!("G "),
                        consts::BLUE => print!("B "),
                        consts::WHITE => print!("W "),
                        e => panic!("this pixel hits different: {:?}", e),
                    }

                    // Print new line for end of image
                    if x == image.get_width() - 1 {
                        println!();
                    }
                }
            }
            // Print error
            Err(e) => {
                println!("Error! {:?}", e);
            }
        }
    }
}