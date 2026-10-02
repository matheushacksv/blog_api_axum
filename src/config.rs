use std::env;

use dotenvy::dotenv;

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16
}

#[derive(Debug)]
struct DatabaseConfig {
    url: String
}

#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
    db: DatabaseConfig,
    auth: AuthConfig
}

#[derive(Debug)]
struct AuthConfig {
    jwt_secret: String
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        // get server config from env
        let server_config = ServerConfig {
            host: env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
            port: env::var("PORT").unwrap_or_else(|_| String::from("8088"))
                .parse::<u16>()
                .unwrap(),
        };
        
        // get database config from env
        let database_config = DatabaseConfig {
            url: env::var("DATABASE_URL").expect("DATABASE_URL must be set in env")
        };

        // get auth config from env
        let auth_config = AuthConfig {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set in env")
        };

        Config {
            server: server_config,
            db: database_config,
            auth: auth_config,
        }
    }

    pub fn db_url(&self) -> &str {
        &self.db.url
    }

    pub fn db_host(&self) -> &str {
        &self.server.host
    }
    
    pub fn db_port(&self) -> u16 {
        self.server.port
    }

    pub fn jwt_secret(&self) -> &str {
        &self.auth.jwt_secret
    }
    
}