use std::io::{stdin, stdout, Write};

fn main() {
    let mut s = String::new();
    let mut state = false;
    loop {
        s.clear();
        println!("Current state is {}", state);
        print!(">>> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut s).unwrap();
        let v: Vec<&str> = s.trim()
            .split(" ")
            .filter(|x|{*x != ""})
            .collect();
        println!("v: {:?}",v);
        let s  = &v[..];
        match s {
            [] => println!("Empty line"),
            ["!", .. ] => println!("Comment"),
            ["inc", x] if x.parse::<i32>().is_ok() => println!("increment"),
            [x] => println!("Single element: {}",x),
            [x,y] => println!("two elements: {} {}",x,y),
            x => println!("Unknown command '{:?}'",x)
        }
    }

}