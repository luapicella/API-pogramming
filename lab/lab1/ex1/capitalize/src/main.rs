mod capitalize;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Capitalize program")
        .version("0.1.0")
        .author("luapicella")
        .about("Capitalize first letter of each word")
        .arg(Arg::new("string")
                .short('s')
                .long("string")
                .takes_value(true)
                .help("string to capitalize"))
        .get_matches();
    
    let my_string = matches.value_of("string");
    match my_string {
        None => return,
        Some(ss) => println!("{}",capitalize::capitalize(ss))
    }    
}
