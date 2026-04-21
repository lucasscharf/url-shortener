mod url_shortener;
use url_shortener::counter::CounterShortener;
use url_shortener::UrlShortner;

fn main() {
    println!("Hello, world!");

    let mut shortener = CounterShortener::default();
    shortener.shorten("URL");

    println!("{shortener:?}");
}
