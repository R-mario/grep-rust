use std::{env,process}; // needed for reading cmd

pub use rgrep::*;

fn main() {
    // recoge args y los transforma en vector
        // args: lo que sigue a la ejecucion de cargo run
    let args: Vec<String> = env::args().collect(); 
    let argumentos = Argumentos::build(&args)
        .unwrap_or_else(|err| {
            eprintln!("Error with the input parameters\n{}",err);
            process::exit(1);
        });

    println!("Reading file `{}`",argumentos.haystack);
    println!("Sarching for `{}`...",argumentos.needle);

    if let Err(e) = run(&argumentos) {
        eprintln!("Error reading file: {e}");
        process::exit(1);
    }
}