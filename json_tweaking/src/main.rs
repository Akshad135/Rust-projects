use serde::{Deserialize, Serialize};
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Serialize)]
struct ApiInfo{
    api_key : String,
    model : String,
}


fn main() -> Result<(), Box<dyn std::error::Error>>{

    let file_path = std::path::Path::new("test.json");
    let extracted_info: ApiInfo= read_json(&file_path)?;
    println!("\nApi key: {}\nModel: {}", extracted_info.api_key, extracted_info.model);

    Ok(())
}

fn read_json(file_path : &Path) -> Result<ApiInfo, io::Error>{
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let content = serde_json::from_reader(reader)?;
    Ok(content)
}