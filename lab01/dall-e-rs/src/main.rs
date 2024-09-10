use bmp::Image;
use bmp::Pixel;

const IMAGE_SIZE_PX: u32 = 200;

fn main() {
	let mut image = Image::new(IMAGE_SIZE_PX, IMAGE_SIZE_PX);

	let purple = create_pixel(213, 171, 255);
	let pink = create_pixel(255, 216, 241);
	let blue = create_pixel(175, 234, 255);
	let white = create_pixel(255, 255, 255);

	for x in 0..IMAGE_SIZE_PX {
		for y in 0..IMAGE_SIZE_PX {
			if (x + y) % 15 == 0 {
				image.set_pixel(x, y, purple);
			} else if (x + y) % 15 == 5 {
				image.set_pixel(x, y, pink);
			} else if (x + y) % 15 == 10 {
				image.set_pixel(x, y, blue);
			} else {
				image.set_pixel(x, y, white);
			}
		}
	}
    println!("Finished Image!");
	image.save("my_image.bmp")
		.expect("Failed to save image!");
}

fn create_pixel(r: u8, g: u8, b: u8) -> Pixel {
	return Pixel::new(r, g, b);
}