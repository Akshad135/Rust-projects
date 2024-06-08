use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::io::{self, BufWriter, Write};
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

    let data = ApiInfo {
        api_key: "hfhfhsjjaanecienen".to_string(),
        model: "llama3 70b".to_string(),
    };

    let _ = write_json(file_path, &data);

    // ? reading the json file
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

fn write_json(file_path : &Path, data : &ApiInfo) -> Result<(), io::Error>{
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    let _ = serde_json::to_writer(&mut writer, &data);
    let _ = writer.flush()?;
    Ok(())
}