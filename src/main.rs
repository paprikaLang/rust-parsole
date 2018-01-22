extern crate clap;
use clap::App;

fn main() {
	let app = App::new("parsole")
	    .version("0.1")
	    .author("paprikaLang")
	    .about("Read JSON")
	    .arg_from_usage("-n, --number=[NUMBER] 'Only print the NUMBER most recent posts'
	    	             -c, --count           'Show the count of posts'");
	let _matches = app.get_matches();
    
}
