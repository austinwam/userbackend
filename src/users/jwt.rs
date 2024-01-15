use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::{jwterror::AuthError, models::User};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    company: String,
    exp: u64,
}

pub fn generate_tokens(user: User) -> String {
    let time_now = chrono::Utc::now();
    // let iat = time_now.timestamp() as usize;
    let sub: i32 = user.userid;

    let access_token_exp = (time_now + chrono::Duration::hours(3500)).timestamp() as u64;

    let my_claims = Claims {
        sub: sub,
        company: "userbackend".to_owned(),
        exp: access_token_exp,
    };
    let key = b"secret";

    let header = Header {
        kid: Some("signing_key".to_owned()),
        alg: Algorithm::HS512,
        ..Default::default()
    };

    let token = match encode(&header, &my_claims, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };
    return token;
}
