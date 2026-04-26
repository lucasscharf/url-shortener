use rusqlite::{Connection, params};

pub struct Database {
    conn: Connection,
}
impl Database {
    const INSERT: &str = "INSERT INTO url(key, value) VALUES (?,?)";
    const SELECT: &str = "SELECT * FROM url";

    pub fn new(conn: Connection) -> Self {
        let _ = conn
            .execute(
                "CREATE TABLE IF NOT EXISTS url (
            key TEXT NOT NULL,
            value TEXT NOT null
        )",
                (),
            )
            .unwrap();

        return Self { conn };
    }

    pub fn select_all(&self) -> Result<Vec<(String, String)>, rusqlite::Error> {
        let conn = &self.conn;

        let mut stmt = conn.prepare(Self::SELECT)?;
        let iterator = stmt.query_map([], |row| {
            let key: String = row.get(0)?;
            let value: String = row.get(1)?;

            return Ok((key, value));
        })?;

        let mut ans = Vec::new();

        for urls in iterator {
            let url = urls?;
            ans.push((url.0, url.1));
        }

        return Ok(ans);
    }

    pub fn insert(&self, url: (String, String)) -> Result<(), rusqlite::Error> {
        self.conn.execute(Self::INSERT, params![url.0, url.1])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new() -> Database {
        let conn = Connection::open_in_memory().unwrap();

        Database::new(conn)
    }

    #[test]
    fn insert_pair_string_should_return_with_select() {
        let database = new();
        let pair = (String::from("1"), String::from("2"));
        let _ = database.insert(pair).unwrap();
        let all = database.select_all().unwrap();
        assert_eq!(1, all.len());
        let next = all.iter().next().unwrap();
        assert_eq!(next.0, String::from("1"));
        assert_eq!(next.1, String::from("2"));
    }

    #[test]
    fn select_return_empty_if_no_element() {
        let database = new();
        let items = database.select_all().unwrap();

        assert_eq!(0, items.len());
    }
}
