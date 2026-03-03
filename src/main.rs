pub mod db;
pub mod embedding;
pub mod generate;

fn main() {
    let v = vec!["test", "test2"];
    let vs: Vec<String> = v.iter().map(|s| s.to_string()).collect();
    let embeds = embedding::embed(&vs);
    let _ = db::data(embeds.unwrap());
}
