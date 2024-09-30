use async_graphql::{BatchResponse, Request, Value as ConstValue, Variables};
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};
pub mod rocket_handler;
use lambda_runtime::{Error, LambdaEvent};
use rocket::State;
use rocket_handler::{graphql_request, SchemaGraphQL};
use serde_json::Value;

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

    let new_data = access_object_value(data, route).unwrap();
    let json_data: Value = graphql_value_to_json(&new_data);
    Ok(json_data)
}

fn graphql_value_to_json(value: &ConstValue) -> Value {
    match value {
        ConstValue::Null => Value::Null,
        ConstValue::Boolean(b) => Value::Bool(*b),
        ConstValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Number(i.into())
            } else if let Some(f) = n.as_f64() {
                Value::Number(serde_json::Number::from_f64(f).unwrap())
            } else {
                Value::Null
            }
        }
        ConstValue::String(s) => Value::String(s.clone()),
        ConstValue::List(l) => Value::Array(l.iter().map(graphql_value_to_json).collect()),
        ConstValue::Object(o) => {
            let mut map = serde_json::Map::new();
            for (key, val) in o {
                map.insert(key.clone().to_string(), graphql_value_to_json(val));
            }
            Value::Object(map)
        }
        _ => Value::Null,
    }
}

fn access_object_value(object: ConstValue, key: &str) -> Option<ConstValue> {
    match object {
        ConstValue::Object(map) => map.get(key).cloned(),
        _ => None,
    }
}
