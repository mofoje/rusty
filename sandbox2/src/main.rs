use std::io;

fn main() {
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");
    let rooms : i32 = input1.trim().parse().unwrap();
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");
    let teams : i32 = input2.trim().parse().unwrap();

    let mut remainder = teams % rooms;
    let mut i = 0;
    while i < rooms {
        let star_count = teams / rooms;
        let mut stars = String::new();
        for _ in 0..star_count {
            stars.push('*');
        }
        if remainder != 0 {
            stars.push('*');
            remainder = remainder - 1;
        }
        println!("{stars}");
        i = i + 1;
    }
}
