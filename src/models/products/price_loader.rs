use std::collections::HashMap;
use std::sync::Arc;
use async_graphql::dataloader::Loader;
use sqlx::{PgPool, query_as, FromRow};
use serde::Deserialize;

pub struct PriceLoader {
    pub(crate) pool: Arc<PgPool>
}

#[async_trait::async_trait]
impl Loader<i32> for PriceLoader {
    type Value = Option<f32>;
    type Error = String;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        use futures::stream::StreamExt;
        let mut database = self.pool.acquire().await.map_err(|e| format!("{e}"))?;
        let geos = query_as::<_, Price>("select id as product_id, price from products")
            .bind(keys)
            .fetch_many(&mut database)
            .filter_map(|r| futures::future::ready(r.ok().and_then(|either| either.right())))
            .map(|Price { price, product_id}| (product_id, price))
            .collect()
            .await;
        Ok(geos)
    }
}

#[derive(Deserialize, FromRow)]
struct Price {
    price: Option<f32>,
    product_id: i32
}