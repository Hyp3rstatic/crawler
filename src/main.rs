use reqwest::Client;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut webclient = Client::new();
    
    let mut webrequest = webclient.get("https://www.rust-lang.org").send().await?;

    let relink = Regex::new("<a[^>]+href=\"(.*?)\"[^>]*>.*?</a>").unwrap();

    if webrequest.status().is_success() {
        let webhtml = webrequest.text().await?;
        let weblinks: Vec<&str> = webhtml.matches("href").collect();

        
        let mut webqueue: Vec<String> = vec![];

        //println!("{:?}", weblinks);
        for hrefbracket in relink.find_iter(&webhtml) {
            //println!("Found: {}", hrefbracket.as_str());
            let mut href: String = hrefbracket.as_str().chars().filter(|c| !c.is_whitespace()).collect();

            let rehttp = Regex::new("\"(?:http.*?)\"").unwrap();

            //let link = rehttp.find(&href);
            
            for link in rehttp.find_iter(&href) {
                let url = &link.as_str()[1.. link.as_str().len()-1];
                //println!("{}", text);
                webqueue.push(url.to_string());
            }
            
        }
        //println!("{:?}", webqueue);
        for link in &webqueue {
            println!("{}", link);
        }
    }
        
    Ok(())

}
