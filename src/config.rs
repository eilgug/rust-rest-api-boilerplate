use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub supabase_jwt_secret: String,
    pub supabase_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            supabase_jwt_secret: env::var("SUPABASE_JWT_SECRET")?,
            supabase_url: env::var("SUPABASE_URL")?,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid u16"),
        })
    }
}
