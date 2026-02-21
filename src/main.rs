use redgrep::search;
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path.display());

    let contents =
        fs::read_to_string(&config.file_path).expect("Should have been able to read the file");

    for line in search(&config.query, &contents) {
        println!("{line}")
    }
    Ok(())
}

struct Config {
    query: String,
    file_path: PathBuf,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments! Use `redgrep <pattern> <path/filename>`");
        }
        let query = args[1].clone();
        let file_path = PathBuf::from(&args[2]);

        Ok(Config { query, file_path })
    }
}
