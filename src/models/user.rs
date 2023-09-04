use crate::models::errors::ErrorResponse;
use crate::models::jwt::{JwtHandler, JwtToken};
use crate::password::Password;
use crate::schema::users::dsl::users;
use crate::schema::users::{email, username};
use diesel::{
    ExpressionMethods, Insertable, OptionalExtension, PgConnection, QueryDsl, Queryable,
    RunQueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt::Debug;

#[derive(Queryable, Selectable, Serialize, Clone, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub status: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::users)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct UserLogin {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub status: bool,
}

pub struct UserHandler;

impl UserHandler {
    pub fn by_id(conn: &mut PgConnection, _id: i32) -> Result<User, ErrorResponse> {
        let user = users
            .find(_id)
            .select(User::as_returning())
            .first(conn)
            .optional();

        match user {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(ErrorResponse::new("User not found".to_string(), 404)),
            Err(error) => Err(ErrorResponse::new(error.to_string(), 502)),
        }
    }

    pub fn delete(conn: &mut PgConnection, _id: i32) -> Result<Value, ErrorResponse> {
        let result = diesel::delete(users.find(_id)).execute(conn).optional();

        match result {
            Ok(result) => {
                if result.unwrap() == 1 {
                    Ok(json!({"status": true}))
                } else {
                    Err(ErrorResponse::new(
                        "Can't delete this user".to_string(),
                        404,
                    ))
                }
            }
            Err(error) => Err(ErrorResponse::new(error.to_string(), 502)),
        }
    }

    pub fn list(conn: &mut PgConnection) -> Result<Vec<User>, ErrorResponse> {
        let list = users
            .select(User::as_select())
            .limit(5)
            .load(conn)
            .optional();
        match list {
            Ok(Some(list)) => Ok(list),
            Ok(None) => Err(ErrorResponse::new("No user found".to_string(), 404)),
            Err(error) => Err(ErrorResponse::new(error.to_string(), 502)),
        }
    }

    pub fn create(conn: &mut PgConnection, mut new_user: NewUser) -> Result<User, ErrorResponse> {
        let check_user = users
            .filter(email.eq(&new_user.email))
            .or_filter(username.eq(&new_user.username))
            .select(User::as_select())
            .load(conn)
            .optional()
            .unwrap()
            .unwrap();

        if check_user.len() > 0 {
            return Err(ErrorResponse::new(
                "User with this name or email already exist".to_string(),
                404,
            ));
        }

        new_user.password = Password::hash(new_user.password);

        let result = diesel::insert_into(users)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(conn)
            .optional();
        match result {
            Ok(result) => Ok(result.unwrap()),
            Err(error) => Err(ErrorResponse::new(error.to_string(), 502)),
        }
    }

    pub fn login(
        conn: &mut PgConnection,
        login_info: LoginInfo,
    ) -> Result<JwtToken, ErrorResponse> {
        let user = users
            .filter(username.eq(&login_info.username))
            .select(UserLogin::as_select())
            .first(conn)
            .optional();

        match user {
            Ok(Some(user)) => {
                if !Password::verify(&login_info.password, &user.password) {
                    return Err(ErrorResponse::new("Wrong password".to_string(), 401));
                }
                if user.status != true {
                    return Err(ErrorResponse::new("User are blocked.".to_string(), 401));
                }

                JwtHandler::create(user)
            }
            Ok(None) => Err(ErrorResponse::new("User not found".to_string(), 404)),
            Err(error) => Err(ErrorResponse::new(error.to_string(), 502)),
        }
    }
}
