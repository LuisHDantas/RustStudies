fn count_down(num: i32) {
    for i in (1..=num).rev() {
        println!("{}", i);
    }
}

fn count(num: i32) {
    for i in 1..=num {
        println!("{}", i);
    }
}

fn main() {
    count(10);
    count_down(10);
}
