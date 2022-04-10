use clap::Parser;
use std::io::{stdin, stdout, Write};

use binary_search::find;


#[derive(Parser)]
struct Cli {    
    key: i32,
}

fn main(){
    let args = Cli::parse();

    let mut s = String::new();
    let mut array: Vec<i32> = Vec::new();

    loop{
        s.clear();
        stdout().flush().unwrap();
        stdin().read_line(&mut s).unwrap();
        let v: Vec<&str> = s.trim()
            .split(" ")
            .filter(|x|{*x != ""})
            .collect();
        let s  = &v[..];
        match s {
           [] => break,
           [val] if val.parse::<i32>().is_ok() => array.push(val.parse::<i32>().unwrap()),
           _ => println!("Unknown ch")
        }
    }

    match find(&array[..], args.key) {
        Some(idx) => println!("Id: {}", idx),
        None => println!("{}", -1)
    }

    
}