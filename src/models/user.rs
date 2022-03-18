use std::future::{Ready, ready};
use actix_identity::Identity;
use actix_web::{FromRequest, HttpMessage};
use crate::jwt::decode_identity;

pub struct User {
    id: i32
}

#[async_graphql::Object]
impl User {
    async fn id(&self) -> i32 {
        self.id
    }
}

#[derive(async_graphql::Enum, Eq, PartialEq, Copy, Clone, serde::Serialize, serde::Deserialize, sqlx::Type, Debug)]
#[repr(i32)]
pub enum Role {
    Consumer = 1,
    Manager = 2,
    Admin = 3,
}

impl CurrentUser {
    pub(crate) const fn is_admin(&self) -> bool {
        (self.role as i32) == 3
    }
}

#[derive(async_graphql::SimpleObject, sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug)]
pub struct CurrentUser {
    pub id: i32,
    pub role: Role,
    pub email: String
}
#[derive(Debug)]
pub struct LoggedUser (Option<CurrentUser>);

impl LoggedUser {
    pub fn is_admin(&self) -> Option<bool> {
        self.0.as_ref().map(|user| user.is_admin())
    }
    pub fn borrow_user(&self) -> Option<&'static CurrentUser> {
        self.0.as_ref().and_then(|user| {
            let ptr: *const CurrentUser = user;
            unsafe { ptr.as_ref() }
        })
    }
}

impl Deref for LoggedUser {
    type Target = Option<CurrentUser>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Option<String>> for LoggedUser {
    fn from(identity: Option<String>) -> Self {
        let user = identity
            .and_then(|i| decode_identity(i).ok());
        Self (user)
    }
}

use actix_web::{error::ResponseError, HttpResponse};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug)]
pub enum ServiceError {}

impl Display for ServiceError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!();
    }
}
// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        unimplemented!();
    }
}

#[derive(async_graphql::InputObject)]
pub struct Credentials {
    pub email: String,
    pub password: String
}

#[derive(async_graphql::InputObject)]
pub struct ChangeUserPassword {
    pub user_id: i32,
    pub password: String
}