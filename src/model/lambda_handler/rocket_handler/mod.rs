use async_graphql::http::GraphiQLSource;
use rocket::{response::content, State};
pub mod schema;
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
pub use schema::SchemaGraphQL;

#[rocket::get("/")]
pub fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[rocket::get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<SchemaGraphQL>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json", rank = 1)]
pub async fn graphql_request(
    schema: &State<SchemaGraphQL>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    println!("request : {:?}",request);
    request.execute(schema.inner()).await
}
