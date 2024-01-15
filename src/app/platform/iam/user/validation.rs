use super::model::User;

static USERNAME_MAX_LENGTH: usize = 32;
static USERNAME_MIN_LENGTH: usize = 2;

static EMAIL_MAX_LENGTH: usize = 320;
static EMAIL_MIN_LENGTH: usize = 5;

/* 
pub struct UserValidator;

impl UserValidator {
    pub fn is_username_proper_length(username: &String) -> bool {
        if username.len() <  USERNAME_MIN_LENGTH && username.len() != 0 {
            return false;
        }

        if username.len() >  USERNAME_MAX_LENGTH {
            return false;
        }
        true
    }
    
    pub fn is_email_proper_length(email: &String) -> bool {
        if email.len() <  EMAIL_MIN_LENGTH && email.len() != 0 {
            return false;
        }

        if email.len() >  EMAIL_MAX_LENGTH {
            return false;
        }
        true
    }

    pub fn is_time_proper(created_at: i64, updated_at: i64) -> bool {
        if created_at > updated_at {
            return false;
        }
        true
    }
}

impl Validator<User> for UserValidator {
    fn validate(user: User) -> ValidationResult<User> {
        if user.info.username.is_some() && !Self::is_username_proper_length(&user.info.username.clone().unwrap()) {
            return Err(ValidationError::ValidationFieldFailure);
        }

        if user.info.email.as_ref().is_some() && !Self::is_email_proper_length(&user.info.email.clone().unwrap()) {
            return Err(ValidationError::ValidationFieldFailure);
        }

        if !Self::is_time_proper(user.info.created_at, user.info.updated_at) {
            return Err(ValidationError::ValidationFieldFailure);
        }
        Ok(user)
    }
}
*/