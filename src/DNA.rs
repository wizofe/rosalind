use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("rosalind_dna.txt").expect("File not found!");
    let mut contents = String::new();

    let mut ca:i32 = 0;
    let mut ct:i32 = 0;
    let mut cc:i32 = 0;
    let mut cg:i32 = 0;

    f.read_to_string(&mut contents)
        .expect("Something wrong while reading the file.");

    println!("Here's your DNA: {}\n", contents);

    for c in contents.chars() {
        println!("{}", c);
        match c {
            'A' => ca += 1,
            'C' => cc += 1,
            'T' => ct += 1,
            'G' => cg += 1,
            _ => println!("SOS\n"),
        }
    }

    println!("{} {} {} {} ", ca, cc, cg, ct);
}
