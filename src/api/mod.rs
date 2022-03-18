mod query;
mod mutation;

use std::sync::Arc;
use actix_identity::Identity;
use actix_web::{get, HttpResponse, post, web::{self, ServiceConfig}};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql::{EmptySubscription, Schema, SchemaBuilder};
use async_graphql::dataloader::DataLoader;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::PgPool;

use crate::api::mutation::Mutation;
use crate::api::query::Query;
use crate::identity::GraphqlIdentity;
use crate::models::{LoggedUser, NameLoader, PicLoader, PriceLoader, TypeLoader, WeightLoader};

#[get("/")]
async fn playground() -> HttpResponse {
    let html = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("/")]
async fn graphql(
    pool: web::Data<PgPool>,
    identity: Identity,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let logged_user: LoggedUser = identity.identity().into();
    let schema = create_schema()
        .data(GraphqlIdentity::from(identity))
        .data(DataLoader::new(NameLoader {pool: Arc::clone(&pool) }, tokio::spawn))
        .data(DataLoader::new(PriceLoader {pool: Arc::clone(&pool) }, tokio::spawn))
        .data(DataLoader::new(PicLoader {pool: Arc::clone(&pool) }, tokio::spawn))
        .data(DataLoader::new(TypeLoader {pool: Arc::clone(&pool) }, tokio::spawn))
        .data(DataLoader::new(WeightLoader {pool: Arc::clone(&pool) }, tokio::spawn))
        .data(pool);
    let user = logged_user.borrow_user();
    let schema = {
        if let Some(user) = user {
            schema.data(user).data(logged_user)
        } else {
            schema.data(logged_user)
        }
    };
    schema.finish().execute(req.into_inner()).await.into()
}

pub fn api_config(config: &mut ServiceConfig) {
    config
        .service(graphql)
        .service(playground);
}

fn create_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query {}, Mutation {}, EmptySubscription::default())
}
