use fastembed::{Embedding, EmbeddingModel, Error, InitOptions, TextEmbedding};
use std::{env, path::PathBuf};

pub fn embd(data: &[String]) -> Result<Vec<Embedding>, Error> {
    let mut model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::NomicEmbedTextV15).with_show_download_progress(true),
    )?;
    model.embed(data, None)
}
