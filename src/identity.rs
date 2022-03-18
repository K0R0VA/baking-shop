use std::ops::Deref;
use actix_identity::Identity;

pub struct GraphqlIdentity(Identity);

impl From<Identity> for GraphqlIdentity {
    fn from(i: Identity) -> Self {
        Self (i)
    }
}

unsafe impl Sync for GraphqlIdentity {}
unsafe impl Send for GraphqlIdentity {}

impl Deref for GraphqlIdentity {
    type Target = Identity;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
