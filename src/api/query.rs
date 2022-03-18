use actix_web::web::Data;
use async_graphql::{Context, Object};
use sqlx::{PgPool, query_as};
use crate::models::Product;

pub struct Query;

#[Object]
impl Query {
    async fn products(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Product>> {
        let mut pool = ctx.data_unchecked::<Data<PgPool>>().acquire().await?;
        let products = query_as("select id from products").fetch_all(&mut pool).await?;
        Ok(products)
    }
}