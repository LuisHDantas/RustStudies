use std::io;

fn um_passo(string1: &str, string2: &str) -> bool {
    
    if (string1.len() as isize - string2.len() as isize).abs() > 1 {
        return false;
    }

    let mut diferencas = 0;

    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < string1.len() && j < string2.len() && diferencas <= 1 {
        if string1.chars().nth(i) != string2.chars().nth(j) {
            diferencas += 1;

            if string1.len() > string2.len() {
                i += 1;
            } else if string1.len() < string2.len() {
                j += 1;
            } else {
                i += 1;
                j += 1;
            }
        } else {    
            i += 1;
            j += 1;
        }
    }

    if i < string1.len() || j < string2.len() {
        diferencas += 1;
    }

    return if diferencas > 1 { false } else { true };
}

fn main() {
    let mut input = String::new();

    println!("Digite a primeira string:");
    io::stdin().read_line(&mut input).expect("Erro ao ler a string");
    let string1 = input.trim().to_string();

    input.clear();

    println!("Digite a segunda string:");
    io::stdin().read_line(&mut input).expect("Erro ao ler a string");
    let string2 = input.trim().to_string();

    if um_passo(&string1, &string2) {
        println!("As strings são uma edição (ou zero edições) de distância uma da outra.");
    } else {
        println!("As strings não são uma edição de distância uma da outra.");
    }
}
