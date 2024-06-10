
fn main()-> Result<(), Box<dyn std::error::Error>>{

    let path = "/path/to/file.csv";
    
    let reader = csv_to_table::from_path(path)?;
    println!("{}", reader);


    Ok(())
}