use clap::{Parser};
use std::{
    fs,
    io::{self, prelude::*, BufReader},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Catr {
    files: Option<Vec<String>>,
}

fn main() {
    let io = Catr::parse();

    match io.files {
        Some(files) => {
            for f in files {
                let file = fs::File::open(f).unwrap();
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    println!("{}", line.unwrap());
                }
            }
        }
        None => loop {
            let mut sin = String::new();
            io::stdin()
                .read_line(&mut sin)
                .expect("Failed to read stdin");
            print!("{}", sin);
        },
    }
}
