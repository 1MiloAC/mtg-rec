use arrow_array::{FixedSizeListArray, RecordBatch, RecordBatchIterator, types::Float32Type};
use arrow_schema::{DataType, Field, Schema};
use fastembed::Embedding;
use std::sync::Arc;

pub async fn data(embeds: Vec<Embedding>) {
    let db = lancedb::connect("./db").execute().await.unwrap();
    // schema of that takes a vector with a field that is a list floats
    // along with the number of dimensions of the vector data
    let schema = Arc::new(Schema::new(vec![Field::new(
        "embedding",
        DataType::FixedSizeList(Arc::new(Field::new("item", DataType::Float32, true)), 768),
        true,
    )]));
    let data = FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
        // converts the embeddings to a list of Some(v(Some))
        embeds.iter().map(|s| {
            let v = s.to_vec();
            Some(v.into_iter().map(Some))
        }),
        768,
    );
    println!("ingest successful");
    // loads the data and the schema into a batch
    let batch = RecordBatch::try_new(schema.clone(), vec![Arc::new(data)]).unwrap();
    // loads x amount of batches and pushes them into the db
    let batch_i = RecordBatchIterator::new(vec![batch].into_iter().map(Ok), schema.clone());
    db.create_table("test", Box::new(batch_i))
        .execute()
        .await
        .unwrap();
}
