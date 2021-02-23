use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::{prelude::*, QueryResult, Queryable};

use crate::{
    config::database::Connection,
    models::user_token::UserToken,
    schema::users::{self, dsl::*},
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub login_session: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfoUser {
    pub id: i32,
    pub login_session: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "users"]
pub struct SignUpUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct ChangeUserName {
    pub username: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct ChangeEmail {
    pub email: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct ChangePassword {
    pub password: String,
}

impl User {
    pub fn find_user_by_email(email_input: &str, conn: &Connection) -> QueryResult<User> {
        users.filter(email.eq(email_input)).get_result::<User>(conn)
    }

    pub fn create_user(new_user: SignUpUser, conn: &Connection) -> QueryResult<usize> {
        let hashed_pwd = hash(&new_user.password, DEFAULT_COST).unwrap();
        let user = SignUpUser {
            password: hashed_pwd,
            ..new_user
        };
        diesel::insert_into(users).values(&user).execute(conn)
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn is_valid_login_session(user_token: &UserToken, conn: &Connection) -> bool {
        users
            .filter(id.eq(user_token.id))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)
            .is_ok()
    }
}
