use std::io;

fn  is_permutation(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len() {
        return false;
    }

    let mut contador = [0; 128];

    for c in str1.chars() {
        contador[c as usize] += 1;
    }

    for c in str2.chars() {
        contador[c as usize] -= 1;
    }

    for value in contador {
        if value != 0 {
            return false;
        }
    }

    true

}

fn main() {
    let mut input = String::new();
    
    println!("Insira string 1:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let string1 = input.trim().to_string();
    
    input.clear();
    
    println!("Insira string 2:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let string2 = input.trim().to_string();

    if is_permutation(&string1, &string2) {
        println!("As strings são permutações uma da outra");
    } else {
        println!("As strings não são permutações uma da outra");
    }
}
