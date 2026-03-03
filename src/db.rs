use arrow_array::{RecordBatch,RecordBatchIterator}
use arrow_schema::{DataType, Field, Schema}
use sbert::{Embeddings, Error, SBertHF};

pub async fn data(embeds: Vec::<Embeddings>){
    let db = lancedb::connect("../db/").execute().await.unwrap();
}
