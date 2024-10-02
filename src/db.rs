use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    // let database_url = "postgres://postgres:8dmAYnE3TLJwJKO9ruaW@alephon-shared-dev-pg.cluster-caxbihou6kbt.ap-southeast-1.rds.amazonaws.com/PT191_OLD";
    // let connection = PgConnection::establish(&database_url)
    //     .expect(&format!("Error connecting to {}", database_url));
    // connection
}