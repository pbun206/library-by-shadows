#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: usize,
    pub jwt_maxage: usize,
    pub pepper: String,
}

impl Config {
    #[cfg(test)]
    pub fn test_default() -> Config {
        Config {
            database_url: "sqlite::memory:".into(),
            jwt_secret: "suppose secret is this".into(),
            jwt_expires_in: 6000,
            jwt_maxage: 6000,
            pepper: "& carrot".to_string(),
        }
    }

    // Init based on environmental values
    pub fn init_from_env() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        let pepper = std::env::var("PEPPER").expect("Pepper must be set");

        Self {
            database_url,
            jwt_secret,
            jwt_expires_in: jwt_expires_in.parse().unwrap(),
            jwt_maxage: jwt_maxage.parse().unwrap(),
            pepper,
        }
    }
}
