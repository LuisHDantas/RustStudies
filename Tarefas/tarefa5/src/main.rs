use std::io;    

fn main() {
    println!("Digite um número para ver a tabuada: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Falha ao ler entrada.");
    let number: i32 = input.trim().parse().expect("Digite um inteiro.");

    for i in 1..=10 {
        println!("{} x {} = {}", number, i, number * i);
    }
}
