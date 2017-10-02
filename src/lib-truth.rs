use std::io;
use std::io::prelude::*;

pub fn read_table() {
    println!("Enter Number of Inputs: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let inputs: u32 = input.trim().parse().expect("Enter a number!");

    let combos = 2u32.pow(inputs);

    for n in 0..combos {
        print!("|{}|", format!("{:0d$b}", n, d = (inputs as usize)));
        io::stdout().flush()
            .ok().expect("Cannot flush");
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    }
}
