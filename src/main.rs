mod url_shortener;
use config_file::FromConfigFile;
use inquire::{InquireError, Select, Text};
use serde::Deserialize;
use url_shortener::UrlShortener;
use url_shortener::counter::CounterShortener;
use url_shortener::hash::HashShortener;

#[derive(Deserialize, Debug)]
struct Config {
    shortenen_algorithm: String,
}

fn main() {
    let config = Config::from_config_file("./config.toml").unwrap();
    let mut shortener: Box<dyn UrlShortener>;

    if "counter".eq_ignore_ascii_case(&config.shortenen_algorithm) {
      shortener = Box::new(CounterShortener::default());
    } else {
      shortener = Box::new(HashShortener::default());
    }

    println!(
        "===================================================================================="
    );

    loop {
        let options: Vec<&str> = vec!["Add", "Get", "List", "List Keys", "Exit"];

        let ans: Result<&str, InquireError> = Select::new("Your command?", options).prompt();
        let option;
        match ans {
            Ok(choice) => {
                option = choice;
            }
            Err(_) => option = "Exit",
        }

        match option {
            "Add" => {
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
            "Get" => {
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
            "List" => {
                println!("{:?}", shortener.list_values());
            }
            "List Keys" => {
                println!("{:?}", shortener.list_keys());
            }
            "Exit" => break,
            _ => unreachable!(),
        }
    }

}
