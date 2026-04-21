use super::UrlShortener;
use hex;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct HashShortener {
    urls: HashMap<String, String>,
}

impl UrlShortener for HashShortener {
    fn shorten(&mut self, url: &str) -> bool {
        let digest = Sha256::digest(url.as_bytes());
        let key: String = hex::encode(digest);
        self.urls.insert(key, String::from(url));
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

    fn novo() -> HashShortener {
        return HashShortener::default();
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
    fn get_retorna_none_para_chave_inexistente() {
        let mut s = novo();
        assert_eq!(
            s.get("4b59642f5a13d013f9a0ae0c70d815c320d846f6333ab46323c594603baff5d5"),
            None
        );

        s.shorten("https://a.com");
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
    fn urls_duplicadas_recebem_chaves_iguais() {
        let mut s = novo();
        s.shorten("https://a.com");
        s.shorten("https://a.com");

        let mesma = String::from("https://a.com");
        assert_eq!(
            s.get("4b59642f5a13d013f9a0ae0c70d815c320d846f6333ab46323c594603baff5d5"),
            Some(&mesma)
        );
        assert_eq!(
            s.get("4b59642f5a13d013f9a0ae0c70d815c320d846f6333ab46323c594603baff5d5"),
            Some(&mesma)
        );
        assert_eq!(s.list_all().len(), 1);
    }
}
