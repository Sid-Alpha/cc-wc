// use std::env;
use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Bytes
    #[arg(short='c')]
    bytes: bool,

    /// Number of lines in the file
    #[arg(short='l')]
    lines: bool,

    // Number of words in the file
    #[arg(short='w')]
    words: bool,

    // Number of chars in the file
    #[arg(short='m')]
    chars: bool,

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
    if args.lines || args.words || args.chars {
        let mut lines: i32 = 0;
        let mut words: i32 = 0;
        let file_str = fs::read_to_string(&file).expect("Should have been able to read the file");
        let chars = file_str.chars().count();
        for line in file_str.lines() {
            for _word in line.split_whitespace() {
                words += 1;
            }
            lines += 1;
        }

        if args.chars {
            let chars_op = format!(" {chars} chars");
            res.push_str(&*chars_op);
        }

        if args.words {
            let words_op = format!(" {words} words");
            res.push_str(&*words_op);
        }
        if args.lines {
            let lines_op = format!(" {lines} lines");
            res.push_str(&*lines_op);
        }
    }

    res.push_str(" ");
    res.push_str(file.as_str());
    println!(" {}", res);
}
