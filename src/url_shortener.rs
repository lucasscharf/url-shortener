pub mod counter;
pub mod hash;

pub trait UrlShortener {
    fn shorten(&mut self, url: &str) -> bool;
    fn get(&mut self, key: &str) -> Option<&String>;
    fn list_all(&mut self) -> Vec<&String>;
}
