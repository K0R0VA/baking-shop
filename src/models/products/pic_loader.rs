use std::collections::HashMap;
use std::sync::Arc;
use async_graphql::dataloader::Loader;
use sqlx::{PgPool, query_as, FromRow};
use serde::Deserialize;


pub struct PicLoader {
    pub(crate) pool: Arc<PgPool>
}

#[async_trait::async_trait]
impl Loader<i32> for PicLoader {
    type Value = String;
    type Error = String;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        use futures::stream::StreamExt;
        let mut database = self.pool.acquire().await.map_err(|e| format!("{e}"))?;
        let geos = query_as::<_, Pic>("select id as product_id, pic from products")
            .bind(keys)
            .fetch_many(&mut database)
            .filter_map(|r| futures::future::ready(r.ok().and_then(|either| either.right())))
            .map(|Pic {picture, product_id}| (product_id, picture))
            .collect::<HashMap<i32, String>>()
            .await;
        Ok(geos)
    }
}

#[derive(Deserialize, FromRow)]
struct Pic {
    picture: String,
    product_id: i32
}