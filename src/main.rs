use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Object, Request, Result,
    Schema, SimpleObject, Variables,
};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use rocket::{response::content, routes, State};
use serde_json::{json, Value};

#[derive(SimpleObject)]
struct Post {
    id: String,
    title: String,
    content: String,
    author: String,
    created_at: String,
}

#[derive(Default)]
struct Query;

#[Object]
impl Query {
    async fn get_post(&self, _ctx: &Context<'_>, id: String) -> Result<Post> {
        Ok(Post {
            id: id.clone(),
            title: "Sample Post Title".to_string(),
            content: "This is the content of the sample post.".to_string(),
            author: "Gia Huy".to_string(),
            created_at: "27/09/2025".to_string(),
        })
    }
}

type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}


#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<MySchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json", rank = 1)]
async fn graphql_request(schema: &State<MySchema>, request: GraphQLRequest) -> GraphQLResponse {
    println!("Here is the request {:?}", request);
    request.execute(schema.inner()).await
}

// Adjusted Lambda handler for MySchema
async fn handle_lambda_event(
    event: LambdaEvent<Value>,
    schema: MySchema, // Accept schema directly without `State`
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
