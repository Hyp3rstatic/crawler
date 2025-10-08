use crawler::getlinks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut webqueue: Vec<String> = vec![];

    let mut seed = "https://www.rust-lang.org";

    webqueue.push(seed.to_string());

    webqueue = getlinks(seed, webqueue).await?;

    for i in 0..10 {
        webqueue = getlinks(&webqueue.clone()[i+1], webqueue).await?;
    }

    //println!("{:?}", webqueue);
    for link in &webqueue {
        println!("{}", link);
    }

    Ok(())

}
