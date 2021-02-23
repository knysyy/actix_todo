use std::env;

use anyhow::Context;
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;

use crate::models::user::LoginInfoUser;

static AUTH_KEY: Lazy<String> = Lazy::new(|| env::var("AUTH_KEY").expect("APP_HOST not found."));
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub id: i32,
    pub login_session: String,
}

impl UserToken {
    pub fn generate_token(login_user: &LoginInfoUser) -> anyhow::Result<String> {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000;
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            id: login_user.id,
            login_session: login_user.login_session.clone(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&*AUTH_KEY.as_ref()),
        )
        .context("エンコードに失敗しました。")
    }

    pub fn decode_token(token: String) -> anyhow::Result<TokenData<UserToken>> {
        jsonwebtoken::decode::<UserToken>(
            &token,
            &DecodingKey::from_secret(&AUTH_KEY.as_ref()),
            &Validation::default(),
        )
        .context("デコードに失敗しました。")
    }
}
