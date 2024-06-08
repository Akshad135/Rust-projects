use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::copy;
use std::io;
use std::io::BufReader;
use std::time::Instant;

fn main(){

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

    let mut input_file = match File::open(source_path) {
        Ok(file) => BufReader::new(file),
        Err(error) => {
            println!("Failed to open file: {}", error);
            return;
        }
    };

    let output_file = match File::create(output_path) {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to create file: {}", error);
            return;
        }
    };

    let mut result = GzEncoder::new(output_file, Compression::default());
    let start = Instant::now();

    copy(&mut input_file, &mut result).unwrap();

    let output = result.finish().unwrap();

    println!("Source length: {:?}", input_file.get_ref().metadata().unwrap().len());
    println!("Output length: {:?}", output.metadata().unwrap().len());
    println!("Time taken: {:?}", start.elapsed());
}