fn main() {
    let pattern_string = std::env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    let pattern = &pattern_string;

    // TODO: Replace the following with your code:
    loop {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        if input.is_empty() {
            return;
        }
        if input.contains(pattern) {
            print!("{input}");
        }
    }

}
