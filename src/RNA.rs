use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("rosalind_dna.txt").expect("File not found!");
    let mut contents = String::new();
    let t;

    f.read_to_string(&mut contents)
        .expect("Something wrong while reading the file.");

    t = contents.replace("T", "U");
    println!("{}\n", t);
}
