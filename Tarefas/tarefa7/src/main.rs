use std::io;

fn calcula_media(notas: &Vec<f64>) -> f64 {
    let soma: f64 = notas.iter().sum();
    let qtd_notas = notas.len();

    soma / qtd_notas as f64
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler a linha");

    let notas: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().expect("Erro ao converter para f64"))
        .collect();

    let media = calcula_media(&notas);

    println!("A média é: {}", media)
}
