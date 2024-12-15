mod assembler;
mod macros;
use std::{env::args, fs::File, io::Read, process::exit};

use assembler::assemble;

fn main() {
    let mut f = File::open(args().nth(1).unwrap_or(String::from("./exemples/test.nzm"))).unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    assemble(
        File::options()
            .write(true)
            .create(true)
            .open(args().nth(2).unwrap_or(String::from("a.out")))
            .expect("Failed to create a.out file"),
        &content,
    )
    .unwrap_or_else(|e| {
        println!("{e:?}");
        exit(From::from(&e));
    });
}
