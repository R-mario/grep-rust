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

#[derive(PartialEq,Debug)]
struct Lines {
    n_line: usize,
    content: String
}
impl Lines {
    fn new(n: usize, line: &str) -> Lines {
        Lines {n_line: n, content: line.to_string()}
    }
}
/// lee y guarda los contenidos de un archivo
/// recibe una ref a un argumento, lee su ruta y devuelve un string o un error
pub fn run(argumentos: &Argumentos) -> Result<(),Box<dyn Error>> {

    let contenido = fs::read_to_string(&argumentos.haystack)?;
    let results = search_needle(&argumentos.needle, &contenido, argumentos.ignore_case);

    for line in results {
        println!("{}-   {}",line.n_line,line.content)
    }


    Ok(())
}

fn search_needle<'a>(query: &str, contenido: &'a str, ignore_case: bool) -> Vec<Lines> {

    let query = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    let mut content = Vec::new();

    for (i,line) in contenido.lines().enumerate() {
        let line_to_check = if ignore_case {
            line.to_lowercase()
        } else {
            line.to_owned()
        };

        if line_to_check.contains(&query){
            content.push(Lines::new(i+1,line))
        }
         
    }
    content
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_case_sens() {
        let query = "mancha";
        let contenido = r"\ 
en un lugar de la mancha
de cuyo nombre no quiero acordarme
vivía un hidalgo
...";
        assert_eq!(
            vec![Lines{n_line:2,content:"en un lugar de la mancha".to_string()}], 
            search_needle(query, contenido, false),
            "comprobando que devuelve las lineas que contienen la 'query'");
    }

    #[test]
    fn find_case_ins() {
        let query = "mancha";
        let contenido = r"\ 
en un lugar de la MANCHA
de cuya mancha no quiero acordarme
vivía un hidalgo
...";
        assert_eq!(
            vec![Lines{
                    n_line:2,
                    content:"en un lugar de la MANCHA".to_string()},
                Lines{
                    n_line:3,
                    content:"de cuya mancha no quiero acordarme".to_string()}
                ], 
            search_needle(query, contenido, true),
            "devuelve las lineas que contienen la 'query' ya sea en mayuscual o min");
    }
}

