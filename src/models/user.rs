use crate::database::Database;
use crate::models::errors::ErrorResponse;
use crate::models::jwt::{JwtHandler, JwtToken};
use crate::password::Password;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::FromRow;
use std::fmt::Debug;

#[derive(Serialize, Clone, Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub status: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(FromRow, Deserialize, Serialize, Debug)]
pub struct UserLogin {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub status: bool,
}

pub struct UserHandler;

impl UserHandler {
    pub async fn by_id(db: Database, _id: i32) -> Result<User, ErrorResponse> {
        let q = "SELECT id, username, email, status FROM users WHERE id = $1";
        let user = sqlx::query_as::<_, User>(q)
            .bind(_id)
            .fetch_one(&db.connection)
            .await;

        match user {
            Ok(user) => Ok(user),
            Err(error) => Err(ErrorResponse::new(error.to_string(), 500)),
        }
    }

    pub async fn delete(db: Database, _id: i32) -> Result<Value, ErrorResponse> {
        let q = "DELETE FROM users WHERE id = $1";
        let result = sqlx::query(q).bind(_id).execute(&db.connection).await;

        match result {
            Ok(result) => {
                let test = match result.rows_affected() {
                    0 => false,
                    1 => true,
                    _ => false,
                };
                Ok(json!({"status": test}))
            }
            Err(error) => Err(ErrorResponse::new(error.to_string(), 500)),
        }
    }

    pub async fn create(db: Database, mut new_user: NewUser) -> Result<User, ErrorResponse> {
        let q = "SELECT id, username, email, status FROM users WHERE username = $1 OR email = $2";

        let check_user = match sqlx::query_as::<_, User>(q)
            .bind(&new_user.username)
            .bind(&new_user.email)
            .fetch_all(&db.connection)
            .await
        {
            Ok(check_user) => check_user,
            Err(error) => {
                return Err(ErrorResponse::new(error.to_string(), 500));
            }
        };

        if check_user.len() > 0 {
            return Err(ErrorResponse::new(
                "User with this name or email already exist".to_string(),
                404,
            ));
        }

        new_user.password = Password::hash(new_user.password);

        let q = "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING id, username, email, status";
        let result = sqlx::query_as::<_, User>(q)
            .bind(new_user.username)
            .bind(new_user.email)
            .bind(new_user.password)
            .fetch_one(&db.connection)
            .await;

        match result {
            Ok(result) => Ok(result),
            Err(error) => Err(ErrorResponse::new(error.to_string(), 500)),
        }
    }

    pub async fn login(db: Database, login_info: LoginInfo) -> Result<JwtToken, ErrorResponse> {
        let q = "SELECT id, username, status, password FROM users WHERE username = $1";
        let user = sqlx::query_as::<_, UserLogin>(q)
            .bind(&login_info.username)
            .fetch_one(&db.connection)
            .await;

        match user {
            Ok(user) => {
                if !Password::verify(&login_info.password, &user.password) {
                    return Err(ErrorResponse::new("Wrong password".to_string(), 401));
                }
                if user.status != true {
                    return Err(ErrorResponse::new("User are blocked.".to_string(), 401));
                }

                JwtHandler::create(user)
            }
            Err(error) => Err(ErrorResponse::new(error.to_string(), 500)),
        }
    }
}
