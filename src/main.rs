// use std::env;
use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Bytes
    #[arg(short='c')]
    bytes: bool,

    /// Number of times to greet
    #[arg(short='l')]
    lines: bool,

    file: String,
}

fn main() {
    let args: Args = Args::parse();
    let file = args.file;
    let mut res: String = "".to_owned();
    if args.bytes {
        let vec = fs::read(&file).expect("Should have been able to read the file");
        let size = vec.len(); //metadata().unwrap.len();
        res.push_str(size.to_string().as_str());
        res.push_str(" bytes");
    }
    if args.lines {
        let mut lines = 0;
        let file_str = fs::read_to_string(&file).expect("Should have been able to read the file");
        for _line in file_str.lines() {
            lines += 1;
        }
        let lines_op = format!(" {lines} lines");
        res.push_str(&*lines_op);
    }

    res.push_str(" ");
    res.push_str(file.as_str());
    println!(" {}", res);
}
