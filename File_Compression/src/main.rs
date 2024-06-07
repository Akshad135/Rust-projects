extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::io::prelude::*;
use std::time::Instant;

fn main(){
    if args().len() != 3{
        eprintln!("usage: source output");
        return;
    }
    let mut input_file = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output_file = File::create(args().nth(2).unwrap()).unwrap();

    let mut result = GzEncoder::new(output_file, Compression::default());
    let start = Instant::now();

    copy(&mut input_file, &mut result).unwrap();

    let output = result.finish().unwrap();

    println!("Source length: {:?}", input_file.get_ref().metadata().unwrap().len());
    println!("Output length: {:?}", output.metadata().unwrap().len());
    println!("Time taken: {:?}", start.elapsed());
}