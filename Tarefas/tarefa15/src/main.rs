use std::io;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn is_palindrome(frase: &str) -> bool {
    let input = frase.to_lowercase().replace(|c: char| !c.is_alphanumeric(), "");

    for i in 0..input.len()/2 {
        if input.chars().nth(i) != input.chars().nth(input.len()-i-1) {
            return false;
        }
    }
    true
}

fn main() {
    println!("Enter the first string:");
    let str1 = read_input().trim().to_string();


    //check if the strings are palindromes
    if is_palindrome(&str1) {
        println!("The string is palindrome");
    } else {
        println!("The string is not palindrome");
    }
}
