extern crate libtruth;

use libtruth::read_table;

fn main() {
    let outputs = read_table();
    println!("{:?}", outputs);
}
