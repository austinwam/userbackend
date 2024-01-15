use super::db;
use super::models::{CreateUser, User, UserLogin};
use axum::extract::{self, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use bcrypt::verify;
use serde_json::json;
use sqlx::PgPool;

#[utoipa::path(
    post,
    path = "/api/login",
    request_body = CreateUser,
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
                let json_response = serde_json::json!({
                    "status": "success",
                    "data": userdata
                });
                return Ok(Json(json_response));
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
pub async fn get_users(State(pool): State<PgPool>) -> Result<impl IntoResponse, Json<Vec<User>>> {
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
