mod database;
mod url_shortener;

use std::collections::HashMap;

use config_file::FromConfigFile;
use csv::Reader;
use database::Database;
use inquire::{Select, Text};
use rusqlite::Connection;
use serde::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use url_shortener::UrlShortener;
use url_shortener::counter::CounterShortener;
use url_shortener::hash::HashShortener;

#[derive(Deserialize, Debug)]
struct Config {
    shortenen_algorithm: String,
    mode: Mode,
    batch_file_path: String,
    persistent: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
enum Mode {
    Interactive,
    Batch,
}

#[derive(Deserialize, Debug)]
struct Operation {
    operation: Operations,
    url: String,
}

#[derive(Debug, PartialEq, EnumString, Display, EnumIter, Clone, Deserialize)]
enum Operations {
    Shorten,
    Retrieve,
    List,
    #[strum(to_string = "List Keys")]
    ListKeys,
    Exit,
}

fn batch_mode(mut shortener: Box<dyn UrlShortener>, config: Config, database: Option<Database>) {
    let batch_file_path = config.batch_file_path;
    let mut rdr = Reader::from_path(batch_file_path).unwrap();

    for result in rdr.deserialize::<Operation>() {
        let record = result.unwrap();

        match record.operation {
            Operations::Shorten => {
                let key = shortener.shorten(record.url.as_str());
                if let Some(db) = &database {
                    let _ = db.insert((key, record.url));
                }
            }
            Operations::Retrieve => {
                let value = shortener.get(record.url.as_str());
                if value.is_none() {
                    println!("Chave não existente");
                    continue;
                }
                println!("{}", value.unwrap());
            }
            Operations::List => {
                println!("{:?}", shortener.list_values());
            }
            Operations::ListKeys => {
                println!("{:?}", shortener.list_keys());
            }
            Operations::Exit => {
                break;
            }
        }
    }
}

fn interactive_mode(mut shortener: Box<dyn UrlShortener>, database: Option<Database>) {
    loop {
        let options: Vec<Operations> = Operations::iter() //
            .collect();

        let ans = Select::new("Your command?", options).prompt();
        let option;
        match ans {
            Ok(choice) => {
                option = choice;
            }
            Err(_) => option = Operations::Exit,
        }

        match option {
            Operations::Shorten => {
                let name = Text::new("Insert the URL to shorten").prompt();
                match name {
                    Ok(name) => {
                        let key = shortener.shorten(name.as_str());
                        if let Some(db) = &database {
                            let _ = db.insert((key, name));
                        }
                    }
                    Err(_) => {
                        //Do nothing
                    }
                }
            }
            Operations::Retrieve => {
                let name = Text::new("Insert the key to retrieve").prompt();
                let key: String;
                match name {
                    Ok(opt) => {
                        key = opt;
                    }
                    Err(_) => {
                        break;
                    }
                }
                let value = shortener.get(key.as_str());
                if value.is_none() {
                    println!("Chave não existente");
                    continue;
                }
                println!("{}", value.unwrap());
            }
            Operations::List => {
                println!("{:?}", shortener.list_values());
            }
            Operations::ListKeys => {
                println!("{:?}", shortener.list_keys());
            }
            Operations::Exit => break,
        }
    }
}

fn main() {
    let config = Config::from_config_file("./config.toml").unwrap();
    let shortener: Box<dyn UrlShortener>;

    let initial: HashMap<String, String>;
    let mut database: Option<Database> = None;

    if config.persistent {
        let conn = Connection::open("sqls.db").unwrap();
        let mut temp: HashMap<String, String> = HashMap::new();
        let db = Database::new(conn);
        db.select_all().unwrap().into_iter().for_each(|f| {
            temp.insert(f.0, f.1);
        });
        database = Option::Some(db);
        initial = temp;
    } else {
        initial = HashMap::new();
    }

    if "counter".eq_ignore_ascii_case(&config.shortenen_algorithm) {
        shortener = Box::new(CounterShortener::new(initial));
    } else {
        shortener = Box::new(HashShortener::new(initial));
    }

    if config.mode == Mode::Interactive {
        interactive_mode(shortener, database);
    } else {
        batch_mode(shortener, config, database);
    }
}
