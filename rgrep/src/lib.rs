use std::process::Command;
use std::{fs,env};
use std::error::Error;

#[derive(Debug,Clone)]
pub struct CommandArgs {
    ignoreCase: bool ,
    nLine: bool, 
    help: bool,
}
impl CommandArgs {
    pub fn build(args: &[String]) -> Option<CommandArgs> {
        if args.len() == 0 {
            None
        } else {
        let mut ignoreCase = false;
        let mut nLine = false;
        let mut help = false;

        if args.len() > 0 {
            ignoreCase = args.contains(&String::from("--ignoreCase"));
            nLine = args.contains(&String::from("-n"));
            help = args.contains(&String::from("--help"));
        }

        Some( CommandArgs { ignoreCase, nLine, help } )
    }
    }
}

#[derive(Debug)]
pub struct Argumentos {
    // TODO:
    // - cambiar ignore_case a que sea un tercer parametro opcional 
    // indicado con str (-i o --ignore-case)
    query: String,
    ruta: String,
    ignore_case: bool,
    argum: Option<CommandArgs>,
}
impl Argumentos {
    pub fn build(args: &Vec<String>) -> Result<Argumentos, &'static str> {
        if args.len() <= 2 {
            return Err("debes especificar un patron y un archivo\npara más información ver rgrep --help");
        } else {    
            // args0 ignored as it is the program name
            let query = &args[1];
            let path = &args[2];
            let ignore_case = env::var("IGNORE_CASE").is_ok();
            let argum =  CommandArgs::build(&args[3..]);
            
            // devolvemos un struct de argumentos
            Ok(
                Argumentos {
                    query: query.clone(),
                    ruta: path.clone(),
                    ignore_case, //enviroment variable
                    argum: argum,
                }
            )
        }
    }
}

// lee y guarda los contenidos de un archivo
// recibe una ref a un argumento, lee su ruta y devuelve un string o un error
pub fn ejecuta(argumentos: &Argumentos) -> Result<(),Box<dyn Error>> {

    let contenido = fs::read_to_string(&argumentos.ruta)?;
    let other_args = &argumentos.argum.as_ref().unwrap();
    let results = if argumentos.ignore_case || other_args.ignoreCase{
        busqueda_in(&argumentos.query, &contenido)
    } else{
        busqueda(&argumentos.query, &contenido)
    };
    // arreglar cuando no se introducen args adicionales
    if other_args.nLine {
        for linea in results{
            // aqui decidir si se printa o no el n lineas
            println!("{}--{}",linea.0,linea.1);
        }
    } else {
        for linea in results{
            // aqui decidir si se printa o no el n lineas
            println!("{}",linea.1);
        }
    }

    Ok(())
}
// busqueda ignorando mayus
pub fn busqueda_in<'a>(query: &str, contenido: &'a str) -> Vec<(usize,&'a str)> {
    let query = query.to_lowercase();
    let mut content = Vec::new();

    for (i,line) in contenido.lines().enumerate() {
        if line.to_lowercase().contains(&query){
            content.push((i,line.clone()))
        }
         
    }
    content
}

pub fn busqueda<'a>(query: &str, contenido: &'a str) -> Vec<(usize,&'a str)> {

    let mut content = Vec::new();

    for (i,line) in contenido.lines().enumerate() {
        if line.contains(query){
            content.push((i,line.clone()))
        }
         
    }
    content
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encuentra_uno() {
        let query = "mancha";
        let contenido = r"\ 
en un lugar de la mancha
de cuyo nombre no quiero acordarme
vivía un hidalgo
...";
        assert_eq!(
            vec!["en un lugar de la mancha"], 
            busqueda(query, contenido),
            "comprobando que devuelve las lineas que contienen la 'query'");
    }

    #[test]
    fn case_insensitive() {
        let query = "mancha";
        let contenido = r"\ 
en un lugar de la MANCHA
de cuya mancha no quiero acordarme
vivía un hidalgo
...";
        assert_eq!(
            vec!["en un lugar de la MANCHA","de cuya mancha no quiero acordarme"], 
            busqueda_in(query, contenido),
            "devuelve las lineas que contienen la 'query' ya sea en mayuscual o min");
    }
}

