use actix_web::web::Json;
use actix_web::{Responder, post, web};
use rusqlite::Connection;
use serde::Deserialize;

use crate::api::auth::ApiResponse;
use crate::repository::auth::Error;

#[derive(Debug, Deserialize)]
pub struct VerifyInfo {
	pub verification_code: String,
	pub auth_key:          String,
}

#[post("/confirm")]
pub async fn confirm(verify_info: web::Query<VerifyInfo>) -> impl Responder {
	let VerifyInfo {
		auth_key,
		verification_code,
	} = verify_info.into_inner();

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

	let result: Result<Option<String>, _> = conn.query_row(
		"SELECT verification_code FROM users WHERE auth_key = ?",
		[auth_key.clone()],
		|row| row.get::<_, Option<String>>(0),
	);

	let stored_code = match result {
		| Ok(code) => code,
		| Err(rusqlite::Error::QueryReturnedNoRows) => {
			return Json(ApiResponse {
				success:  false,
				reason:   Some(Error::InvalidAuthKey.to_string()),
				auth_key: None,
			});
		},
		| Err(e) => {
			return Json(ApiResponse {
				success:  false,
				reason:   Some(Error::DatabaseError(e).to_string()),
				auth_key: None,
			});
		},
	};

	let Some(stored_code) = stored_code else {
		return Json(ApiResponse {
			success:  false,
			reason:   Some(Error::InvalidAuthKey.to_string()),
			auth_key: None,
		});
	};

	if stored_code != verification_code {
		return Json(ApiResponse {
			success:  false,
			reason:   Some(Error::InvalidVerificationCode.to_string()),
			auth_key: None,
		});
	}

	match conn.execute(
		"UPDATE users SET confirmed = TRUE, verification_code = NULL WHERE auth_key = ?",
		[auth_key],
	) {
		| Ok(_) => Json(ApiResponse {
			success:  true,
			reason:   None,
			auth_key: None,
		}),
		| Err(e) => Json(ApiResponse {
			success:  false,
			reason:   Some(Error::from(e).to_string()),
			auth_key: None,
		}),
	}
}
