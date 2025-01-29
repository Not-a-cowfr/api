use rusqlite::{params, Connection};

use crate::repository::auth::Error;

pub async fn login_user(
    identifier: String,
    password: String,
) -> Result<String, Error> {
    let query = "SELECT password, auth_key FROM users WHERE username = ?1 OR email = ?2";

    let conn = Connection::open("src/repository/users.db")?;
    let mut stmt = conn.prepare(query)?;

    let (stored_hash, auth_key): (String, String) = stmt
        .query_row(params![identifier, identifier], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .map_err(|e| match e {
            | rusqlite::Error::QueryReturnedNoRows => Error::InvalidCredentials,
            | _ => Error::DatabaseError(e),
        })?;

    let valid = bcrypt::verify(&password, &stored_hash).map_err(Error::EncryptionError)?;

    if valid {
        Ok(auth_key)
    } else {
        Err(Error::InvalidCredentials)
    }
}
