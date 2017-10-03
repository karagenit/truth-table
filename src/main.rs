extern crate libtruth;

use std::io;

use libtruth::read_table;
use libtruth::get_minterms;

fn main() {

    println!("Enter Number of Inputs: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let inputs: u32 = input.trim().parse::<u32>().expect("Enter a number!");

    //TODO check 26 >= inputs > 0

    println!("\nTruth Table");
    let outputs = read_table(inputs);

    let minterms = get_minterms(inputs, outputs);
    println!("\nMinterm Expression: {}", minterms);
}
