use serde::Serialize;

pub mod auth;

#[derive(Serialize)]
pub struct ApiResponse {
	success: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	reason:  Option<String>,
}
