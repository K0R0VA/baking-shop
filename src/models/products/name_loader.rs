use std::collections::HashMap;
use std::sync::Arc;
use async_graphql::dataloader::Loader;
use sqlx::{PgPool, query_as, FromRow};
use serde::Deserialize;

pub struct NameLoader {
    pub(crate) pool: Arc<PgPool>
}

#[async_trait::async_trait]
impl Loader<i32> for NameLoader {
    type Value = String;
    type Error = String;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        use futures::stream::StreamExt;
        let mut database = self.pool.acquire().await.map_err(|e| format!("{e}"))?;
        let geos = query_as::<_, Name>("select id as product_id, name from products")
            .bind(keys)
            .fetch_many(&mut database)
            .filter_map(|r| futures::future::ready(r.ok().and_then(|either| either.right())))
            .map(|Name {name, product_id}| (product_id, name))
            .collect::<HashMap<i32, String>>()
            .await;
        Ok(geos)
    }
}

#[derive(Deserialize, FromRow)]
struct Name {
    name: String,
    product_id: i32
}