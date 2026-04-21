pub mod counter;

pub trait UrlShortner {
    fn shorten(&mut self, url: &str) -> bool;
    fn get(&mut self, key: &str) -> Option<&String>;
    fn list_all(&mut self) -> Vec<&String>;
}
