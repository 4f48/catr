use clap::Parser;
use std::{fs, io};

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
                let out = match fs::read_to_string(&f) {
                    Ok(str) => str,
                    Err(error) => panic!("{}", error),
                };

                print!("{}", out);
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
