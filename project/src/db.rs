use postgres::{Client, NoTls};
use std::{error::Error, io::Write};

pub fn connect_db() -> Result<Client, Box<dyn Error>> {
    let host = dotenv::var("PG_HOST").unwrap_or_else(|_| "localhost".to_string());
    let user = dotenv::var("PG_USER").unwrap_or_else(|_| "postgres".to_string());
    let pwd = dotenv::var("PG_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
    let db = dotenv::var("PG_DB").unwrap_or_else(|_| "postgres".to_string());
    let schema = dotenv::var("PG_SCHEMA").unwrap_or_else(|_| "public".to_string());

    std::io::stdout().flush()?;
    let mut client = Client::connect(
        &format!("host={host} user={user} password='{pwd}' dbname={db}"),
        NoTls,
    )?;
    client.execute(&format!("SET SCHEMA '{}'", schema), &[])?;
    Ok(client)
}

pub fn init_db(contents: String) {
    let schema = dotenv::var("PG_SCHEMA").unwrap_or_else(|_| "public".to_string());
    match connect_db() {
        Ok(mut db) => {
            db.execute(&format!("DROP SCHEMA IF EXISTS {schema} CASCADE"), &[])
                .expect("");
            db.execute(&format!("CREATE SCHEMA {schema}"), &[])
                .expect("");
            db.execute(&format!("SET SCHEMA '{schema}'"), &[])
                .expect("");
            db.batch_execute(&contents).expect("");
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
