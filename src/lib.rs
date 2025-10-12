use reqwest::Client;
use reqwest::Response;
use regex::Regex;
use std::collections::HashMap;

pub async fn getlinks(urltocrawl: &str, webqueue: HashMap<String, i32>) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    //create web client
    let webclient = Client::new();
    
    let webrequest: Response;

    //proceed if request doesn't error; end search for this site on error
    match webclient.get(urltocrawl).send().await {
        Ok(validreq) => webrequest = validreq,
        Err(_) => {
            println!{"Error on Get Request"};
            return Ok(webqueue)
        },
    }

    //regex pattern for getting <a hrefs>
    let relink = Regex::new("<a[^>]+href=\"(.*?)\"[^>]*>.*?</a>").unwrap();

    //current urls in use
    let mut urllist: HashMap<String, i32>;

    //clone other links stored
    urllist = webqueue.clone();

    //list of links on this site; used to prevent more than one increment on ref counts
    let mut linksonsite: Vec<String> = vec![];

    if webrequest.status().is_success() {

        let webhtml = webrequest.text().await?;

        //apply first regex pattern
        for hrefbracket in relink.find_iter(&webhtml) {

            //filter out whitespace
            let href: String = hrefbracket.as_str().chars().filter(|c| !c.is_whitespace()).collect();

            //regex pattern for http links
            let rehttp = Regex::new("\"(?:http.*?)\"").unwrap();

            //apply second regex pattern
            for link in rehttp.find_iter(&href) {
                let url = &link.as_str()[1.. link.as_str().len()-1];
                //add link not found previously
                if !urllist.contains_key(&url.to_string()) {
                    //add to all links list
                    urllist.insert(url.to_string(), 1);

                    //add to this site's list
                    linksonsite.push(url.to_string());
                }
                else if !linksonsite.contains(&url.to_string()) {
                //increment link reference on repeat find *for the first instance on site
                //no repeats on same site
                    let mut refstolink: i32 = *urllist.get(url).unwrap();
                    refstolink += 1;
                    urllist.insert(url.to_string(), refstolink);
                }
            }
        }
    }

    Ok(urllist)
}

fn sortlinkpartition (links: (&mut Vec<String>, &mut Vec<i32>), low: isize, high: isize) -> isize {

    let mut index: isize = low - 1;

    let linkurls: &mut Vec<String> = links.0;
    let linkrefs: &mut Vec<i32> = links.1;
    let x = linkrefs[high as usize];

    for j in low..high {
        if (linkrefs[j as usize] as isize) <= (x as isize) {
            index += 1;
            //swap(linkurls[index], linkurls[j]);
            linkrefs.swap(index as usize, j as usize);
            linkurls.swap(index as usize, j as usize);
        }
    }

    linkrefs.swap((index+1) as usize, high as usize);
    linkurls.swap((index+1) as usize, high as usize);

    return index+1;

}

pub async fn sortlink (links: HashMap<String, i32>, mut low: isize, mut high: isize) -> (Vec<String>, Vec<i32>) {//(Vec<String>, Vec<i32>) {

    let mut linkurls: Vec<String> = links.keys().cloned().collect();
    let mut linkrefs: Vec<i32> = links.values().cloned().collect();

    let size = high-low + 1;
    let mut stackrefs = vec![0; size as usize]; //+1?

    let mut top: isize = -1;

    top = top + 1;
    stackrefs[top as usize] = low;
    top = top + 1;
    stackrefs[top as usize] = high;

    while top >= 0 {

        high = stackrefs[top as usize];
        top = top-1;
        low = stackrefs[top as usize];
        top = top-1;

        let partition = sortlinkpartition((&mut linkurls, &mut linkrefs), low, high);

        if ( partition-1 ) > low{
            top = top + 1;
            stackrefs[top as usize] = low;
            top = top + 1;
            stackrefs[top as usize] = partition -1;
        }
        
        if ( partition + 1) < high {
            top = top + 1;
            stackrefs[top as usize] = partition + 1;
            top = top + 1;
            stackrefs[top as usize] = high;
        }

    }

    return (linkurls, linkrefs);
}
