fn main() {
    loop {
        let mut line = String::new();
        let input = std::io::stdin().read_line(&mut line);
        print!("{line}");
    }
}
