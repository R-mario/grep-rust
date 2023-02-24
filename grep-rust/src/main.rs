use std::{fs,env,process}; // needed for reading cmd

fn main() {
    // recoge args y los transforma en vector
        // args: lo que sigue a la ejecucion de cargo run
    let args: Vec<String> = env::args().collect(); 
    let argumentos = Argumentos::build(&args)
        .unwrap_or_else(|err| {
            println!("error procesando los argumentos\n{}",err);
            process::exit(1);
        });
    println!("{:#?}",argumentos);
    if let Err(e) = read_file(&argumentos) {
        println!("Error leyendo archivo: {e}");
        process::exit(1);
    }
}

#[derive(Debug)]
struct Argumentos {
    query: String,
    ruta: String,
}
impl Argumentos {
    fn build(args: &Vec<String>) -> Result<Argumentos, &'static str> {
        if args.len() > 3 {
            return Err("rgrep solo acepta 2 argumentos\npara más información ver rgrep --help");
        } else if args.len() <= 2 {
            return Err("debes especificar un archivo\npara más información ver rgrep --help");
        } else {    
            // args0 ignored as it is the program name
            let query = &args[1];
            let path = &args[2];
            // devolvemos un struct de argumentos
            Ok(
                Argumentos {
                    query: query.clone(),
                    ruta: path.clone()
                }
            )
        }
    }
}
use std::error::Error;
fn read_file(argumentos: &Argumentos) -> Result<(),Box<dyn Error>> {

    let contenido =fs::read_to_string(&argumentos.ruta)?;
    Ok(())
}

