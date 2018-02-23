use std::error::Error;
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> String {
    let mut file = match File::open(path) {
        Err(err) => panic!("Couldn't open: {}", err.description()),
        Ok(file) => file,
    };

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(err) => panic!("Couldn't read: {}", err.description()),
        Ok(_) => (),
    };
    data
}

fn main() {
    let hello = read_file("hello.txt");
    let world = read_file("world.txt");

    println!("Content is: {}", hello);
    println!("Content is: {}", world);
}
