use super::{UrlShortener};
use std::{collections::HashMap};

#[derive(Debug, Default)]
pub struct CounterShortener {
    urls: HashMap<String, String>
}

impl UrlShortener for CounterShortener {
    fn shorten(&mut self, url: &str) -> bool {
        let size = self.urls.len().to_string();
        self.urls.insert(size, String::from(url));
        return true;
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

    fn novo() -> CounterShortener {
        return CounterShortener::default();
    }

    #[test]
    fn shorten_retorna_true() {
        let mut s = novo();
        assert!(s.shorten("https://a.com"));
    }

    #[test]
    fn shorten_usa_indice_sequencial_como_chave() {
        let mut s = novo();
        s.shorten("https://a.com");
        s.shorten("https://b.com");
        s.shorten("https://c.com");

        assert_eq!(s.get("0"), Some(&String::from("https://a.com")));
        assert_eq!(s.get("1"), Some(&String::from("https://b.com")));
        assert_eq!(s.get("2"), Some(&String::from("https://c.com")));
    }

    #[test]
    fn get_retorna_none_para_chave_inexistente() {
        let mut s = novo();
        assert_eq!(s.get("0"), None);

        s.shorten("https://exemplo.com");
        assert_eq!(s.get("99"), None);
    }

    #[test]
    fn list_all_vazia_quando_nada_inserido() {
        let mut s = novo();
        assert!(s.list_values().is_empty());
    }

    #[test]
    fn list_all_contem_todas_as_urls_inseridas() {
        let mut s = novo();
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
    fn urls_duplicadas_recebem_chaves_diferentes() {
        let mut s = novo();
        s.shorten("https://a.com");
        s.shorten("https://a.com");

        let mesma = String::from("https://a.com");
        assert_eq!(s.get("0"), Some(&mesma));
        assert_eq!(s.get("1"), Some(&mesma));
        assert_eq!(s.list_values().len(), 2);
    }
}
