use reqwest::Client;
use reqwest::Response;
use regex::Regex;
use std::collections::HashMap;

pub async fn getlinks(urltocrawl: &str, webqueue: HashMap<String, i32>) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut webclient = Client::new();
    
    //let mut webrequest = webclient.get(urltocrawl).send().await?;

    let mut webrequest: Response;

    match(webclient.get(urltocrawl).send().await){
        Ok(validreq) => webrequest = validreq,
        Err(_) => {println!{"Error on Get Request"}; return Ok(webqueue)},
    }

    let relink = Regex::new("<a[^>]+href=\"(.*?)\"[^>]*>.*?</a>").unwrap();

    let mut urllist: HashMap<String, i32> = HashMap::new();
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
                if !urllist.contains_key(&url.to_string()) {
                    urllist.insert(url.to_string(), 1);
                }
                else {
                    let mut refstolink: i32 = *urllist.get(url).unwrap();
                    refstolink += 1;
                    urllist.insert(url.to_string(), refstolink);
                }
            }
            
        }
    }
        
    Ok(urllist)
}