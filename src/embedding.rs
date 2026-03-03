use sbert::{Embeddings, Error, SBertHF};
use std::{env, path::PathBuf};

pub fn embed(s: &[String]) -> Result<Vec<Embeddings>, Error> {
    let mut home: PathBuf = env::current_dir().unwrap();
    home.push("../model/nomic");

    let sbert_model = SBertHF::new(home.to_str().unwrap());
    let batch_size = 64;
    sbert_model?.forward(s, batch_size)
}
