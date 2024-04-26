use std::fs::File;
use std::io::Read;
fn main(){
    let f1 = File::open("perm.rs").unwrap();
    let f2 = File::open("hashstring.rs").unwrap();

    let mut chained_handle = f1.chain(f2);
    let mut buffer = String::new();

    chained_handle.read_to_string(&mut buffer).unwrap();

    println!("REad from chained handle:\n{}", buffer);
}
