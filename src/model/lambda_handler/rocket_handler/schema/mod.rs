use async_graphql::{
    Context, EmptyMutation, EmptySubscription, InputObject, Object, Result, Schema, SimpleObject,
};

pub type SchemaGraphQL = Schema<Query, EmptyMutation, EmptySubscription>;

#[derive(SimpleObject)]
struct Post {
    id: i32,
    title: String,
    content: String,
    author: String,
    created_at: String,
}

#[derive(InputObject)]
struct CreatePostInput {
    title: String,
    content: String,
    author: String,
}

#[derive(InputObject)]
struct GetPostRequest {
    id: i32,
}

#[derive(SimpleObject)]
struct GetPostResponse {
    item: Post,
}

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn get_post(&self, _ctx: &Context<'_>, input: GetPostRequest) -> Result<GetPostResponse> {
        let post = Post {
            id: input.id,
            title: "Sample Post Title".to_string(),
            content: "This is the content of the sample post.".to_string(),
            author: "Gia Huy".to_string(),
            created_at: "27/09/2025".to_string(),
        };
        Ok(GetPostResponse { item: post })
    }
}

#[derive(Default)]
struct Mutation;

#[Object]
impl Mutation {
    async fn create_post(&self, _ctx: &Context<'_>, input: CreatePostInput) -> Result<Post> {
        let post = Post {
            id: 1, // Example post ID generation
            title: input.title,
            content: input.content,
            author: input.author,
            created_at: "27/09/2025".to_string(),
        };
        Ok(post)
    }
}
