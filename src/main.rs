use crawler::getlinks;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut webqueue: HashMap<String, i32> = HashMap::new();

    let mut seed = "https://rust-lang.org/";

    webqueue.insert(seed.to_string(), 0);

    webqueue = getlinks(seed, webqueue).await?;

    for i in 0..100 {

        println!("#{} Collecting Links", i);

        //super slow needs better system desperately, actual queue structure?
        let mut currentlink: String = "".to_string();
        for (key, value) in webqueue.iter().take(i+1) {
            currentlink = key.clone();
        }

        webqueue = getlinks(currentlink.as_str(), webqueue).await?;
    }

    for link in &webqueue {
        if *link.1 > 1 {
            println!("{:?}", link);
        }
    }

    Ok(())

}
