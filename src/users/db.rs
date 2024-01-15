use super::models::{CreateUser, User};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;

pub async fn addtodb(pool: PgPool, newuser: CreateUser) -> Result<User, sqlx::Error> {
    let hashed = hash(newuser.password, DEFAULT_COST).unwrap();
    let sqlurl =
        "INSERT INTO users (username,phone,email,status,role,password) VALUES ($1,$2,$3,$4,$5,$6)  RETURNING  *";
    let nuser = sqlx::query_as::<_, User>(sqlurl)
        .bind(newuser.username)
        .bind(newuser.phone)
        .bind(newuser.email)
        .bind(newuser.status)
        .bind(newuser.role)
        .bind(hashed)
        .fetch_one(&pool)
        .await?;
    Ok(nuser)
}
pub async fn getbyemail(pool: PgPool, email: String) -> Result<User, sqlx::Error> {
    let fnsql = "SELECT * FROM users where email = $1";
    let euser = sqlx::query_as::<_, User>(fnsql)
        .bind(email)
        .fetch_one(&pool)
        .await?;
    Ok(euser)
}

// pub async fn get_user_by_email(username: &str, state: &SharedState) -> Option<User> {
//     let query_get = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
//         .bind(username)
//         .fetch_one(&state.pgpool)
//         .await;

//     match query_get {
//         Ok(user) => Some(user),
//         Err(e) => {
//             tracing::error!("{}", e);
//             None
//         }
//     }
// }

pub async fn allusers(pool: PgPool) -> Result<Vec<User>, sqlx::Error> {
    let fnsql = "SELECT * FROM users";
    let agents = sqlx::query_as::<_, User>(fnsql)
        .fetch_all(&pool)
        .await
        .unwrap();
    Ok(agents)
}

// pub async fn edit_user(pool: PgPool, edituser: EditUser) -> Result<User, sqlx::Error> {
//     let fnsql: &str =
//         "UPDATE users SET name = $1,phone = $2,status = $3,paid = $4,unpaid = $5, amount = $6 WHERE agentid = $7 RETURNING *";
//     let newuser = sqlx::query_as::<_, User>(fnsql)
//         .bind(edituser.username)
//         .bind(edituser.phone)
//         .bind(edituser.status)
//         .bind(edituser.paid)
//         .bind(edituser.unpaid)
//         .bind(edituser.amount)
//         .bind(edituser.userid)
//         .fetch_one(&pool)
//         .await?;
//     Ok(newuser)
// }
