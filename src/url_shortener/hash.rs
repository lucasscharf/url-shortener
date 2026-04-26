use super::UrlShortener;
use hex;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug)]
pub struct HashShortener {
    urls: HashMap<String, String>,
}

impl HashShortener {
    pub fn new(urls: HashMap<String, String>) -> Self {
        Self { urls: urls }
    }
}

impl UrlShortener for HashShortener {
    fn shorten(&mut self, url: &str) -> String {
        let digest = Sha256::digest(url.as_bytes());
        let key: String = hex::encode(digest);
        self.urls.insert(key.clone(), String::from(url));
        return key;
    }

    fn get(&mut self, key: &str) -> Option<&String> {
        return self.urls.get(key);
    }

    fn list_values(&mut self) -> Vec<&String> {
        let mut all_urls: Vec<&String> = Vec::new();
        for url in self.urls.iter() {
            all_urls.push(url.1);
        }

        return all_urls;
    }

    fn list_keys(&mut self) -> Vec<&String> {
        let mut all_keys: Vec<&String> = Vec::new();
        for url in self.urls.iter() {
            all_keys.push(url.0);
        }

        return all_keys;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new() -> HashShortener {
        return HashShortener::new(HashMap::new());
    }

    #[test]
    fn shorten_returns_true() {
        let mut s = new();
        assert_eq!(
            "4b59642f5a13d013f9a0ae0c70d815c320d846f6333ab46323c594603baff5d5",
            s.shorten("https://a.com")
        );
    }

    #[test]
    fn shorten_uses_hash_as_index() {
        let mut s = new();
        s.shorten("https://a.com");
        s.shorten("https://b.com");
        s.shorten("https://c.com");

        assert_eq!(
            s.get("4b59642f5a13d013f9a0ae0c70d815c320d846f6333ab46323c594603baff5d5"),
            Some(&String::from("https://a.com"))
        );
        assert_eq!(
            s.get("d6fd2a8e03cac6d7ccc7ad18558a46caecf3228bdde5252ac586e9e9661fd379"),
            Some(&String::from("https://b.com"))
        );
        assert_eq!(
            s.get("d149ea8c04719973fb30a10529772d99aab483bd1f6368521bf2c42dfac33c77"),
            Some(&String::from("https://c.com"))
        );
    }

    #[test]
    fn get_return_none_to_unexistent_key() {
        let mut s = new();
        assert_eq!(
            s.get("4b59642f5a13d013f9a0ae0c70d815c320d846f6333ab46323c594603baff5d5"),
            None
        );

        s.shorten("https://a.com");
        assert_eq!(s.get("99"), None);
    }

    #[test]
    fn list_all_returns_empty_when_there_is_no_element() {
        let mut s = new();
        assert!(s.list_values().is_empty());
    }

    #[test]
    fn list_all_have_all_urls() {
        let mut s = new();
        s.shorten("https://a.com");
        s.shorten("https://b.com");

        let todas = s.list_values();
        let a = String::from("https://a.com");
        let b = String::from("https://b.com");

        assert_eq!(todas.len(), 2);
        assert!(todas.contains(&&a));
        assert!(todas.contains(&&b));
    }

    #[test]
    fn shorten_same_url_twice_receive_same_key() {
        let mut s = new();
        s.shorten("https://a.com");
        s.shorten("https://a.com");

        let same = String::from("https://a.com");
        assert_eq!(
            s.get("4b59642f5a13d013f9a0ae0c70d815c320d846f6333ab46323c594603baff5d5"),
            Some(&same)
        );
        assert_eq!(
            s.get("4b59642f5a13d013f9a0ae0c70d815c320d846f6333ab46323c594603baff5d5"),
            Some(&same)
        );
        assert_eq!(s.list_values().len(), 1);
    }
}
