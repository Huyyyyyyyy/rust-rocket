use async_graphql::{Request, Variables};
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};
pub mod rocket_handler;
use lambda_runtime::{Error, LambdaEvent};
use rocket::State;
use rocket_handler::{graphql_request, SchemaGraphQL};
use serde_json::{json, Value};

pub async fn handle_lambda_event(
    event: LambdaEvent<Value>,
    schema: SchemaGraphQL,
) -> Result<Value, Error> {
    println!("Here is payload {}", event.payload);
    // let selection_set_graphql = event.payload["info"]["selectionSetGraphQL"].as_str().unwrap();
    // let route  = event.payload["info"]["fieldName"].as_str().unwrap();
    // let query = format!(
    //     "query MyQuery {{\n  {}(id: \"\") {{\n    {}\n  }}\n}}",
    //     route,
    //     selection_set_graphql.trim()
    // );
    // let variables = if let Some(arguments) = event.payload["arguments"].as_object() {
    //     Variables::from_json(Value::Object(arguments.clone()))
    // } else {
    //     Variables::default()
    // };
    let query = "query MyQuery {\n  getPost(id: \"1\") {\n    author\n    createdAt\n    content\n    id\n    title\n  }\n}";

    let mut graphql_request_arg = Request::new(query);
    graphql_request_arg = graphql_request_arg.variables(Variables::default());

    let graphql_request_rocket = GraphQLRequest(graphql_request_arg);

    let response: GraphQLResponse =
        graphql_request(&State::from(&schema), graphql_request_rocket).await;

    println!("GraphQL Response: {:?}", response);
    Ok(json!({}))
}
