use async_graphql::{Context, Error, Guard};
use crate::models::{LoggedUser};

pub struct AdminGuard;

#[async_trait::async_trait]
impl Guard for AdminGuard {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        let user = ctx.data_unchecked::<LoggedUser>();
        if let Some(is_admin) = user.is_admin() {
            if !is_admin {
                Err(Error::new("allowed only for admins"))
            } else {
                Ok(())
            }
        } else {
            Err(Error::new("not authorized"))
        }
    }
}