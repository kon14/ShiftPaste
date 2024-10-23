use std::env;

pub fn get_database_url() -> String {
    const ERROR_MSG: &str = "DATABASE_URL should be defined!";
    match env::var("DATABASE_URL") {
        Ok(url) if !url.is_empty() => url,
        _ => panic!("{}", ERROR_MSG),
    }
}

pub fn get_api_port() -> String {
    const DEFAULT_API_PORT: &str = "4000";
    match env::var("API_PORT") {
        Ok(port) if !port.is_empty() => port,
        _ => DEFAULT_API_PORT.to_string(),
    }
}

pub fn get_api_base_url() -> String {
    const ERROR_MSG: &str = "API_BASE_URL should be defined!";
    match env::var("API_BASE_URL") {
        Ok(url) if !url.is_empty() => url,
        _ => panic!("{}", ERROR_MSG),
    }
}

pub fn get_app_redirect_url() -> String {
    const API_GET_SNIPPETS_PATH: &str = "/snippets";
    match env::var("API_PORT") {
        Ok(url) if !url.is_empty() => url,
        _ => format!(
            "{}{}",
            env::var("API_BASE_URL").unwrap(),
            API_GET_SNIPPETS_PATH
        ),
    }
}
