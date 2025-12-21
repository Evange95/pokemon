use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub poke_api_base_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|_| "PORT must be a valid number between 0 and 65535".to_string())?;
        let poke_api_base_url = env::var("POKE_API_BASE_URL")
            .unwrap_or_else(|_| "https://pokeapi.co/api/v2".to_string());

        Ok(Config {
            host,
            port,
            poke_api_base_url,
        })
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn poke_api_base_url(&self) -> &str {
        &self.poke_api_base_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Use a mutex to ensure tests don't run in parallel and interfere with each other
    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_config_defaults() {
        let _lock = TEST_MUTEX.lock().unwrap();

        env::remove_var("HOST");
        env::remove_var("PORT");
        env::remove_var("POKE_API_BASE_URL");

        let config = Config::from_env().unwrap();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert_eq!(config.address(), "0.0.0.0:3000");
        assert_eq!(config.poke_api_base_url(), "https://pokeapi.co/api/v2");
    }

    #[test]
    fn test_config_custom_host() {
        let _lock = TEST_MUTEX.lock().unwrap();

        env::set_var("HOST", "127.0.0.1");
        env::set_var("PORT", "3000");

        let config = Config::from_env().unwrap();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert_eq!(config.address(), "127.0.0.1:3000");

        env::remove_var("HOST");
        env::remove_var("PORT");
        env::remove_var("POKE_API_BASE_URL");
    }

    #[test]
    fn test_config_custom_port() {
        let _lock = TEST_MUTEX.lock().unwrap();

        env::set_var("HOST", "0.0.0.0");
        env::set_var("PORT", "8080");

        let config = Config::from_env().unwrap();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 8080);
        assert_eq!(config.address(), "0.0.0.0:8080");

        env::remove_var("HOST");
        env::remove_var("PORT");
        env::remove_var("POKE_API_BASE_URL");
    }

    #[test]
    fn test_config_invalid_port() {
        let _lock = TEST_MUTEX.lock().unwrap();

        env::set_var("PORT", "invalid");

        let result = Config::from_env();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "PORT must be a valid number between 0 and 65535"
        );

        env::remove_var("PORT");
        env::remove_var("POKE_API_BASE_URL");
    }

    #[test]
    fn test_config_port_out_of_range() {
        let _lock = TEST_MUTEX.lock().unwrap();

        env::set_var("PORT", "99999");

        let result = Config::from_env();
        assert!(result.is_err());

        env::remove_var("PORT");
        env::remove_var("POKE_API_BASE_URL");
    }
}
