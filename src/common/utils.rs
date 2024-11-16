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

pub fn get_auth_jwt_secret() -> String {
    const ERROR_MSG: &str = "AUTH_JWT_SECRET should be defined!";
    match env::var("AUTH_JWT_SECRET") {
        Ok(url) if !url.is_empty() => url,
        _ => panic!("{}", ERROR_MSG),
    }
}

pub fn get_auth_access_token_duration_secs() -> u32 {
    const DEFAULT_AUTH_ACCESS_TOKEN_DURATION_SECS: u32 = 5 * 60; // 5 minutes
    const ERROR_MSG: &str =
        "Invalid access token duration specified! (AUTH_ACCESS_TOKEN_DURATION_SECS)";
    match env::var("AUTH_ACCESS_TOKEN_DURATION_SECS") {
        Ok(duration) if !duration.is_empty() => duration.parse::<u32>().expect(ERROR_MSG),
        _ => DEFAULT_AUTH_ACCESS_TOKEN_DURATION_SECS,
    }
}

pub fn get_auth_refresh_token_duration_secs() -> u32 {
    const DEFAULT_AUTH_REFRESH_TOKEN_DURATION_SECS: u32 = 24 * 60 * 60; // 1 day
    const ERROR_MSG: &str =
        "Invalid refresh token duration specified! (AUTH_REFRESH_TOKEN_DURATION_SECS)";
    match env::var("AUTH_REFRESH_TOKEN_DURATION_SECS") {
        Ok(duration) if !duration.is_empty() => duration.parse::<u32>().expect(ERROR_MSG),
        _ => DEFAULT_AUTH_REFRESH_TOKEN_DURATION_SECS,
    }
}
