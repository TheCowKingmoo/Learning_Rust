
use std::fs::File;
use std::io::{BufRead, BufReader, Result};



fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn main() -> Result<()> {
    let file = File::open("E:\\crafttweaker.log")?;
    for line in BufReader::new(file).lines() {
        let (first_chr, remainder) = car_cdr("test");
        if first_chr != "["  {
            println!("{}", line?);
        }
    }
    Ok(())
}
