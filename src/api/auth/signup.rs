use actix_web::web::Json;
use actix_web::{post, Responder};
use serde::Deserialize;

use crate::api::auth::ApiResponse;
use crate::repository::auth::signup::add_user;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
	pub name:     String,
	pub username: String,
	pub email:    String,
	pub password: String,
}

#[post("/signup")]
pub async fn signup(user_info: Json<UserInfo>) -> impl Responder {
	let UserInfo {
		name,
		username,
		email,
		password,
	} = user_info.into_inner();

	match add_user(name, email, username, password).await {
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
