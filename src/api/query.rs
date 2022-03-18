use async_graphql::{Context, Object};
pub struct Query;

#[Object]
impl Query {
    async fn products(&self, ctx: &Context<'_>) -> Vec<i32> {
        vec![]
    }
}