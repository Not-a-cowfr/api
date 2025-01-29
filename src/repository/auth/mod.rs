use std::fmt;

pub mod login;
pub mod signup;

#[derive(Debug)]
pub enum Error {
	InvalidEmail,
	EmailTaken,
	UsernameTaken,
	InvalidUsername,
	WeakPassword,
	EmptyParam(String),
	DatabaseError(rusqlite::Error),
	EncryptionError(bcrypt::BcryptError),
	InvalidCredentials,
}

impl From<rusqlite::Error> for Error {
	fn from(err: rusqlite::Error) -> Self { Error::DatabaseError(err) }
}

impl From<bcrypt::BcryptError> for Error {
	fn from(err: bcrypt::BcryptError) -> Self { Error::EncryptionError(err) }
}

impl fmt::Display for Error {
	fn fmt(
		&self,
		f: &mut fmt::Formatter,
	) -> fmt::Result {
		match self {
			| Error::InvalidEmail => write!(f, "Invalid Email"),
			| Error::EmailTaken => write!(f, "Email already in use"),
			| Error::UsernameTaken => write!(f, "Username already exists"),
			| Error::InvalidUsername => write!(f, "Username too long"),
			| Error::WeakPassword => write!(f, "Password does not meet strength requirements"),
			| Error::EmptyParam(e) => write!(f, "{} is empty", e),
			| Error::DatabaseError(e) => write!(f, "Internal database error: {}", e),
			| Error::EncryptionError(e) => write!(f, "Internal encryption error: {}", e),
			| Error::InvalidCredentials => write!(f, "Invalid credentials"),
		}
	}
}
