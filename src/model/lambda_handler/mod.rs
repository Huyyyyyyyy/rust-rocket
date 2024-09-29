use async_graphql::{BatchResponse, Request, Value as ConstValue, Variables};
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
    println!("payload : {}", event.payload);
    let selection_set_graphql = event.payload["info"]["selectionSetGraphQL"]
        .as_str()
        .unwrap();
    let route = event.payload["info"]["fieldName"].as_str().unwrap();
    let mut input_fields = String::new();
    if let Some(variables) = event.payload["info"]["variables"]["var"].as_object() {
        for (key, value) in variables {
            if !input_fields.is_empty() {
                input_fields.push_str(", ");
            }
            if value.is_string() {
                input_fields.push_str(&format!("{}: \"{}\"", key, value.as_str().unwrap()));
            } else {
                input_fields.push_str(&format!("{}: {}", key, value));
            }
        }
    }
    let query = format!(
        "query MyQuery {{\n  {}(input: {{ {} }}) {} \n}}",
        route,
        input_fields,
        selection_set_graphql.trim()
    );

    let mut graphql_request_arg = Request::new(query);
    graphql_request_arg = graphql_request_arg
        .variables(Variables::default())
        .operation_name("MyQuery".to_string());

    let graphql_request_rocket = GraphQLRequest(graphql_request_arg);

    let response: GraphQLResponse =
        graphql_request(&State::from(&schema), graphql_request_rocket).await;

    let data: ConstValue;
    if let BatchResponse::Single(single_response) = response.0 {
        data = single_response.data;
    } else {
        data = ConstValue::default();
    }

    println!("data : {}", data);
    Ok(json!({
        "status" : "success",
        "statusCode" : 200,
        "data" : data.into_json()?,
    }))
}
