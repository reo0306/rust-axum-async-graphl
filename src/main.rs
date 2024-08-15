use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json,
    Router,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation,
    EmptySubscription,
    Request,
    Response,
    Schema,
};
use std::{net::SocketAddr, sync::Arc};
//use resolvers::QueryRoot;
use tokio::net::TcpListener;

use async_graphql::{
    Object,
    SimpleObject,
};

pub struct QueryRoot;

#[derive(SimpleObject)]
struct Ping {
    status: String,
    code: i32,
}

#[Object]
impl QueryRoot {
  async fn ping(&self) -> Ping {
    Ping { 
      status: "ok".to_string(), 
      code: 200 
    }
  }
}

pub type BlogSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<BlogSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn not_found_handler() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, "not found".to_string())
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish(); 

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(Arc::new(schema)))
        .fallback(not_found_handler);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = TcpListener::bind(addr)
    .await
    .unwrap_or_else(|e| panic!("failed to listen on {addr}: {e}"));

    axum::serve(listener, app)
    .await
    .unwrap_or_else(|e| panic!("failed to run `auxm::serve`: {e}"));
}
