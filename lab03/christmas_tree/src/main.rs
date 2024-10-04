use serde::Deserialize;
use std::collections::VecDeque;
use std::io;

#[derive(Debug, Deserialize)]
enum Instruction {
    Set(i32),
    Left,
    Right,
    Reset,
}

#[derive(Debug, Default)]
struct Light {
    // TODO: change me!
    left: Option<Box<Light>>,
    right: Option<Box<Light>>,
    brightness: i32,
}

fn get_instructions_from_stdin() -> VecDeque<Instruction> {
    let mut instructions = String::new();
    io::stdin().read_line(&mut instructions).unwrap();
    ron::from_str(&instructions).unwrap()
}

fn get_tree_sum(light: &Light) -> i32 {
    let left = if let Some(left) = &light.left {
        get_tree_sum(left)
    } else {
        0
    };
    let right = if let Some(right) = &light.right {
        get_tree_sum(right)
    } else {
        0
    };
    left + right + light.brightness
}

fn get_tree_count(light: &Light) -> i32 {
    let left = if let Some(left) = &light.left {
        get_tree_count(left)
    } else {
        0
    };
    let right = if let Some(right) = &light.right {
        get_tree_count(right)
    } else {
        0
    };
    left + right + 1
}

fn main() {
    let mut instructions = get_instructions_from_stdin();
    let mut light = Light { left: None, right: None, brightness: 0};
    // println!("{instructions:?}");
    // println!("{light:?}");
    // TODO: your implementation here
    let mut curr = &mut light;
    loop {
        match instructions.pop_front() {
            Some(Instruction::Set(x)) => curr.brightness = x,
            Some(Instruction::Left) => curr = curr.left.get_or_insert_with(Default::default),
            Some(Instruction::Right) => {
                curr = curr.right.get_or_insert_with(Default::default)
            }
            Some(Instruction::Reset) => curr = &mut light,
            None => break,
        }
    }

    let sum = get_tree_sum(&light);
    let count = get_tree_count(&light);
    let avg = sum / count;
    println!("{avg}");
}
