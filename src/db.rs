use arrow_array::{
    FixedSizeListArray, Int32Array, ListArray, RecordBatch, RecordBatchIterator, types::Float32Type,
};
use arrow_schema::{DataType, Field, Schema};
use sbert::Embeddings;
use std::sync::Arc;

pub async fn data(embeds: Vec<Embeddings>) {
    let db = lancedb::connect("../db/").execute().await.unwrap();
    let schema = Arc::new(Schema::new(vec![Field::new(
        "embedding",
        DataType::FixedSizeList(Arc::new(Field::new("item", DataType::Float32, false)), 128),
        true,
    )]));
    let data = FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
        embeds.iter().map(|s| {
            let v = s.to_vec();
            Some(v.into_iter().map(Some))
        }),
        128,
    );
    let batch = RecordBatch::try_new(schema.clone(), vec![Arc::new(data)]).unwrap();
    let batch_i = RecordBatchIterator::new(vec![batch].into_iter().map(Ok), schema.clone());
    db.create_table("test", Box::new(batch_i))
        .execute()
        .await
        .unwrap();
}
