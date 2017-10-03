use std::io;
use std::io::prelude::*;

pub fn read_table(inputs: u32) -> Box<[bool]> {

    let combos: usize = 2u32.pow(inputs) as usize;

    let mut outputs = vec![false; combos].into_boxed_slice();

    for n in 0..combos {
        print!("|{}|", format!("{:0d$b}", n, d = (inputs as usize)));
        io::stdout().flush()
            .ok().expect("Cannot flush");

        let mut input = String::new(); //apparently read_line() appends to the string
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        outputs[n] = input.trim().parse::<u32>().expect("Enter 1 or 0") > 0;
    }

    return outputs;
}

pub fn get_minterms(inputs: u32, outputs: Box<[bool]>) -> String {
    let mut str = String::new();
    for i in 0..outputs.len() {
        if outputs[i] {
            if str.len() > 0 {
                str.push_str("+");
            }
            for l in 0..inputs {
                let b = inputs - 1 - l;
                let bit = (i >> b) & 0b1;
                let c = 65u8 + (l as u8);
                str.push_str(&((c as char).to_string()));
                if bit == 0 {
                    str.push_str("\u{0304}");
                }
            }
        }
    }
    return str;
}

/* Row example:
 * 101 | 1
 *
 * for each row in the table
 * if this row's output is true, we're gonna add the minterm
 * start with the highest bit (e.g. bit #[input-1]) and go to the lowest (bit #0)
 * if this bit is zero, append '!'; always append corresponding letter (A, B, ...)
 */
