pub mod db;
pub mod embedding;
pub mod generate;
use anyhow::Result;
use tokio::task;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let v = vec!["test", "test2"];
    let vs: Vec<String> = v.iter().map(|s| s.to_string()).collect();
    println!("sucess");
    let embeds = embedding::embd(&vs);
    db::data(embeds.unwrap()).await;
    Ok(())
}
