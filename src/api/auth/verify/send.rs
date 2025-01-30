use actix_web::web::Json;
use actix_web::{Responder, post, web};
use rusqlite::Connection;
use serde::Deserialize;

use crate::api::auth::ApiResponse;
use crate::repository::auth::Error;
use crate::repository::auth::verify::verify_email;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
	pub auth_key: String,
}

#[post("/send")]
pub async fn send(auth_key: web::Query<UserInfo>) -> impl Responder {
	let conn = match Connection::open("src/repository/users.db") {
		| Ok(conn) => conn,
		| Err(e) => {
			return Json(ApiResponse {
				success:  false,
				reason:   Some(Error::DatabaseError(e).to_string()),
				auth_key: None,
			});
		},
	};

	let UserInfo { auth_key } = auth_key.into_inner();

	let result: Result<(String, String), rusqlite::Error> = conn.query_row(
		"SELECT name, email FROM users WHERE auth_key = ?",
		[&auth_key],
		|row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
	);

	match result {
		| Ok((name, email)) => match verify_email(name, email).await {
			| Ok(_) => Json(ApiResponse {
				success:  true,
				reason:   None,
				auth_key: None,
			}),
			| Err(e) => Json(ApiResponse {
				success:  false,
				reason:   Some(e.to_string()),
				auth_key: None,
			}),
		},
		| Err(rusqlite::Error::QueryReturnedNoRows) => Json(ApiResponse {
			success:  false,
			reason:   Some(Error::InvalidAuthKey.to_string()),
			auth_key: None,
		}),
		| Err(e) => Json(ApiResponse {
			success:  false,
			reason:   Some(Error::DatabaseError(e).to_string()),
			auth_key: None,
		}),
	}
}
