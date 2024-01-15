use crate::common::jwt_auth::{decode_claims, generate_token};

use super::db;

use super::models::{CreateUser, UserLogin};
use axum::extract::{self, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use bcrypt::verify;
use hyper::HeaderMap;
use serde_json::json;
use sqlx::PgPool;

// - - - - - - - - - - - [ROUTES] - - - - - - - - - - -

// pub fn user_route(pool: Pool<Postgres>) -> Router {
//     Router::new()
//         .route(
//             "/users",
//             axum::routing::post(create_user).get(get_users), // .put(users::apis::edit_user),
//         )
//         .route(
//             "/auth/login",
//             axum::routing::post(login), //.post(users::apis::get_users), // .put(users::apis::edit_user),
//         )
//         .route(
//             "/auth/register",
//             axum::routing::post(register), //.post(users::apis::get_users), // .put(users::apis::edit_user),
//         )
//         .with_state(pool)
// }

// - - - - - - - - - - - [HANDLERS] - - - - - - - - - - -
#[utoipa::path(
    post,
    path = "/api/login",
    request_body = UserLogin,
    responses(
        (status = 201, description = "User created successfully", body = Json<serde_json::Value>),
        (status = 500, description = "User could not be created", body = Json<serde_json::Value>),
    )
)]
pub async fn login(
    extract::State(pool): extract::State<PgPool>,
    Json(userlogin): Json<UserLogin>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = db::getbyemail(pool, userlogin.email).await;

    match user {
        Ok(userdata) => {
            let valid = verify(userlogin.password, &userdata.password).unwrap();
            println!("{}", valid);
            if valid == true {
                let token = generate_token(&userdata);
                match token {
                    Ok(usertoken) => {
                        println!("{}", usertoken);
                        let json_response = serde_json::json!({
                            "status": "success",
                            "data": usertoken
                        });
                        return Ok(Json(json_response));
                    }
                    Err(err) => {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            Json(json!({
                                "status": "error",
                                "message": format!("{:?}", err)
                            })),
                        ));
                    }
                }
            } else {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "status": "error",
                        "message": "user auth failed."
                    })),
                ));
            }
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/register",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created successfully", body = Json<serde_json::Value>),
        (status = 500, description = "User could not be created", body = Json<serde_json::Value>),
    )
)]

pub async fn register(
    extract::State(pool): extract::State<PgPool>,
    Json(newiuser): Json<CreateUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let useremail = db::getbyemail(pool.clone(), newiuser.email.clone()).await;
    match useremail {
        Ok(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": "user already exists."})),
            ));
        }
        Err(_) => {
            let user = db::addtodb(pool, newiuser).await;
            match user {
                Ok(userdata) => {
                    let gtoken = generate_token(&userdata);
                    match gtoken {
                        Ok(token) => {
                            let json_response = serde_json::json!({
                                "status": "success",
                                "token": token,
                                "data": userdata,
                            });
                            return Ok(Json(json_response));
                        }
                        Err(err) => {
                            return Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({"status": "error","message": format!("{:?}", err)})),
                            ));
                        }
                    }
                }
                Err(err) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"status": "error","message": format!("{:?}", err)})),
                    ));
                }
            }
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created successfully", body = Json<serde_json::Value>),
        (status = 500, description = "User could not be created", body = Json<serde_json::Value>),
    )
)]

pub async fn create_user(
    extract::State(pool): extract::State<PgPool>,
    Json(newiuser): Json<CreateUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = db::addtodb(pool, newiuser).await;
    match user {
        Ok(userdata) => {
            let json_response = serde_json::json!({
                "status": "success",
                "data": userdata
            });
            return Ok(Json(json_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List all user successfully", body = Json<Vec<User>>),
        (status = 500, description = "Internal server error when retrieving list of all user", body = Json<Vec<User>>)
    )
)]
pub async fn get_users(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    tracing::trace!("authentication details: {:#?}", headers);
    let claims = match decode_claims(&headers) {
        Ok(claimhs) => {
            println!("{:#?}", claimhs);
        }
        Err((status_code, json_value)) => return Err((status_code, json_value)),
    };

    let results = db::allusers(pool).await.unwrap();
    Ok(Json(results))
}

// pub async fn getauser(
//     State(pool): State<PgPool>,
//     Path(useruid): Path<String>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let user: Result<User, sqlx::Error> = db::getauser(pool, useruid).await;
//     match user {
//         Ok(userdata) => {
//             let json_response = serde_json::json!({
//                 "status": "success",
//                 "data": userdata
//             });
//             return Ok(Json(json_response));
//         }
//         Err(err) => {
//             return Err((
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(json!({"status": "error","message": format!("{:?}", err)})),
//             ));
//         }
//     }
// }

// pub async fn edit_user(
//     extract::State(pool): extract::State<PgPool>,
//     Json(euser): Json<EditUser>,
// ) -> Result<Json<User>, ApiError> {
//     let newuser = db::edit_user(pool, euser).await;
//     match newuser {
//         Ok(newuser) => Ok(Json(User::from(newuser))),
//         Err(e) => Err(ApiError::new_internal(e.to_string())),
//     }
// }

// pub async fn userpay(extract::State(pool): extract::State<PgPool>, Json(userf): Json<Userpay>) {
//     let user: Result<User, sqlx::Error> = db::getauser(pool, userf.useruid.clone()).await;
//     match user {
//         Ok(userdata) => {
//             println!("{:?}", userdata);
//             println!("{:?}", userf);
//             if userf.status == "add" {
//                 println!("addedd======");
//                 let newupaid = userdata.unpaid + userf.count;
//                 println!("new count=={}", newupaid)
//             } else if userf.status == "pay" {
//                 println!("pay======");
//             } else {
//                 println!(" show error");
//             }
//         }
//         Err(err) => {
//             println!("{}", err)
//         }
//     }
// }

// pub async fn delete_user(
//     extract::State(pool): extract::State<PgPool>,
//     Json(newagent): Json<CreateAgent>,
// ) -> Result<Json<Agent>, ApiError> {
//     let agent = db::addtodb(pool, newagent).await;
//     match agent {
//         Ok(agent) => Ok(Json(Agent::from(agent))),
//         Err(e) => Err(ApiError::new_internal(e.to_string())),
//     }
// }
