extern crate clap;
extern crate reqwest;

pub static URL: &str = "https://newsapi.org/v2/sources?language=en&apiKey=c16fc7641ca44e0f85995fbbe7a5ae09";
use clap::App;

fn get_feed() -> String {
    let client = reqwest::Client::new();
    let mut request = client.get(URL);
    let mut resp = request.send().unwrap();
    assert!(resp.status().is_success());
    return resp.text().unwrap();
}

fn main() {
	let app = App::new("parsole")
	    .version("0.1")
	    .author("paprikaLang")
	    .about("Read JSON")
	    .arg_from_usage("-n, --number=[NUMBER] 'Only print the NUMBER most recent posts'
	    	             -c, --count           'Show the count of posts'");
	let _matches = app.get_matches();
    let _feed = get_feed();
    
}
