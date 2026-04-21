mod url_shortener;
use url_shortener::counter::CounterShortener;
use url_shortener::hash::HashShortener;
use url_shortener::UrlShortener;

fn main() {
    println!("Hello, world!");

    let mut counter = CounterShortener::default();
    counter.shorten("URL");
    println!("{counter:?}");

    let mut hasher = HashShortener::default();
    hasher.shorten("https://a.com");
    hasher.shorten("https://b.com");
    hasher.shorten("https://c.com");

    println!("{hasher:?}");
}
