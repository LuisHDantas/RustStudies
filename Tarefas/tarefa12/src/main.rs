
use std::io;

fn is_palindrome(num: isize) -> bool {
    if num < 0 || (num != 0 && num % 10 == 0) {
        return false;
    }

    let mut input = num;
    let mut reversed = 0;

    while input > 0 {
        let digit = input % 10;

        reversed = reversed * 10 + digit;

        input /= 10;
    }

    num == reversed

}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: isize = input.trim().parse().expect("Please type a number");

    println!("{}", is_palindrome(num));
}
