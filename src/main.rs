mod assembler;
mod macros;
use std::{fs::File, io::Read, process::exit};

use assembler::assemble;

fn main() {
    let mut f = File::open("./exemples/test.nzm").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    assemble(&content).unwrap_or_else(|e| {
        println!("{e:?}");
        exit(e as i32);
    });
}
