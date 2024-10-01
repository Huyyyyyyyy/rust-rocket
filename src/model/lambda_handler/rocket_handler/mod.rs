use async_graphql::http::GraphiQLSource;
use rocket::{response::content, State};
pub mod schema;
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rust_rocket::{models, schema::pt_account};
pub use schema::SchemaGraphQL;

pub use db;
pub use models;
pub use schema;

#[rocket::get("/")]
pub fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[rocket::get("/hehe")]
pub fn graphiql() -> models::PtAccount {
    use self::schema::pt_account::dsl::*;

    let connection = &mut db::establish_connection();

    let results = schema::pt_account
        .limit(5)
        .select(models::PtAccount::as_select())
        .load(connection)
        .expect("Error loading posts");
    results
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
    println!("requesttt : {:?}", request);
    let response: GraphQLResponse = request.execute(schema.inner()).await;
    response
}
