#[derive(Debug, Clone)]
pub struct Config{
    pub database_url: String,
    pub jwt_token: String,
    pub jwt_maxage: i64,
    pub port: u16
}

impl Config {
    pub fn init_config() -> Config {
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let jwt_token = std::env::var("JWT_TOKEN").unwrap();
        let jwt_maxage = std::env::var("JWT_MAXAGE").unwrap();
        Config { 
            database_url, 
            jwt_token, 
            jwt_maxage: jwt_maxage.parse::<i64>().expect("Something SOMEHOW!! went fucking wrong with parsing the jwt_maxage"),
            port:8000
        }
    }
}