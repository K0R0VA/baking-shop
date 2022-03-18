use actix_web::web::Data;
use async_graphql::{Context, Object};
use sqlx::{PgPool, query, query_as};
use crate::identity::GraphqlIdentity;
use crate::jwt::create_jwt;
use crate::models::{ChangeUserPassword, Credentials, CurrentUser};
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
}