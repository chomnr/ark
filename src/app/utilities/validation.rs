
pub type ValidationResult<T> = Result<T, ValidationError>;

#[derive(Debug)]
pub enum ValidationError {
    ValidationFieldFailure
}

pub trait Validator<T> {
    fn validate(to_valid: T) -> ValidationResult<T>;
} 