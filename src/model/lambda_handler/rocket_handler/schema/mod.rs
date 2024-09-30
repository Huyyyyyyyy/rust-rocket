use async_graphql::{
    Context, EmptyMutation, EmptySubscription, InputObject, Object, Result, Schema, SimpleObject,
};
use serde::{Deserialize, Serialize};

pub type SchemaGraphQL = Schema<Query, EmptyMutation, EmptySubscription>;

#[derive(SimpleObject)]
pub struct Post {
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
    pub status_code: i16,
    pub status: String,
    pub data: Post
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Response<T>{
    pub status_code: i16,
    pub status: String,
    pub data: T
}

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn get_post(&self, _ctx: &Context<'_>, input: GetPostRequest) -> Result<GetPostResponse> {
        let post: Post = Post {
            id: input.id,
            title: "Sample Post Title".to_string(),
            content: "This is the content of the sample post.".to_string(),
            author: "Gia Huy".to_string(),
            created_at: "27/09/2025".to_string(),
        };
        Ok(GetPostResponse { 
            status_code : 200,
            status : String::from("success"),
            data : post
         })
    }
}

#[derive(Default)]
struct Mutation;

#[Object]
impl Mutation {
    async fn create_post(&self, _ctx: &Context<'_>, input: CreatePostInput) -> Result<Post> {
        let post = Post {
            id: 1,
            title: input.title,
            content: input.content,
            author: input.author,
            created_at: "27/09/2025".to_string(),
        };
        Ok(post)
    }
}
