use std::fs::File;
use std::io::{ Read, Error };

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = try!(File::open(path));
    let mut data = String::new();

    try!(file.read_to_string(&mut data));
    Ok(data)
}

fn main() {
    let hello = read_file("hello.txt").expect("Couldn't read 'hello.txt'.");
    let world = read_file("world.txt").expect("Couldn't read 'world.txt'.");

    let hello_world = hello + " " + &world + "!";

    println!("Content is: {}", hello_world);
}
