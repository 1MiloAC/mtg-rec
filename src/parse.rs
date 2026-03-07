use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::{env, fs::File, io::BufReader, path::Path, path::PathBuf};

#[derive(Deserialize, Debug)]
pub struct Package {
    id: String,
    package: String,
}
pub fn parse() -> Result<Package, Box<dyn Error>> {
    let mut path: PathBuf = env::current_dir().unwrap();
    path.push("../mtg.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let p = serde_json::from_reader(reader)?;
    Ok(p)
}
