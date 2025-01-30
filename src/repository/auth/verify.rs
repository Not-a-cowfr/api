use std::env;
use std::env::VarError;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::random_range;
use rusqlite::{Connection, params};

use crate::repository::auth::Error;

pub async fn verify_email(
	name: String,
	email_address: String,
) -> Result<(), Error> {
	let mut verification_code = String::new();
	for _digit in 0..6 {
		let num = random_range(1..10);
		verification_code.push_str(&num.to_string());
	}

	match env::var("EMAIL_USERNAME") {
		| Ok(username) => {
			let email = Message::builder()
				.from(username.clone().parse().unwrap())
				.to(format!("{} <{}>", name, email_address).parse().unwrap())
				.subject("Verify your account")
				.body(String::from(format!(
					"your verification code is {}",
					verification_code
				)))
				.unwrap();

			match env::var("EMAIL_PASSWORD") {
				| Ok(password) => {
					let creds = Credentials::new(username, password);

					let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")
						.unwrap()
						.credentials(creds)
						.build();

					match mailer.send(&email) {
						| Ok(_) => {
							let conn = Connection::open("src/repository/users.db")?;
							conn.execute(
								"UPDATE users
								SET verification_code = ?
								WHERE email = ?;",
								params![verification_code, email_address],
							)?;

							Ok(())
						},
						| Err(e) => Err(Error::InternalEmailError(e.to_string())),
					}
				},
				| Err(e) => match e {
					| VarError::NotPresent => Err(Error::MissingEnvVariable(e)),
					| VarError::NotUnicode(_) => Err(Error::InvalidEnvVariable(e)),
				},
			}
		},
		| Err(e) => Err(Error::MissingEnvVariable(e)),
	}

	// TODO: use own smtp service and implement detection for when the email doesnt exist
}
