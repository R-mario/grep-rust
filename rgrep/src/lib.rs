// #![warn(missing_docs)]

use std::{fs,env};
use std::error::Error;

#[derive(Debug)]
pub struct Argumentos {
    // TODO:
    // - cambiar ignore_case a que sea un tercer parametro opcional 
    // indicado con str (-i o --ignore-case)
    pub needle: String,
    pub haystack: String,
    ignore_case: bool
}
impl Argumentos {
    pub fn build(args: &Vec<String>) -> Result<Argumentos, &'static str> {
        if args.len() <= 2 {
            return Err("Pattern and file are needed \nmore info in rgrep --help");
        } else {    
            // args0 ignored as it is the program name
            let query = &args[1];
            let path = &args[2];
            let ignore_case = env::var("IGNORE_CASE").is_ok();
            
            // devolvemos un struct de argumentos
            Ok(
                Argumentos {
                    needle: query.clone(),
                    haystack: path.clone(),
                    ignore_case, //enviroment variable
                }
            )
        }
    }
}

/// lee y guarda los contenidos de un archivo
/// recibe una ref a un argumento, lee su ruta y devuelve un string o un error
pub fn run(argumentos: &Argumentos) -> Result<(),Box<dyn Error>> {

    let contenido = fs::read_to_string(&argumentos.haystack)?;
    let results = if argumentos.ignore_case {
        search_needle_ic(&argumentos.needle, &contenido)
    } else{
        search_needle(&argumentos.needle, &contenido)
    };

    for (n, line) in results {
        println!("{}-   {}",n,line)
    }


    Ok(())
}
// busqueda ignorando mayus
pub fn search_needle_ic<'a>(query: &str, contenido: &'a str) -> Vec<(usize,&'a str)> {
    let query = query.to_lowercase();
    let mut content = Vec::new();

    for (i,line) in contenido.lines().enumerate() {
        if line.to_lowercase().contains(&query){
            content.push((i+1,line))
        }
         
    }
    content
}

pub fn search_needle<'a>(query: &str, contenido: &'a str) -> Vec<(usize,&'a str)> {

    let mut content = Vec::new();

    for (i,line) in contenido.lines().enumerate() {
        if line.contains(query){
            content.push((i+1,line))
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
            vec![(2,"en un lugar de la mancha")], 
            search_needle(query, contenido),
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
            vec![(2,"en un lugar de la MANCHA"),(3,"de cuya mancha no quiero acordarme")], 
            search_needle_ic(query, contenido),
            "devuelve las lineas que contienen la 'query' ya sea en mayuscual o min");
    }
}

