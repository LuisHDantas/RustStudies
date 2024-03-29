use std::io;
use std::collections::HashMap;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn is_anagram(s1: &str, s2: &str) -> bool {

    let mut hash: HashMap<char, i32> = HashMap::new();
    
    for c in s1.chars() {
        let count = hash.entry(c).or_insert(0);
        *count += 1;
    }

    for c in s2.chars() {
        let count = hash.entry(c).or_insert(0);
        *count -= 1;
    }

    //if every key in hash is 0, then it is an anagram
    hash.values().all(|&x| x == 0)
}
fn main() {
    let string1 = read_input();
    let string2 = read_input();

    println!("{}", is_anagram(&string1, &string2));
}
