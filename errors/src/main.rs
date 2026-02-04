use std::fs::File;

fn main() {
    let f = read_username_from_file().unwrap();

    println!("{f}")
}

use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
