use luhn::is_valid;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Luhn program")
        .version("0.1.0")
        .author("luapicella")
        .about("Check credit number")
        .arg(Arg::new("credit number")
                .short('c')
                .long("number")
                .takes_value(true)
                .help("credit card number to check"))
        .get_matches();
    
    let my_string = matches.value_of("credit number");
    let mut count = 0;

    match my_string {
        None => return,
        Some(ss) => {
            let v: Vec<&str> = ss.trim().split(" ").filter(|x|{*x != ""}).collect();
            for tmp in v.into_iter(){
                match tmp {
                    ss if ss.chars().all(char::is_numeric) && ss.chars().count() == 4 => count+=4,
                    _ => break 
                }
            }
            match count {
                16 => if is_valid(ss){
                    println!("Valid")
                }else{
                    println!("Invalid")
                }

                _ => println!("Not credit number")
                
            }
        }
    }    
}