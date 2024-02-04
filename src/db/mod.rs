
    use postgres::{Client, Error as PostgresError, NoTls};
    use crate::DB_URL;
    //main setup
    pub fn set_database() -> Result<(), PostgresError> {
        let mut client = Client::connect(&DB_URL, NoTls)?;
        client.batch_execute(
            "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )
    "
        )?;
        Ok(())
    }
