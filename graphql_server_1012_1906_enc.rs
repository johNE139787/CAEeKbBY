use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use async_graphql::{http::{playground_source, GraphQLRequest, GraphQLResponse}, EmptyMutation, EmptySubscription, Schema, Object, SimpleObject};
use async_graphql::Context;
use rocket_graphql;

#[macro_use] extern crate rocket;

#[derive(SimpleObject)]
struct QueryRoot;

#[Object]
impl QueryRoot {
    // 示例查询：返回一个简单的字符串
    async fn hello(&self) -> &'static str {
        "Hello, GraphQL!"
    }
}

#[derive(Default)]
struct MySchema;

#[async_graphql::Object]
impl MySchema {
    // 这里是其他查询和突变的实现
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            rocket_graphql::catchers(
                async_graphql::http::GraphQLPlayground::new("/playground")
                    .title("GraphQL Playground"),
                async move |request: rocket::Request<'_>| {
                    let schema = Schema::new(QueryRoot, EmptyMutation::new(), EmptySubscription::new());
                    let gql_request = match GraphQLRequest::from_request(&request, &schema).await {
                        Ok(req) => req,
                        Err(_) => return Err(GraphQLResponse::error("Invalid request", &schema)),
                    };

                    schema.execute(gql_request).await.map_err(|e| {
                        GraphQLResponse::error(&e.to_string(), &schema)
                    }).map(|response| {
                        rocket::response::Response::build()
                            .status(Status::Ok)
                            .header("Content-Type", "application/json")
                            .sized_body(response.size_hint())
                            .ok()?
                            .set_body(response.to_string().unwrap())
                            .ok()?
                    }).await
                },
            ),
        )
        .manage(schema)
}
