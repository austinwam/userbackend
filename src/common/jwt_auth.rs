// use crate::{
//     common::{db::ConnectionPool, util::load_environment_variable},
//     users::{
//         model::{string_to_user_role, Claims, UpsertUser, User, UserRole},
//         service::service::UsersTable as UsersDB,
//     },
// };
use axum::{http, Json};

use http::{HeaderMap, StatusCode};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind as JwtErrorKind, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use std::time::{Duration, SystemTime};

use crate::users::models::User;

use super::util::load_environment_variable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
}

pub fn generate_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    // let role = string_to_user_role(user.clone().role);
    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(3600)) // Set the token to expire in 1 hour
        .expect("Failed to calculate token expiration")
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH")
        .as_secs() as i64;

    let claims = Claims {
        sub: user.userid.clone(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(load_environment_variable("ENCRYPTION_KEY").as_ref()),
    )
}

pub fn decode_claims(
    headers: &HeaderMap,
) -> Result<Option<TokenData<Claims>>, (StatusCode, Json<Value>)> {
    // Retrieve Authorization header from the map of request headers
    let token_header = headers.get("Authorization");

    // Map token if it exists - return error if not
    let token = match token_header {
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Missing header"})),
            ));
        }
        Some(header) => header.to_str().unwrap(),
    };

    // Return error if the the token does not start with "Bearer"
    if !token.starts_with("Bearer ") {
        eprintln!("Token is missing 'Bearer ' prefix");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Token is missing 'Bearer ' prefix"})),
        ));
    }

    // Attempt to decode token and match the results
    match decode::<Claims>(
        &token[7..],
        &DecodingKey::from_secret(load_environment_variable("ENCRYPTION_KEY").as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Err(err) => {
            match err.kind() {
                // Handle the specific ExpiredSignature error
                JwtErrorKind::ExpiredSignature => {
                    eprintln!("JWT expired: {:?}", err);
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({"error": "Token has expired"})),
                    ))
                }
                _ => {
                    // Handle other decoding errors
                    eprintln!("Error decoding JWT: {:?}", err);
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({"error": "Invalid JWT"})),
                    ))
                }
            }
        }
        Ok(decoded_claims) => Ok(Some(decoded_claims)),
    }
}

// pub fn hash_password(body: &mut UpsertUser) -> Result<(), (StatusCode, Json<Value>)> {
//     if let Ok(hashed_password) = hash(&body.password, 12) {
//         body.password = hashed_password;
//         Ok(())
//     } else {
//         Err((
//             StatusCode::INTERNAL_SERVER_ERROR,
//             Json(json!({"error": "Failed to hash password"})),
//         ))
//     }
// }

// pub async fn enforce_role_policy(
//     shared_state: &ConnectionPool,
//     claims: &Option<TokenData<Claims>>,
//     required_role: UserRole,
// ) -> Result<Option<User>, (StatusCode, Json<Value>)> {
//     let connection = shared_state
//         .pool
//         .get()
//         .expect("Failed to acquire connection from pool");
//     let mut users = UsersDB::new(connection);

//     match users.get_by_email(claims.clone().unwrap().claims.sub) {
//         Ok(user) => {
//             let user_role = string_to_user_role(user.clone().unwrap().role);

//             // Accessing this map under UserRole key will return a list of associated subset roles
//             let role_hierarchy: HashMap<UserRole, Vec<UserRole>> = {
//                 let mut hierarchy = HashMap::new();
//                 hierarchy.insert(
//                     UserRole::ADMIN,
//                     vec![
//                         UserRole::ADMIN,
//                         UserRole::EDITOR,
//                         UserRole::WRITER,
//                         UserRole::READER,
//                     ],
//                 );
//                 hierarchy.insert(
//                     UserRole::EDITOR,
//                     vec![UserRole::EDITOR, UserRole::WRITER, UserRole::READER],
//                 );
//                 hierarchy.insert(UserRole::WRITER, vec![UserRole::WRITER, UserRole::READER]);
//                 hierarchy.insert(UserRole::READER, vec![UserRole::READER]);
//                 hierarchy
//             };

//             // Check if the list of UserRoles associated with HashMap retrieval under key '&user_role' contains the required role '&required_role'
//             if role_hierarchy
//                 .get(&user_role)
//                 .map(|roles| roles.contains(&required_role))
//                 .unwrap_or(false)
//             {
//                 eprintln!("Access granted: User role '{}' is a superset of or equal to required role '{}'", user_role, required_role);
//                 Ok(user)
//             } else {
//                 eprintln!(
//                     "User role: {} does not match required role: {}",
//                     user_role, required_role
//                 );
//                 Err((
//                     StatusCode::UNAUTHORIZED,
//                     Json(
//                         json!({"error": format!("Current role of {} does not have access to {}", user_role, required_role)}),
//                     ),
//                 ))
//             }
//         }
//         Err(err) => {
//             eprintln!("User in claims not found in DB {:?}", err);
//             Err((
//                 StatusCode::UNAUTHORIZED,
//                 Json(json!({"error": "User in claims not found in DB"})),
//             ))
//         }
//     }
// }
