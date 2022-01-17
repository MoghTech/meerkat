//! Retrieves multiple documents from a collection by their id field.
//!
//! Retrieves documents from collection if they match the ids specified in the input.
//! Returns a Vec of the type that the collection is assigned to.
//!  
//! # Examples
//!
//! ```
//!     use mungos::{Mungos}
//!     use serde::{Serialize, Deserialize}
//!     
//!     #[derive(Debug, Serialize, Deserialize)]
//!     struct Item {  
//!         field: String
//!     }
//!
//!     let db = Mungos::new("uri", "app name", timeout).await;
//!     let collection = db.connection::<Item>("db name", "collection name");
//!
//!     let ids = Vec::from(["...", "...", "..."]);
//!     collection.get_many_by_id(ids).await.unwrap();
//!
//! ```
//!

use crate::Collection;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Result,
};
use serde::de::DeserializeOwned;
use std::str::FromStr;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn get_many_by_id(&self, ids: &Vec<String>) -> Result<Vec<T>> {
        let ids: Vec<ObjectId> = ids
            .iter()
            .map(|id| ObjectId::from_str(id).unwrap())
            .collect();
        let mut cursor = self
            .collection
            .find(
                doc! {
                    "_id": {
                        "$in": ids
                    }
                },
                None,
            )
            .await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        Ok(items)
    }
}
