#![allow(dead_code)]

mod models;
mod schema;
mod model;
mod db;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use lambda_runtime::{service_fn, Error};
use model::lambda_handler::rocket_handler::{
    graphiql, graphql_query, graphql_request, schema::Query,
};
use model::lambda_handler::handle_lambda_event;
use rocket::routes;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::*;
use schema::*;

// use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish();

    // Lambda handler
    // let func = service_fn(|event| handle_lambda_event(event, schema.clone()));
    // lambda_runtime::run(func).await?;

    // Uncomment for Rocket usage

    // DB
    let mut db_conn = db::establish_connection();
    get_1_account(&mut db_conn);
    // DB

    rocket::build()
        .manage(schema)
        .mount("/", routes![graphql_query, graphql_request, graphiql])
        .launch()
        .await?;

    Ok(())
}

fn get_1_account(connection: &mut PgConnection) {
    // let todos = todos::dsl::todos.filter(todos::done.eq(false).and(todos::id.eq(1)));

    let row = pt_account::dsl::pt_account
        .filter(pt_account::account_enabled.eq(false))
        .limit(1)
        .load::<PtAccount>(connection)
        .expect("Error loading ToDos");

    println!("{:?}", row);
}