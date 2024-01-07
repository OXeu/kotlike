use std::{fs::File, io::{Read, Write}};

use kotlike::kotlike;

#[kotlike]
fn main() {
    let a = "Hello".to_string();
    let c: Option<()> = File::create("test.txt")?.write_all(a.as_bytes())?.clone();
    let mut b: String = String::new();
    let len: Option<usize> = File::open("test.txt")?.read_to_string(&mut b)?.clone();
    println!("Hello, {:?}({:?})!", b, len);
}