fn maior_numero(input: &[i32]) -> i32 {
    let mut maior: i32 = input[0];
    
    for i in input {
        if *i > maior {
            maior = *i;
        }
    }

    maior
}

fn main() {
    let numeros: [i32; 10] = [42, 2, 0, 92, 12, 13, 71, 53, 51, 123];

    println!("{}",maior_numero(&numeros));
}
