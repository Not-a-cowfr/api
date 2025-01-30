use bcrypt;
use regex::Regex;
use rusqlite::{Connection, params};
use uuid::Uuid;

use crate::repository::auth::Error;

fn validate_email(email: &str) -> Result<(), Error> {
	if email.is_empty() {
		return Err(Error::EmptyParam("Email".to_owned()));
	}

	let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,63}$").unwrap();
	if !email_regex.is_match(email) {
		return Err(Error::InvalidEmail);
	};

	let conn = Connection::open("src/repository/users.db")?;
	let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE email = ?1")?;
	let count: i64 = stmt.query_row(params![email], |row| row.get(0))?;
	if count > 0 {
		Err(Error::EmailTaken)
	} else {
		Ok(())
	}

	// TODO: send verification email, though still permit adding user if unverified, but dont if email gets bounced (email doesnt exist)
}

fn validate_username(username: &str) -> Result<(), Error> {
	if username.is_empty() {
		return Err(Error::EmptyParam("Username".to_owned()));
	}

	if username.len() > 16 {
		return Err(Error::InvalidUsername);
	}

	let conn = Connection::open("src/repository/users.db")?;
	let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE username = ?1")?;
	let count: i64 = stmt.query_row(params![username], |row| row.get(0))?;
	if count > 0 {
		Err(Error::UsernameTaken)
	} else {
		Ok(())
	}
}

fn validate_password(password: &str) -> Result<(), Error> {
	if password.is_empty() {
		return Err(Error::EmptyParam("Password".to_owned()));
	}

	if password.len() < 7 {
		return Err(Error::WeakPassword);
	}

	let has_digit = password.chars().any(|c| c.is_ascii_digit());
	let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
	let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
	let has_special = password.chars().any(|c| !c.is_ascii_alphanumeric());

	if !(has_digit && has_upper && has_lower && has_special) {
		return Err(Error::WeakPassword);
	}

	Ok(())
}

fn validate_name(name: &str) -> Result<(), Error> {
	if name.is_empty() {
		return Err(Error::EmptyParam("Name".to_owned()));
	}

	Ok(())
}

pub async fn add_user(
	name: String,
	email: String,
	username: String,
	password: String,
) -> Result<String, Error> {
	validate_email(&email)?;
	validate_password(&password)?;
	validate_username(&username)?;
	validate_name(&name)?;

	let user_id = Uuid::new_v4().to_string();
	let auth_key = Uuid::new_v4().to_string();
	let hashed_password =
		bcrypt::hash(&password, bcrypt::DEFAULT_COST).map_err(|e| Error::EncryptionError(e))?;

	let conn = Connection::open("src/repository/users.db")?;
	conn.execute(
		"INSERT INTO users (id, name, username, email, confirmed, password, two_factor, auth_key)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
		params![
			user_id,
			name,
			username,
			email,
			false,
			hashed_password,
			false,
			auth_key,
		],
	)?;

	Ok(auth_key)
}
