use std::io;

pub fn read_table() {
    println!("Enter Number of Variables: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    println!("{}", input);
}
