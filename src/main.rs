use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let file = &args[1];

    println!("File path: {}", file);
    let vec = fs::read(file).expect("Should have been able to read the file");
    // let size = content.metadata().unwrap.len();
    println!("{}", vec.len());
    // println!("println/zHello, world!");

}
