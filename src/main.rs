use std::fs;
use std::io;
use std::io::{Read};
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

    file: Option<String>,
}

fn wc_helper(file_str: String, args: Args) -> String {
    let mut res: String = String::new();
    if args.bytes {
        let size = file_str.len();
        res.push_str(size.to_string().as_str());
        res.push_str(" bytes");
    }
    if args.lines || args.words || args.chars {
        let mut lines: i32 = 0;
        let mut words: i32 = 0;

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

    return res;
}

fn main() {
    let args: Args = Args::parse();
    let file = &args.file;
    let mut file_str: String = "".to_string();
    if file.is_none() {
        file_str = String::new();
        io::stdin().lock().read_to_string(&mut file_str).expect("Failed to read stdIN");
    } else if let Some(file) = file {
        let file: &String = file;
        file_str = fs::read_to_string(&file).expect("Should have been able to read the file");
    }

    println!(" {}", wc_helper(file_str, args));
}
