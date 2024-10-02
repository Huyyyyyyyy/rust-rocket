use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use lambda_runtime::{service_fn, Error};
use std::env::{self};
mod model;
use model::lambda_handler::handle_lambda_event;
use model::lambda_handler::rocket_handler::{
    graphiql, graphql_query, graphql_request, schema::Query,
};
use rocket::routes;

enum EnvDeployment {
    Development,
    Production,
    Unknown,
}

impl EnvDeployment {
    fn catch_env(env: &str) -> Self {
        match env {
            "Development" => EnvDeployment::Development,
            "Production" => EnvDeployment::Production,
            _ => EnvDeployment::Unknown,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish();
    let environment_deployment = env::var("ENVIRONMENT").unwrap_or_else(|_| "Unknown".to_string());
    match EnvDeployment::catch_env(&environment_deployment) {
        EnvDeployment::Development => {
            rocket::build()
                .manage(schema)
                .mount("/", routes![graphql_query, graphql_request, graphiql])
                .launch()
                .await?;
        }
        EnvDeployment::Production => {
            let func = service_fn(|event| handle_lambda_event(event, schema.clone()));
            lambda_runtime::run(func).await?;
        }
        EnvDeployment::Unknown => {
            println!("env deployment not found !")
        }
    }

    Ok(())
}
