use arrow_array::{RecordBatch, RecordBatchIterator};
use arrow_schema::{DataType, Field, Schema};
use sbert::{Embeddings, Error, SBertHF};
use std::sync::Arc;

pub async fn data(embeds: Vec<Embeddings>) {
    let db = lancedb::connect("../db/").execute().await.unwrap();
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new(
            "vector",
            DataType::FixedSizeList(Arc::new(Field::new("item", DataType::Float32, true)), 128),
            true,
        ),
    ]));
}
