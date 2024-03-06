fn eh_primo(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    
    let limite: i32 = (num as f64).sqrt().ceil() as i32;

    for i in 2..=limite {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!("{}",eh_primo(13));
}
