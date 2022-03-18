use async_graphql::{Context, Error, Guard};
use crate::models::LoggedUser;

pub struct AuthGuard;

#[async_trait::async_trait]
impl Guard for AuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        let user = ctx.data_unchecked::<LoggedUser>();
        if user.is_none() {
            return Err(Error::new("not authorized"));
        }
        Ok(())
    }
}