pub struct User {
    id: i32
}

#[async_graphql::Object]
impl User {
    async fn id(&self) -> i32 {
        self.id
    }
}

#[derive(async_graphql::Enum, Eq, PartialEq, Copy, Clone, serde::Serialize, sqlx::Type)]
#[repr(i32)]
pub enum Role {
    Consumer = 1,
    Manager = 2,
    Admin = 3,
}

#[derive(async_graphql::SimpleObject, sqlx::FromRow, serde::Serialize)]
pub struct CurrentUser {
    pub id: i32,
    pub role: Role,
    pub email: String
}

#[derive(async_graphql::InputObject)]
pub struct Credentials {
    pub email: String,
    pub password: String
}
