use actix_web::web::Data;
use async_graphql::{Context, Object};
use sqlx::{PgPool, query, query_as, query_scalar};
use crate::identity::GraphqlIdentity;
use crate::jwt::create_jwt;
use crate::models::{ChangeUserPassword, Credentials, CurrentUser, EditProduct, EditProductType, NewProduct, Product, ProductType};
use crate::guards::{AdminGuard, AuthGuard};

pub struct Mutation;

#[Object]
impl Mutation {
    async fn sign_in(&self, ctx: &Context<'_>, credentials: Credentials) -> anyhow::Result<CurrentUser> {
        let mut pool = ctx.data_unchecked::<Data<PgPool>>().acquire().await?;
        let identity = ctx.data_unchecked::<GraphqlIdentity>();
        let user = query_as::<_, CurrentUser>("select id, email, role from users where email = $1 and password = $2")
            .bind(credentials.email)
            .bind(credentials.password)
            .fetch_optional(&mut pool)
            .await?
            .ok_or_else(|| anyhow::anyhow!("not found"))?;
        let token = create_jwt(&user)?;
        identity.remember(token);
        Ok(user)
    }
    #[graphql(guard = "AuthGuard")]
    async fn change_current_user_password(&self, ctx: &Context<'_>, password: String) -> anyhow::Result<&str> {
        let current_user = ctx.data_unchecked::<&CurrentUser>();
        let mut pool = ctx.data_unchecked::<Data<PgPool>>().acquire().await?;
        query("update users set password = $1 where id = $2")
            .bind(password)
            .bind(current_user.id)
            .execute(&mut pool)
            .await?;
        Ok("Password has been successfully changed")
    }
    #[graphql(guard = "AdminGuard")]
    async fn change_user_password(&self, ctx: &Context<'_>, credentials: ChangeUserPassword) -> anyhow::Result<&str> {
        let mut pool = ctx.data_unchecked::<Data<PgPool>>().acquire().await?;
        query("update users set password = $1 where id = $2")
            .bind(credentials.password)
            .bind(credentials.user_id)
            .execute(&mut pool)
            .await?;
        Ok("Password has been successfully changed")
    }
    #[graphql(guard = "AdminGuard")]
    async fn add_product(&self, ctx: &Context<'_>, product: NewProduct) -> anyhow::Result<Product> {
        let mut pool = ctx.data_unchecked::<Data<PgPool>>().acquire().await?;
        let id = query_scalar::<_, i32>(
            "insert into products (name, pic, price, type, weight, description) values ($1, $2, $3, $4, $5, $6)\
                 returning id")
            .bind(product.name)
            .bind(product.pic)
            .bind(product.price)
            .bind(product.r#type)
            .bind(product.weight)
            .bind(product.description)
            .fetch_one(&mut pool)
            .await?;
        Ok(Product { id })
    }
    #[graphql(guard = "AdminGuard")]
    async fn edit_product(&self, ctx: &Context<'_>, product: EditProduct) -> anyhow::Result<Option<Product>> {
        let mut pool = ctx.data_unchecked::<Data<PgPool>>().acquire().await?;
        let r = query_scalar::<_, i32>(
            "update products set name = $1, pic = $2, price = $3, type = $4, weight= $5, description= $6 where id = $7
                 returning id")
            .bind(product.name)
            .bind(product.pic)
            .bind(product.price)
            .bind(product.r#type)
            .bind(product.weight)
            .bind(product.description)
            .bind(product.id)
            .fetch_optional(&mut pool)
            .await?;
        Ok(r.map(|id| Product {id}))
    }
    #[graphql(guard = "AdminGuard")]
    async fn delete_product(&self, ctx: &Context<'_>, product_id: i32) -> anyhow::Result<&str> {
        let mut pool = ctx.data_unchecked::<Data<PgPool>>().acquire().await?;
        query("delete products where id = $1")
            .bind(product_id)
            .execute(&mut pool)
            .await?;
        Ok("Product has been successfully deleted")
    }
    #[graphql(guard = "AdminGuard")]
    async fn add_product_type(&self, ctx: &Context<'_>, name: String) -> anyhow::Result<ProductType> {
        let mut pool = ctx.data_unchecked::<Data<PgPool>>().acquire().await?;
        let r#type = query_as::<_, ProductType>("insert into product_types (name) values ($1) returning id, name")
            .bind(name)
            .fetch_one(&mut pool)
            .await?;
        Ok(r#type)
    }
    #[graphql(guard = "AdminGuard")]
    async fn delete_product_type(&self, ctx: &Context<'_>, type_id: i32) -> anyhow::Result<&str> {
        let mut pool = ctx.data_unchecked::<Data<PgPool>>().acquire().await?;
        query("delete product_types where id = $1")
            .bind(type_id)
            .execute(&mut pool)
            .await?;
        Ok("Product type has been successfully deleted")
    }
    #[graphql(guard = "AdminGuard")]
    async fn edit_product_type(&self, ctx: &Context<'_>, r#type: EditProductType) -> anyhow::Result<ProductType> {
        let mut pool = ctx.data_unchecked::<Data<PgPool>>().acquire().await?;
        let r#type = query_as::<_, ProductType>("")
            .bind(r#type.name)
            .bind(r#type.id)
            .fetch_one(&mut pool)
            .await?;
        Ok(r#type)
    }
}