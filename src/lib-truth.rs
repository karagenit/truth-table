use std::io;
use std::io::prelude::*;

pub fn read_table() -> Box<[bool]> {
    println!("Enter Number of Inputs: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let inputs: u32 = input.trim().parse::<u32>().expect("Enter a number!");

    let combos: usize = 2u32.pow(inputs) as usize;

    let mut outputs = vec![false; combos].into_boxed_slice();

    for n in 0..combos {
        print!("|{}|", format!("{:0d$b}", n, d = (inputs as usize)));
        io::stdout().flush()
            .ok().expect("Cannot flush");

        input = String::new(); //apparently read_line() appends to the string
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        outputs[n] = input.trim().parse::<u32>().expect("Enter 1 or 0") > 0;
    }

    return outputs;
}
