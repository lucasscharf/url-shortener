mod url_shortener;
use config_file::FromConfigFile;
use csv::Reader;
use inquire::{InquireError, Select, Text};
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
    file_path: String,
}

#[derive(Deserialize, Debug, PartialEq)]
enum Mode {
    Interactive,
    File,
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

fn file_mode(mut shortener: Box<dyn UrlShortener>, config: Config) {
    let file_path = config.file_path;
    let mut rdr = Reader::from_path(file_path).unwrap();

    for result in rdr.deserialize::<Operation>() {
        let record = result.unwrap();

        match record.operation {
            Operations::Shorten => {
                shortener.shorten(record.url.as_str());
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

fn interactive_mode(mut shortener: Box<dyn UrlShortener>) {
    println!(
        "===================================================================================="
    );

    loop {
        let options: Vec<Operations> = Operations::iter() //
            .collect();

        let ans: Result<Operations, InquireError> = Select::new("Your command?", options).prompt();
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
                        shortener.shorten(name.as_str());
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

    if "counter".eq_ignore_ascii_case(&config.shortenen_algorithm) {
        shortener = Box::new(CounterShortener::default());
    } else {
        shortener = Box::new(HashShortener::default());
    }

    if config.mode == Mode::Interactive {
        interactive_mode(shortener);
    } else {
        file_mode(shortener, config);
    }
}
