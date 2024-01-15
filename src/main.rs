mod common;
mod db;
mod router;
mod users;
use axum::{routing::get, Router};
use hyper::StatusCode;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

// cargo watch -x run
// sqlx database create --database-url "postgresql://postgres:password@localhost:5432/userdb"
// sqlx migrate run --database-url "postgresql://postgres:password@localhost:5432/userdb"

#[tokio::main]
async fn main() {
    let db_url = "postgresql://postgres:password@127.0.0.1:5432/userdb";
    let pool = db::get_postgres_pool(db_url).await.unwrap_or_else(|_| {
        panic!(
            "Failed to connect to Postgres with provided URL: {}",
            db_url
        )
    });

    tracing_subscriber::fmt::init();
    let hc_router = Router::new().route("/", get(health_check));
    let approuter = router::build_routes(pool);
    let app: Router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .nest("/hc", hc_router)
        .nest("/api", approuter);
    println!("🚀 Server started successfully");
    println!("🚀 Server started at => http://0.0.0.0:3000");
    println!("🚀 let go .............");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(OpenApi)]
#[openapi(
    paths(
        users::apis::get_users,
        users::apis::create_user
    ),
    components(
        schemas(
            users::models::User,
            users::models::CreateUser
        )
    ),

    tags(
        (name = "users", description = "users management API")
    )
)]
struct ApiDoc;

// use serde::{Deserialize, Serialize};

// use jsonwebtoken::errors::ErrorKind;
// use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//     sub: String,
//     company: String,
//     exp: u64,
// }

// fn main() {
//     let my_claims = Claims {
//         sub: "b@b.com".to_owned(),
//         company: "ACME".to_owned(),
//         exp: 10000000000,
//     };
//     let key = b"secret";

//     let header = Header {
//         kid: Some("signing_key".to_owned()),
//         alg: Algorithm::HS512,
//         ..Default::default()
//     };

//     let token = match encode(&header, &my_claims, &EncodingKey::from_secret(key)) {
//         Ok(t) => t,
//         Err(_) => panic!(), // in practice you would return the error
//     };
//     println!("{:?}", token);

//     let token_data = match decode::<Claims>(
//         &token,
//         &DecodingKey::from_secret(key),
//         &Validation::new(Algorithm::HS512),
//     ) {
//         Ok(c) => c,
//         Err(err) => match *err.kind() {
//             ErrorKind::InvalidToken => panic!(), // Example on how to handle a specific error
//             _ => panic!(),
//         },
//     };
//     println!("{:?}", token_data.claims);
//     println!("{:?}", token_data.header);
// }
