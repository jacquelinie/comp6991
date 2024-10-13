use std::num::ParseIntError;

pub mod utils;
pub mod constants;

#[derive(Debug)]
pub struct TribonacciError(pub String);

/// Computes the tribonacci sequence of a given size
/// Prints the sequence, and its sum
pub fn compute_tribonacci(size: Result<usize, ParseIntError>) -> Result<(), TribonacciError> {
    let mut tribonacci = vec![1_u128; 3];
    let size = size.map_err(|_| TribonacciError(constants::ERROR_MESSAGE.to_string()))?;

    for i in 3..size {
        tribonacci.push(tribonacci[i - 1] + tribonacci[i - 2] + tribonacci[i - 3]);
    }

    println!("Values: {:?}", tribonacci);

    let value: u128 = tribonacci.into_iter().sum();
    println!("\nSum: {}", value);

    Ok(())
}
