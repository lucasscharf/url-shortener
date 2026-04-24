mod url_shortener;
use config_file::FromConfigFile;
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
}

#[derive(Deserialize, Debug, PartialEq)]
enum Mode {
    Interactive,
    File,
}

#[derive(Debug, PartialEq, EnumString, Display, EnumIter, Clone)]
enum Operations {
    Add,
    Get,
    List,
    #[strum(to_string = "List Keys")]
    ListKeys,
    Exit,
}

fn interactive(mut shortener: Box<dyn UrlShortener>) {
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
            Operations::Add => {
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
            Operations::Get => {
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
        interactive(shortener);
    } else {
    }
}
