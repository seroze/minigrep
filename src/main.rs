extern crate minigrep;

//use core::panic;
use std::env;
//use std::fs;
use std::process; 

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
           process::exit(1);
        });
  
//    minigrep::run(config);
    if let Err(e) = minigrep::run(config) {
        
        eprintln!("Application error: {}",e);
    
        process::exit(1);
    } 

}
