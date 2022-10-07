use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


#[derive(Deserialize, Debug)]
struct Config {
    db_name: String,
    db_user: String,
    db_password: String,
}

fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let conf = serde_json::from_reader(reader)?;
    Ok(conf)
}

fn main() {
    let config = read_config_from_file("config.json").unwrap();
    println!("{:#?}", config);
}
