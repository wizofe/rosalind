use std::fs::File;
use std::iter::Iterator;
use std::io::prelude::*;
use std::borrow::Borrow;
use std::collections::HashMap;;

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let mut f = File::open("rosalind_dna.txt").expect("File not found!");
    let mut contents = String::new();

    let mut nucl = Hashmap::new();

    nucl.insert("A", "T");
    nucl.insert("G", "C");

    f.read_to_string(&mut contents)
        .expect("Something wrong while reading the file.");

    
    println!("{}", reverse_string(contents.borrow()));
}
