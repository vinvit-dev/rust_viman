use crate::models::{Claims, ErrorResponse, JwtToken, LoginInfo, NewUser, User, UserLogin};
use crate::utils::get_secret_jwt;
use crate::{
    password::Password,
    schema::users::{dsl::users, email, username},
};
use chrono::Local;
use diesel::{
    result::Error, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::form::validate::Len;

pub struct UserHander;

impl UserHander {
    pub fn by_id(conn: &mut PgConnection, _id: i32) -> Result<Option<User>, Error> {
        users
            .find(_id)
            .select(User::as_returning())
            .first(conn)
            .optional()
    }

    pub fn delete(conn: &mut PgConnection, _id: i32) -> Result<bool, ErrorResponse> {
        let result = diesel::delete(users.find(_id)).execute(conn).optional();

        match result {
            Ok(_) => Ok(true),
            Err(_) => Err(ErrorResponse::new("Fail to delete User".to_string())),
        }
    }

    pub fn list(conn: &mut PgConnection) -> Result<Option<Vec<User>>, Error> {
        users.select(User::as_select()).load(conn).optional()
    }

    pub fn create(conn: &mut PgConnection, mut new_user: NewUser) -> Result<Option<User>, Error> {
        use crate::schema::users;

        let check_user = users
            .filter(email.eq(&new_user.email))
            .or_filter(username.eq(&new_user.username))
            .select(User::as_select())
            .load(conn)
            .optional()
            .unwrap();

        if check_user.len() > 0 {
            panic!("User with this name or email already exist!");
        }

        new_user.password = Password::hash(new_user.password);

        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(conn)
            .optional()
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
                if user.status != true {
                    return Err(ErrorResponse::new("User are blocked.".to_string()));
                }
                if !Password::verify(&login_info.password, &user.password) {
                    return Err(ErrorResponse::new("Wrong password".to_string()));
                }
                let claims = Claims {
                    id: user.id,
                    password: user.password,
                    expire: Local::now().timestamp() + 24 * 3600,
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(get_secret_jwt().as_ref()),
                )
                .unwrap();

                Ok(JwtToken { token })
            }
            Ok(None) => Err(ErrorResponse::new("User not found".to_string())),
            Err(_) => Err(ErrorResponse::new("Something wrong".to_string())),
        }
    }
}
