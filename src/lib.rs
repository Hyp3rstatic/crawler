use reqwest::Client;
use regex::Regex;

pub async fn getlinks(urltocrawl: &str, webqueue: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut webclient = Client::new();
    
    let mut webrequest = webclient.get(urltocrawl).send().await?;

    let relink = Regex::new("<a[^>]+href=\"(.*?)\"[^>]*>.*?</a>").unwrap();

    let mut urllist: Vec<String> = vec![];
    urllist = webqueue.clone();

    if webrequest.status().is_success() {
        let webhtml = webrequest.text().await?;
        let weblinks: Vec<&str> = webhtml.matches("href").collect();

        //println!("{:?}", weblinks);
        for hrefbracket in relink.find_iter(&webhtml) {
            //println!("Found: {}", hrefbracket.as_str());
            let mut href: String = hrefbracket.as_str().chars().filter(|c| !c.is_whitespace()).collect();

            let rehttp = Regex::new("\"(?:http.*?)\"").unwrap();

            //let link = rehttp.find(&href);
            
            for link in rehttp.find_iter(&href) {
                let url = &link.as_str()[1.. link.as_str().len()-1];
                //println!("{}", text);
                if !urllist.contains(&url.to_string()) {
                    urllist.push(url.to_string());
                }
            }
            
        }
    }
        
    Ok(urllist)
}