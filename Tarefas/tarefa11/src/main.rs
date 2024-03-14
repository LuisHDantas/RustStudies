use std::io;

fn compress_string(input: &str) -> String {
    if input.len() <= 1 {
        return String::from(input);
    }
    
    let mut compressed = String::new();
    let mut counter = 0;
    let mut last_char: char = input.chars().nth(0).unwrap();

    for c in input.chars() {
        if c == last_char {    
            counter += 1;
        }
        else{
            compressed.push(last_char);
            compressed.push_str(&counter.to_string());
            counter = 1;
            last_char = c;
        }
    }

    compressed.push(last_char);
    compressed.push_str(&counter.to_string());

    compressed
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let output = compress_string(input.trim());
    println!("{} -> {}", input, output);    
}
