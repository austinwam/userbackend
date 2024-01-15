use axum::{routing::post, Router};
use sqlx::{Pool, Postgres};

use crate::users;

pub fn build_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route(
            "/users",
            post(users::apis::create_user).get(users::apis::get_users), // .put(users::apis::edit_user),
        )
        .route(
            "/auth/login",
            post(users::apis::login), //.post(users::apis::get_users), // .put(users::apis::edit_user),
        )
        .with_state(pool)
}
