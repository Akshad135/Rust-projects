use error_chain::error_chain;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main()-> Result<()>{


    // ? using a block request
    // let req = reqwest::blocking::get("https://www.rust-lang.org")?;

    // ? using async request
    let req = reqwest::get("https://www.rust-lang.org").await?;
    println!("{:?}", req.status());
    println!("{:?}", req.headers());

    Ok(())
}