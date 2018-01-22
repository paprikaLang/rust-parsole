extern crate clap;
use clap::{Arg};
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate prettytable;
pub static URL: &str = "https://newsapi.org/v2/sources?language=en&apiKey=c16fc7641ca44e0f85995fbbe7a5ae09";
use clap::App;

#[derive(Debug,Deserialize,Serialize)]
struct FeedItem {
    id: String,
    name: String,
    description: String,
    url: String,
    category: String,
    language: String,
    country: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Feed {
    status: String,
    sources: Vec<FeedItem>,
}


fn get_feed() -> Feed {
    let client = reqwest::Client::new();
    let mut request = client.get(URL);

    let mut resp = request.send().unwrap();

    assert!(resp.status().is_success());

    let data = resp.text().unwrap();

    serde_json::from_str(&data).unwrap()
}

fn print_count(feed: &Feed) {
    println!("Number of posts: {}",feed.sources.len());
}
fn print_feed_table<'feed, I: Iterator<Item = &'feed FeedItem>>(items: I) {
	let mut table = prettytable::Table::new();
	table.add_row(row!["Name","Desc","URL"]);
	for item in items {
	    let name = if item.name.len() >= 50 {
	    	&item.name[0..49]
	    }else{
	    	&item.name
	    };
	    let desc = if item.description.len() >= 50 {
	    	&item.description[0..49]
	    }else{
	    	&item.description
	    };
	    table.add_row(row![name,desc,item.url]);
	}
	table.printstd();
}
fn main() {
	let app = App::new("parsole")
	    .version("0.1")
	    .author("paprikaLang")
	    .about("Read JSON")
	    .arg(Arg::with_name("count")
	    	 .short("c")
	    	 .long("count")
	    	 .help("Show the count of posts"))
	    .arg_from_usage("-n, --number=[NUMBER] 'Only print the NUMBER most recent posts'");
	let matches = app.get_matches();
    let feed = get_feed();
    if matches.is_present("count") {
        print_count(&feed);
    }else{
    	let iter = feed.sources.iter();
    	if let Some(string) = matches.value_of("number") {
    		let number = string.parse().unwrap();
    		print_feed_table(iter.take(number))
    	}else {
    		print_feed_table(iter)
    	}
    }
    
}
