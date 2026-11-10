//! This module helps us to read (and mock) environment variables

use std::env::VarError;

// A trait and blanket implementation for functions similar to std::env::var
pub trait EnvVar<'a>: Fn(&'a str) -> Result<String, VarError> + Copy {}
impl<'a, F> EnvVar<'a> for F where F: Copy + Fn(&'a str) -> Result<String, VarError> + 'a {}

#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
/// Create mock environment variables using a HashMap, for use in tests
pub fn fake_environment<'a>(values: &'a HashMap<&'a str, &'a str>) -> impl EnvVar<'a> {
    |key| match values.get(key) {
        Some(value) => Ok(value.to_string()),
        _ => Err(VarError::NotPresent),
    }
}
