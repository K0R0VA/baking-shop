use async_graphql::{Context, SimpleObject, Object};

#[derive(sqlx::FromRow)]
pub struct Product {
    id: i32
}

#[Object]
impl Product {
    async fn id(&self) -> i32 {
        self.id
    }
    async fn name(&self, ctx: &Context<'_>) -> anyhow::Result<String> {
        Ok("".to_string())
    }
    async fn pic(&self, ctx: &Context<'_>) -> anyhow::Result<String> {
        Ok("".to_string())
    }
    async fn r#type(&self, ctx: &Context<'_>) -> anyhow::Result<ProductType> {
        Ok(ProductType {name: "".to_string(), id: 0})
    }
}

#[derive(SimpleObject)]
pub struct ProductType {
    pub id: i32,
    pub name: String
}

