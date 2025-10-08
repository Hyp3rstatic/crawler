use crawler::getlinks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut webqueue: Vec<String> = vec![];

    webqueue = getlinks("https://www.rust-lang.org", webqueue).await?;
    webqueue = getlinks("https://google.com", webqueue).await?;

    //println!("{:?}", webqueue);
    for link in &webqueue {
        println!("{}", link);
    }

    Ok(())

}
