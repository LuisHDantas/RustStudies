use std::io;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    println!("Enter a string to reverse: ");
    let input = read_input();
    let reversed = reverse_string(&input);
    println!("Reversed string: {}", reversed);
}
