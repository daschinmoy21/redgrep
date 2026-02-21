pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str,contents: &'a str)->Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:safe,and productive.
            Duct tape.
            Pick three.";

        assert_eq!(vec!["Rust:safe,and productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
            Rust:
            fast,safe and productive.
            Pick three.
            Trust me.";

            assert_eq!(vec!["Rust:", "            Trust me."],search_case_insensitive(query, contents))
    }

    #[test]

    fn no_result() {
        let query = "foobar";
        let contents = "\
            Rust:safe,and productive.
            Pick three.";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "Rust";
        let contents = "\
            Rust:safe,and productive.
            Duct tape.
            rust is great.
            Trust me.";

        assert_eq!(vec!["Rust:safe,and productive."], search(query, contents));
    }
}
