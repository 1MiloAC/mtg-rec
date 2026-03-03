use arrow_array::{
    FixedSizeListArray, Int32Array, RecordBatch, RecordBatchIterator, types::Float32Type,
};
use arrow_schema::{DataType, Field, Schema};
use sbert::Embeddings;
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
    let batches = RecordBatchIterator::new(
        vec![
            RecordBatch::try_new(
                schema.clone(),
                vec![
                    Arc::new(Int32Array::from_iter_values(0..256)),
                    Arc::new(
                        FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
                            (0..256).map(|_| Some(vec![Some(1.0); 128])),
                            128,
                        ),
                    ),
                ],
            )
            .unwrap(),
        ]
        .into_iter()
        .map(Ok),
        schema.clone(),
    );
    db.create_table("my_table", Box::new(batches))
        .execute()
        .await
        .unwrap();
}
