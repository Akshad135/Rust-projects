use std::fs;
use std::env;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run <query> <file_path>");
        process::exit(1);
    }

    let query = &args[1];
    let file_path = Path::new(&args[2]);

    match file_read(file_path) {
        Ok(data) => search(query, &data),
        Err(_) => process::exit(1),
    }
}

fn file_read(path: &Path) -> Result<String, ()> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => {
            eprintln!("Error reading the file: {}", e);
            Err(())
        }
    }
}

fn search(query: &str, data: &str) {
    let mut final_data = vec![];
    for line in data.lines() {
        if line.contains(query) {
            final_data.push(line)
        }
    }

    if final_data.len() == 0{
        println!("Specified string does not exists");
    }else{
        for lines in final_data {
            println!("{}", lines)
        }
    }
}
