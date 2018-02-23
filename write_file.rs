use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("Couldn't open file.");
    let mut data = String::new();

    file.read_to_string(&mut data).expect("Couldn't read file");
    data
}

fn main() {
    let hello = read_file("hello.txt");
    let world = read_file("world.txt");

    println!("Content is: {}", hello);
    println!("Content is: {}", world);
}
