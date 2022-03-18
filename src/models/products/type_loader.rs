use std::collections::HashMap;
use std::sync::Arc;
use async_graphql::dataloader::Loader;
use sqlx::{PgPool, query_as, FromRow};
use serde::Deserialize;
use crate::models::products::ProductType;

pub struct TypeLoader {
    pub(crate) pool: Arc<PgPool>,
}

#[async_trait::async_trait]
impl Loader<i32> for TypeLoader {
    type Value = ProductType;
    type Error = String;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        use futures::stream::StreamExt;
        let mut database = self.pool.acquire().await.map_err(|e| format!("{e}"))?;
        let product_types = query_as::<_, Type>(
            "select products.id as product_id, type as type_id, pt.name as type_name from products \
                join product_types pt on products.type = pt.id ")
            .bind(keys)
            .fetch_many(&mut database)
            .filter_map(|r| futures::future::ready(r.ok().and_then(|either| either.right())))
            .map(|Type { type_id, type_name, product_id }| (product_id, ProductType { id: type_id, name: type_name }))
            .collect::<HashMap<i32, ProductType>>()
            .await;
        Ok(product_types)
    }
}

#[derive(Deserialize, FromRow)]
struct Type {
    type_id: i32,
    type_name: String,
    product_id: i32,
}