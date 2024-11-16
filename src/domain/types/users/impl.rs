use regex::Regex;
use std::fmt;
use std::ops::Deref;

use super::types::*;
use crate::prelude::*;

impl Email {
    const EMAIL_REGEX: &'static str = r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$";

    pub fn try_from_user_input(email: &str) -> Result<Self, AppError> {
        let re = Regex::new(Self::EMAIL_REGEX).unwrap();
        if re.is_match(email) {
            Ok(Email(email.to_string()))
        } else {
            Err(AppError::bad_request(format!(
                "Invalid email address: {}",
                email
            )))
        }
    }

    pub fn try_from_db(email: &str) -> Result<Self, AppError> {
        let re = Regex::new(Self::EMAIL_REGEX).unwrap();
        if re.is_match(email) {
            Ok(Email(email.to_string()))
        } else {
            Err(AppError::internal_with_private(
                "Invalid email address!",
                format!("Invalid email address: {}", email),
            ))
        }
    }
}

impl Deref for Email {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_lowercase())
    }
}

impl fmt::Display for UniqueUserIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UniqueUserIdentifier::Id(uuid) => write!(f, "{}", uuid),
            UniqueUserIdentifier::Email(email) => write!(f, "{}", email.0),
        }
    }
}
