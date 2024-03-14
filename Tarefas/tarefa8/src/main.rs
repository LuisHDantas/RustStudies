use std::io;

fn verifica_unicidade(input: &str) -> bool {
    let mut mapa_caracteres = [false; 128];

    for c in input.chars() {
        let index = c as usize;
        
        if mapa_caracteres[index] {
            return false;
        }else{
            mapa_caracteres[index] = true;   
        }
    }

    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler a linha");

    if verifica_unicidade(input.trim()) {
        println!("Todos os caracteres são únicos");
    } else {
        println!("Existem caracteres repetidos");
    }
}
