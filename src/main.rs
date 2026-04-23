mod url_shortener;
use inquire::{InquireError, Select, Text};
use url_shortener::UrlShortener;
use url_shortener::counter::CounterShortener;
use url_shortener::hash::HashShortener;

fn main() {
    let mut counter = CounterShortener::default();
    let mut hasher = HashShortener::default();

    println!(
        "===================================================================================="
    );

    loop {
        let options: Vec<&str> = vec!["Add", "Get", "List", "List Keys", "Exit"];

        let ans: Result<&str, InquireError> =
            Select::new("Your command?", options).prompt();
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
                        hasher.shorten(name.as_str());
                        counter.shorten(name.as_str());
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
                let mut value = counter.get(key.as_str());
                if value.is_none() {
                    value = hasher.get(key.as_str());
                }
                if value.is_none() {
                  println!("Chave não existente");
                  continue;
                }
                println!("{}", value.unwrap());
            }
            "List" => {
                println!("{:?}", counter.list_values());
                println!("{:?}", hasher.list_values());
            }
            "List Keys" => {
                println!("{:?}", counter.list_keys());
                println!("{:?}", hasher.list_keys());
            }
            "Exit" => break,
            _ => unreachable!(),
        }
    }

    println!("{hasher:?}");
    println!("{counter:?}");
}
