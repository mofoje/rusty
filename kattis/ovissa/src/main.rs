use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");
    let parsed_input1: u32 = input1.trim().parse().unwrap();
    let parsed_input2: u32 = input2.trim().parse().unwrap();


    println!("{}", parsed_input1 + parsed_input2);
}
