use std::io;

fn main() {
    let mut result = Vec::new();
    while 1 > 0 {
        let mut input1 = String::new();
        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read line");
        let n: u32 = input1.trim().parse().unwrap();
        if n == 0 {
            result.pop();
            print_result(result.clone());
            return;
        }

        let mut i = 0;
        let mut names = Vec::with_capacity(n as usize);
        while i < n {
            let mut input2 = String::new();
            io::stdin()
                .read_line(&mut input2)
                .expect("Failed to read line");
            names.push(input2.trim().to_string());
            i = i + 1;
        }
        names.sort_by(| a, b| a[0..2].cmp(&b[0..2]));
        for name in names {
            result.push(name.clone().to_string());
        }
        result.push(String::new());
    }

}

fn print_result(result_set: Vec<String>) {
    for r in result_set {
        println!("{r}");
    }
}
