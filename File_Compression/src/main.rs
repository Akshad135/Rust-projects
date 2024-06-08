use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::copy;
use std::io;
use std::io::BufReader;
use std::time::Instant;

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

    let input_file = File::open(source_path)?;
    let mut input_file = BufReader::new(input_file);

    let output_file = File::create(output_path)?;

    let mut result = GzEncoder::new(output_file, Compression::default());

    let start = Instant::now();

    copy(&mut input_file, &mut result)?;
    let output = result.finish()?;

    println!("Source length: {:?}", input_file.get_ref().metadata().unwrap().len());
    println!("Output length: {:?}", output.metadata().unwrap().len());
    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}