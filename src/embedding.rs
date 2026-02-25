use sbert::SBertHF;
use std::path::PathBuf;

fn embed(s: &[String]) -> Result<Vec<Embeddings, Error>> {
    let mut home: PathBuf = env::current_dir().unwrap();
    home.push("path-to-model");

    let sbert_model = SBertHF::new(home.to_str().unwrap());
    let batch_size = 64;
    sbert_model.forward(texts.to_vec(), batch_size)
}
