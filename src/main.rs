use crawler::getlinks;
use crawler::sortlink;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut webqueue: HashMap<String, i32> = HashMap::new();

    let seed = "https://waapple.org/varieties/all/";

    webqueue.insert(seed.to_string(), 0);

    webqueue = getlinks(seed, webqueue).await?;

    for i in 0..1000 {

        println!("#{} Collecting Links", i);

        //slow; needs better system, actual queue?
        let mut currentlink: String = "".to_string();
        for (key, _value) in webqueue.iter().take(i+1) {
            currentlink = key.clone();
        }

        webqueue = getlinks(currentlink.as_str(), webqueue).await?;
    }

    for link in &webqueue {
        if *link.1 > 1 {
            println!("{:?}", link);
        }
    }

    println!("\n\n\n");

    let reflist = sortlink(webqueue.clone(), 0, (webqueue.len()-1) as isize).await;

    println!("{:?}", reflist);

    
    for i in 0..reflist.1.len() {
        println!("[{}]   {}", reflist.1[i], reflist.0[i]);
    }

    Ok(())

}
