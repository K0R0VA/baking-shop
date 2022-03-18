use actix_web::web::Data;
use async_graphql::{Context, Object};
use sqlx::{PgPool, query_as};
use crate::identity::GraphqlIdentity;
use crate::jwt::create_jwt;
use crate::models::{Credentials, CurrentUser};

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
}