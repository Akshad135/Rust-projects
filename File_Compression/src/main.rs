use flate2::write::GzEncoder;
use flate2::Compression;
use std::ffi::OsStr;
use std::fs::File;
use std::{io, path};
use std::io::{copy, BufReader, BufWriter, Read, Write};
use std::time::Instant;
use flate2::read::GzDecoder;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>>{

    println!("Enter the path to source file: ");
    let mut source_path = String::new();
    io::stdin().read_line(&mut source_path)
        .expect("Failed to read the source path");

    let source_path: String = source_path.trim().parse()
        .expect("Failed to parse as string");

    println!("Enter the path to output file (also include the file name): ");
    let mut output_path = String::new();
    io::stdin().read_line(&mut output_path)
        .expect("Failed to read the output path");

    let output_path: String = output_path.trim().parse()
    .expect("Failed to parse as string");

    let input_file = File::open(&source_path)?;
    let mut input_file = BufReader::new(input_file);

    let output_file = File::create(&output_path)?;

    let mut result = GzEncoder::new(output_file, Compression::default());

    let start = Instant::now();

    copy(&mut input_file, &mut result)?;
    let output = result.finish()?;

    println!("Source length: {:?}", input_file.get_ref().metadata().unwrap().len());
    println!("Output length: {:?}", output.metadata().unwrap().len());
    println!("Time taken: {:?}", start.elapsed());



    print!("Do you want to decode the file? (y/n): " );
    io::stdout().flush().expect("Failed to flush stdout");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
        .expect("Failed to read the source path");

    let choice: char = choice.trim().parse()
        .expect("Failed to parse as string");

    let choice = choice.to_ascii_lowercase();
    if choice == 'y'{
        let source_path = std::path::Path::new(source_path.as_str());

        let extension: &OsStr = match source_path.extension() {
            Some(value) => value,
            None => return Err("No file extension found".into()),
        };
    
        let output_path_decode: &str = &output_path.as_str();
        decoder(&output_path_decode, extension)?;
    };


    Ok(())
}

fn decoder(path_to_file : &str, extension : &OsStr) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path_to_file)?;
    let mut gz = GzDecoder::new(file);
    let mut data = String::new();
    gz.read_to_string(&mut data)?;
    let output_file_path = PathBuf::from(path_to_file).with_extension(extension);
    File::create(&output_file_path)?;
    write_file(&output_file_path, &data)?;
    let file_name = match output_file_path.file_stem(){
        Some(value) => value,
        None => return Err("No file name found".into()),
    };
    println!("Decoded the file with name: {:?}", file_name);
    Ok(())
}

fn write_file(file_path : &Path, data : &str) -> Result<(), Box<dyn std::error::Error>>{
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    let _ = writer.write_all(data.as_bytes())?;
    let _ = writer.flush()?;
    Ok(())
}