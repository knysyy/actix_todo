use actix_web::web;

use crate::models::user::LoginInfoUser;
use crate::models::user_token::UserToken;
use crate::{
    config::{constants, database::Pool},
    error::CustomServiceError,
    models::user::{LoginUser, SignUpUser, User},
};

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub fn login(
    login_user: LoginUser,
    pool: &web::Data<Pool>,
) -> anyhow::Result<TokenBodyResponse, CustomServiceError> {
    return match User::find_user_by_email(&login_user.email, &pool.get().unwrap()) {
        Ok(user) => {
            if bcrypt::verify(&login_user.password, &user.password).unwrap() {
                let token_result = UserToken::generate_token(&LoginInfoUser {
                    id: user.id,
                    login_session: User::generate_login_session(),
                });
                if token_result.is_err() {
                    let e = token_result.err();
                    log::error!("Tokenの生成に失敗しました。 {:?}", e);
                    return Err(CustomServiceError::Other(anyhow::anyhow!("{:?}", e)));
                }
                let token = token_result.unwrap();

                serde_json::from_value(json!({ "token": token, "token_type": "bearer" })).map_err(
                    |e| {
                        log::error!("Jsonのデコードに失敗しました。 {:?}", e);
                        CustomServiceError::Other(anyhow::anyhow!("{:?}", e))
                    },
                )
            } else {
                log::info!("パスワードが間違っています。");
                Err(CustomServiceError::AuthenticationFailed)
            }
        }
        Err(e) => {
            log::info!("ユーザーの取得中にエラーが発生しました。 {:?}", e);
            Err(CustomServiceError::AuthenticationFailed)
        }
    };
}

pub fn signup(
    signup_user: SignUpUser,
    pool: &web::Data<Pool>,
) -> anyhow::Result<String, CustomServiceError> {
    match User::find_user_by_email(&signup_user.email, &pool.get().unwrap()) {
        Ok(_) => {
            log::info!("ユーザーは既に存在しています。");
            Err(CustomServiceError::Other(anyhow::anyhow!(
                "ユーザーは既に存在しています。"
            )))
        }
        Err(e) => match e {
            diesel::NotFound => {
                User::create_user(signup_user, &pool.get().unwrap()).map_err(|e| {
                    log::error!("{:?}", e);
                    CustomServiceError::Other(anyhow::anyhow!("データの保存に失敗しました。"))
                })?;
                Ok(String::from("会員登録に成功しました。"))
            }
            _ => {
                log::error!("{:?}", e);
                Err(CustomServiceError::Other(anyhow::anyhow!(
                    constants::MESSAGE_UNEXPECTED_ERROR_OCCURRED
                )))
            }
        },
    }
}
