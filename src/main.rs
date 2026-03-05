pub mod db;
pub mod embedding;
pub mod generate;
use anyhow::Result;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let v = vec!["test", "test2"];
    //convert the vec! to type Vec<String> and pass as reference to the embd funciton
    let embeds = embedding::embd(&v.iter().map(|s| s.to_string()).collect::<Vec<String>>());
    //pass embedings returned from model to the database
    db::data(embeds.unwrap()).await;
    Ok(())
}
