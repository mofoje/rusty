use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let option1: f32 = input.trim().parse().unwrap();
    let first_result = 100.0 / option1;
    let second_result = 100.0 / (100.0 - option1);
    println!("{first_result}");
    println!("{second_result}");
}


