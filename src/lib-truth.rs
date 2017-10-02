use std::io;

pub fn read_table() {
    println!("Enter Number of Variables: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let var_cnt: u64 = input.trim().parse().expect("Enter a number!");

    let digits = (((var_cnt - 1)as f64).log2() as usize) + 1;

    for n in 0..var_cnt {
        println!("|{}|?|", format!("{:0d$b}", n, d = digits));
    }
}
