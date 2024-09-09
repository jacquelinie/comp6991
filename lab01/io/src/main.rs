fn main() {
    println!("What is your name? ");

    // allocate some absurd amount of memory
    let mut name = String::new();
    let name = std::io::stdin().read_line(&mut name);

    // trim the newline
    let name = name.trim();

    // if the name is empty, print a message and exit
    if !name.is_empty() {
        println!("Hello, {name}, nice to meet you!");
    } else {
        println!("No name entered :(, goodbye.");
    }
}
