use std::{collections::HashMap, marker::PhantomData};

struct Counter;
struct Hash;

trait VariantShortner {
    fn shorten(&mut self, url: &str) -> bool;
    fn get(&mut self, key: &str) -> Option<&String>;
    fn list_all(&mut self) -> Vec<&String>;
}

#[derive(Debug, Default)]
struct UrlShortner<T> {
    urls: HashMap<String, String>,
    _t: PhantomData<T>,
}

impl VariantShortner for UrlShortner<Counter> {
    fn shorten(&mut self, url: &str) -> bool {
        let size = self.urls.len().to_string();
        self.urls.insert(size, String::from(url));
        return true;
    }

    fn get(&mut self, key: &str) -> Option<&String> {
        return self.urls.get(key);
    }

    fn list_all(&mut self) -> Vec<&String> {
        let mut all_urls: Vec<&String> = Vec::new();
        for url in self.urls.iter() {
            all_urls.push(url.1);
        }

        return all_urls;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn novo() -> UrlShortner<Counter> {
        UrlShortner {
            urls: HashMap::new(),
            _t: PhantomData,
        }
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
        assert!(s.list_all().is_empty());
    }

    #[test]
    fn list_all_contem_todas_as_urls_inseridas() {
        let mut s = novo();
        s.shorten("https://a.com");
        s.shorten("https://b.com");

        let todas = s.list_all();
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
        assert_eq!(s.list_all().len(), 2);
    }
}
