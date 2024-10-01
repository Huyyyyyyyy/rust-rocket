use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use lambda_runtime::{service_fn, Error};
use model::lambda_handler::rocket_handler::{
    graphiql, graphql_query, graphql_request, schema::Query,
};
use model::lambda_handler::handle_lambda_event;
use rocket::routes;

mod model;
mod db;
// use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish();

    // Lambda handler
    // let func = service_fn(|event| handle_lambda_event(event, schema.clone()));
    // lambda_runtime::run(func).await?;

    // Uncomment for Rocket usage

    rocket::build()
        .manage(schema)
        .mount("/", routes![graphql_query, graphql_request, graphiql])
        .launch()
        .await?;

    Ok(())
}
