use std::io::{BufRead, BufReader};
fn main() {
    let s = std::io::stdin();
    let file_reader = BufReader::new(s);
    for single_line in file_reader.lines() {
        println!("You types:{}", single_line.unwrap());
    }
}
