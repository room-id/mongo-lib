use crate::bson::doc;
use anyhow::Result;
use futures::TryStreamExt;
use mongodb::{
    Client, Collection,
    bson::{self, Document},
    options::ClientOptions,
};
use serde_json::Value;

pub struct MongoLib {
    collection: Collection<Document>,
}

impl MongoLib {
    pub async fn new(args: (&str, &str, &str)) -> Result<Self> {
        let (connection_string, db_name, collection_name) = args;
        let options = ClientOptions::parse(connection_string).await?;
        let client = Client::with_options(options)?;
        let db = client.database(db_name);
        let collection = db.collection::<Document>(collection_name);

        Ok(Self { collection })
    }

    pub async fn insert_json(&self, data: Value) -> Result<()> {
        match data {
            Value::Array(items) => {
                let docs: Vec<Document> = items
                    .into_iter()
                    .map(|v| bson::to_document(&v))
                    .collect::<std::result::Result<_, _>>()?;
                self.collection.insert_many(docs).await?;
            }
            _ => {
                let doc = bson::to_document(&data)?;
                self.collection.insert_one(doc).await?;
            }
        }
        Ok(())
    }

    pub async fn query_json(&self, query: Value) -> Result<Value> {
        let filter = bson::to_document(&query)?;
        let mut cursor = self.collection.find(filter).await?;
        let mut results = vec![];
        while let Some(doc) = cursor.try_next().await? {
            results.push(bson::from_document::<Value>(doc)?);
        }
        Ok(Value::Array(results))
    }

    pub async fn update_one_json(&self, query: Value, update: Value, upsert: bool) -> Result<u64> {
        let filter = bson::to_document(&query)?;
        let mut update_doc = bson::to_document(&update)?;

        if upsert {
            update_doc = doc! { "$set": update_doc, "$setOnInsert": filter.clone() };
        }

        let result = self.collection.update_one(filter, update_doc).await?;
        Ok(result.modified_count)
    }

    pub async fn update_many_json(&self, query: Value, update: Value, upsert: bool) -> Result<u64> {
        let filter = bson::to_document(&query)?;
        let mut update_doc = bson::to_document(&update)?;

        if upsert {
            update_doc = doc! { "$set": update_doc, "$setOnInsert": filter.clone() };
        }

        let result = self.collection.update_many(filter, update_doc).await?;
        Ok(result.modified_count)
    }

    pub async fn delete_one_json(&self, query: Value) -> Result<u64> {
        let filter = bson::to_document(&query)?;
        let result = self.collection.delete_one(filter).await?;
        Ok(result.deleted_count)
    }

    pub async fn delete_many_json(&self, query: Value) -> Result<u64> {
        let filter = bson::to_document(&query)?;
        let result = self.collection.delete_many(filter).await?;
        Ok(result.deleted_count)
    }

    pub async fn aggregate_json(&self, pipeline: Value) -> Result<Value> {
        let stages = match pipeline {
            Value::Array(arr) => arr,
            _ => anyhow::bail!("aggregate_json expects a JSON array pipeline"),
        };

        let pipeline_docs: Vec<Document> = stages
            .into_iter()
            .map(|v| bson::to_document(&v))
            .collect::<std::result::Result<_, _>>()?;

        let mut cursor = self.collection.aggregate(pipeline_docs).await?;
        let mut results = vec![];

        while let Some(doc) = cursor.try_next().await? {
            results.push(bson::from_document::<Value>(doc)?);
        }

        Ok(Value::Array(results))
    }

    pub async fn count_json(&self, query: Value) -> Result<u64> {
        let filter = bson::to_document(&query)?;
        let count = self.collection.count_documents(filter).await?;
        Ok(count)
    }
}