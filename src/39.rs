use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let contents = io::BufReader::new(file)?;

    for line in contents.lines() {
        println!("{}", line);
    }

    Ok(())
}
