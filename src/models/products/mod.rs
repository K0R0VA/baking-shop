mod name_loader;
mod pic_loader;
mod type_loader;
mod price_loader;
mod weight_loader;

use anyhow::anyhow;
use async_graphql::{Context, SimpleObject, Object, InputObject};
use async_graphql::dataloader::DataLoader;
use sqlx::FromRow;
pub use crate::models::products::name_loader::NameLoader;
pub use crate::models::products::price_loader::PriceLoader;
pub use crate::models::products::type_loader::TypeLoader;
pub use crate::models::products::weight_loader::WeightLoader;
pub use crate::models::products::pic_loader::PicLoader;

#[derive(sqlx::FromRow)]
pub struct Product {
    pub(crate) id: i32,
}

#[Object]
impl Product {
    async fn id(&self) -> i32 {
        self.id
    }
    async fn name(&self, ctx: &Context<'_>) -> anyhow::Result<String> {
        let dataloader = ctx.data_unchecked::<DataLoader<NameLoader>>();
        let name = dataloader.load_one(self.id).await
            .map_err(|e| anyhow!(e))?
            .unwrap_or_default();
        Ok(name)
    }
    async fn pic(&self, ctx: &Context<'_>) -> anyhow::Result<String> {
        let dataloader = ctx.data_unchecked::<DataLoader<PicLoader>>();
        let name = dataloader.load_one(self.id).await
            .map_err(|e| anyhow!(e))?
            .unwrap_or_default();
        Ok(name)
    }
    async fn r#type(&self, ctx: &Context<'_>) -> anyhow::Result<ProductType> {
        let dataloader = ctx.data_unchecked::<DataLoader<TypeLoader>>();
        let r#type = dataloader.load_one(self.id).await
            .map_err(|e| anyhow!(e))?
            .ok_or_else(|| anyhow!("product type not found"))?;
        Ok(r#type)
    }
    async fn price(&self, ctx: &Context<'_>) -> anyhow::Result<Option<f32>> {
        let dataloader = ctx.data_unchecked::<DataLoader<PriceLoader>>();
        let price = dataloader.load_one(self.id).await
            .map_err(|e| anyhow!(e))?
            .flatten();
        Ok(price)
    }
    async fn weight(&self, ctx: &Context<'_>) -> anyhow::Result<Option<f32>> {
        let dataloader = ctx.data_unchecked::<DataLoader<WeightLoader>>();
        let weight = dataloader.load_one(self.id).await
            .map_err(|e| anyhow!(e))?
            .unwrap_or_default();
        Ok(weight)
    }
}

#[derive(SimpleObject, Clone, FromRow)]
pub struct ProductType {
    pub id: i32,
    pub name: String,
}

#[derive(InputObject)]
pub struct EditProductType {
    pub id: i32,
    pub name: String,
}

#[derive(InputObject)]
pub struct NewProduct {
    pub name: String,
    pub pic: String,
    pub r#type: i32,
    pub price: Option<f32>,
    pub weight: Option<f32>,
    pub description: Option<String>
}

#[derive(InputObject)]
pub struct EditProduct {
    pub id: i32,
    pub name: String,
    pub pic: String,
    pub r#type: i32,
    pub price: Option<f32>,
    pub weight: Option<f32>,
    pub description: Option<String>
}

