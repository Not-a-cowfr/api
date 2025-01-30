use std::fmt;

pub mod login;
pub mod signup;
pub mod verify;

#[derive(Debug)]
pub enum Error {
	InvalidEmail,
	EmailTaken,
	#[allow(dead_code)]
	BadEmail, // waiting on implementation of detecting a bounced verification email
	UsernameTaken,
	InvalidUsername,
	WeakPassword,
	EmptyParam(String),
	DatabaseError(rusqlite::Error),
	EncryptionError(bcrypt::BcryptError),
	InvalidCredentials,
	MissingEnvVariable(std::env::VarError),
	InvalidEnvVariable(std::env::VarError),
	InternalEmailError(String),
	InvalidAuthKey,
	InvalidVerificationCode,
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
			// signup
			| Error::InvalidEmail => write!(f, "Invalid Email"),
			| Error::EmailTaken => write!(f, "Email already in use"),
			| Error::UsernameTaken => write!(f, "Username already exists"),
			| Error::InvalidUsername => write!(f, "Username too long"),
			| Error::WeakPassword => write!(f, "Password does not meet strength requirements"),
			| Error::EmptyParam(e) => write!(f, "{} is empty", e),
			// login
			| Error::InvalidCredentials => write!(f, "Invalid credentials"),
			// email verification
			| Error::BadEmail => write!(f, "Email does not exist"),
			| Error::InvalidAuthKey => write!(f, "Invalid authentication key"),
			| Error::InvalidVerificationCode => write!(f, "Invalid verification code"),
			// internal error
			| Error::DatabaseError(e) => write!(f, "Internal database error: {}", e),
			| Error::EncryptionError(e) => write!(f, "Internal encryption error: {}", e),
			| Error::MissingEnvVariable(e) => write!(f, "Missing environment variable: {}", e),
			| Error::InvalidEnvVariable(e) => {
				write!(f, "Environment variable contains invalid characters: {}", e)
			},
			| Error::InternalEmailError(e) => write!(f, "Internal error sending email: {}", e),
		}
	}
}
