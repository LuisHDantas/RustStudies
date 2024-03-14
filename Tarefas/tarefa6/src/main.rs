use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");    
    let parts: Vec<f64> = input
                            .trim()
                            .split_whitespace()
                            .map(|x| x.parse::<f64>().expect("Failed to parse float"))
                            .collect();

    let mut sum: f64 = 0.0;

    for num in &parts {
        if num % 2.0 == 0.0 {
            sum += num;
        }
    }

    println!("Sum of even numbers: {}", sum)

}
