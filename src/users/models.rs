use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, ToSchema)]
pub struct User {
    pub userid: i32,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub status: Option<String>,
    pub paid: i32,
    pub unpaid: i32,
    pub amount: i32,
    pub password: String,
    pub role: Option<JsonValue>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct CreateUser {
    pub username: String,
    pub phone: String,
    pub email: String,
    pub status: String,
    pub role: Option<JsonValue>,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct EditUser {
    pub userid: i32,
    pub username: String,
    pub phone: String,
    pub email: String,
    pub status: String,
    pub paid: i32,
    pub unpaid: i32,
    pub amount: i32,
    pub role: Option<JsonValue>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, ToSchema)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, ToSchema)]
pub struct Loginresp {
    pub token: String,
}
