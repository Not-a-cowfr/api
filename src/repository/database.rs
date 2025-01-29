use rusqlite::{Connection, Result};

pub async fn create_user_db() -> Result<()> {
	let conn = Connection::open("src/repository/users.db")?;
	conn.execute(
		"CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            username TEXT NOT NULL,
            email TEXT NOT NULL,
            confirmed BOOLEAN NOT NULL,
            password TEXT NOT NULL,
            twofactor BOOLEAN NOT NULL,
            auth_key TEXT
        )",
		[],
	)?;
	Ok(())
}
