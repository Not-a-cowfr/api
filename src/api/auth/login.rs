use actix_web::web::Json;
use actix_web::{Responder, get, web};
use serde::Deserialize;

use crate::api::auth::ApiResponse;
use crate::repository::auth::login::login_user;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
	pub username: String,
	pub password: String,
}

#[get("/login")]
pub async fn login(user_info: web::Query<UserInfo>) -> impl Responder {
	let UserInfo { username, password } = user_info.into_inner();

	match login_user(username, password).await {
		| Ok(auth_key) => Json(ApiResponse {
			success:  true,
			reason:   None,
			auth_key: Some(auth_key),
		}),
		| Err(e) => Json(ApiResponse {
			success:  false,
			reason:   Some(e.to_string()),
			auth_key: None,
		}),
	}
}
