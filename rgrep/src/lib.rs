use std::{fs,env};
use std::error::Error;

#[derive(Debug)]
pub struct Argumentos {
    // TODO:
    // - cambiar ignore_case a que sea un tercer parametro opcional 
    // indicado con str (-i o --ignore-case)
    query: String,
    ruta: String,
    ignore_case: bool,
}
impl Argumentos {
    pub fn build(args: &Vec<String>) -> Result<Argumentos, &'static str> {
        if args.len() > 3 {
            return Err("rgrep solo acepta 2 argumentos\npara más información ver rgrep --help");
        } else if args.len() <= 2 {
            return Err("debes especificar un archivo\npara más información ver rgrep --help");
        } else {    
            // args0 ignored as it is the program name
            let query = &args[1];
            let path = &args[2];
            let ignore_case = env::var("IGNORE_CASE").is_ok();
            // devolvemos un struct de argumentos
            Ok(
                Argumentos {
                    query: query.clone(),
                    ruta: path.clone(),
                    ignore_case,
                }
            )
        }
    }
}


// lee y guarda los contenidos de un archivo
// recibe una ref a un argumento, lee su ruta y devuelve un string o un error
pub fn read_file(argumentos: &Argumentos) -> Result<(),Box<dyn Error>> {

    let contenido = fs::read_to_string(&argumentos.ruta)?;
    let results = if argumentos.ignore_case{
        busqueda_in(&argumentos.query, &contenido)
    } else {
        busqueda(&argumentos.query, &contenido)
    };
    for linea in results{
        println!("{linea}");
    }
    Ok(())
}
// ToDo:
// - 
pub fn busqueda_in<'a>(query: &str, contenido: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut content = Vec::new();

    for line in contenido.lines() {
        if line.to_lowercase().contains(&query){
            content.push(line.clone())
        }
         
    }
    content
}

pub fn busqueda<'a>(query: &str, contenido: &'a str) -> Vec<&'a str> {

    let mut content = Vec::new();

    for line in contenido.lines() {
        if line.contains(query){
            content.push(line.clone())
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

