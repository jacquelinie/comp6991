//! doctor_who
//! Crate that caesar shifts a string

// Default shift when no shift is given
const DEFAULT_SHIFT: i32 = 5;
// Value for 'A'
const UPPERCASE_A: i32 = 65;
// Value for 'a'
const LOWERCASE_A: i32 = 97;
// Value for size of the alphabet
const ALPHABET_SIZE: i32 = 26;

/// Shift string by shift number
/// # Arguments
/// * `shift_by` - Number shifted by each letter
/// * `lines` - Vector of Strings to be shifted
/// Doc Test:
/// ```
/// // use doctor_who::caesar_shift;
///
/// // let input = vec![String::from("Hello"), String::from("World!")];
/// // output = caesar_shift(None, input);
///
/// // assert_eq!(output, "Shifted ascii by 5 is: Mjqqt");
/// // assert_eq!(output, "Shifted ascii by 5 is: Btwqi!");
/// ```
pub fn caesar_shift(shift_by: Option<i32>, lines: Vec<String>) {
    let shift_number = shift_by.unwrap_or(DEFAULT_SHIFT);
    lines.into_iter().for_each(|line| {
        println!(
            "Shifted ascii by {shift_number} is: {}",
            shift(shift_number, line)
        );
    });
}


/// Shift string by a number
/// # Arguments
/// * `shift_by` - Number to shift the line by
/// * `line` - String that gets shifted
fn shift(shift_by: i32, line: String) -> String {
    let mut result: Vec<char> = Vec::new();

    // turn shift_by into a positive number between 0 and 25
    let shift_by = shift_by % ALPHABET_SIZE + ALPHABET_SIZE;

    line.chars().for_each(|c| {
        let ascii = c as i32;

        if ('A'..='Z').contains(&c) {
            result.push(to_ascii(
                abs_modulo((ascii - UPPERCASE_A) + shift_by, ALPHABET_SIZE) + UPPERCASE_A,
            ));
        } else if ('a'..='z').contains(&c) {
            result.push(to_ascii(
                abs_modulo((ascii - LOWERCASE_A) + shift_by, ALPHABET_SIZE) + LOWERCASE_A,
            ));
        } else {
            result.push(c)
        }
    });

    result.iter().collect()
}


/// Modulo operation
/// # Arguments
/// * `a` - Number which is the dividend
/// * `b` - Number which is the divisor
fn abs_modulo(a: i32, b: i32) -> i32 {
    (a % b).abs()
}


/// Change number to ascii character
/// # Arguments
/// * `i` - Number
fn to_ascii(i: i32) -> char {
    char::from_u32(i as u32).unwrap()
}