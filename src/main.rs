use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use lambda_runtime::{service_fn, Error};
mod model;
use model::lambda_handler::handle_lambda_event;
use model::lambda_handler::rocket_handler::{
    graphiql, graphql_query, graphql_request, schema::Query,
};
use rocket::routes;
// use serde_json::json;

use std::env;
use dotenvy::dotenv;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    //Load .env file
    dotenv().ok();
    //Create db_url from the DATABASE_URL environmental variable
    let db_url = env::var("DATABASE_URL").expect("Need DATABASE_URL");
    //Create our connection
    let conn = PgConnection::establish(&db_url).unwrap();
    //Return the connection
    conn
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish();

    // Lambda handler
    // let func = service_fn(|event| handle_lambda_event(event, schema.clone()));
    // lambda_runtime::run(func).await?;

    // Uncomment for Rocket usage

    let db_conn: PgConnection = establish_connection();

    rocket::build()
        .manage(schema)
        .mount("/", routes![graphql_query, graphql_request, graphiql])
        .launch()
        .await?;

    Ok(())
}
