use serde::Serialize;

pub mod login;
pub mod signup;

#[derive(Serialize)]
pub struct ApiResponse {
	success:  bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	reason:   Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	auth_key: Option<String>,
}
