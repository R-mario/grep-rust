use std::{env,process}; // needed for reading cmd

use rgrep::*;

fn main() {
    // recoge args y los transforma en vector
        // args: lo que sigue a la ejecucion de cargo run
    let args: Vec<String> = env::args().collect(); 
    let argumentos = Argumentos::build(&args)
        .unwrap_or_else(|err| {
            println!("error procesando los argumentos\n{}",err);
            process::exit(1);
        });
    if let Err(e) = read_file(&argumentos) {
        println!("Error leyendo archivo: {e}");
        process::exit(1);
    }
}