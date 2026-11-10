//! This module tracks which environment we are running the code in (local development, tests, staging, or production)

use crate::{CatalogClusterError, envvar::EnvVar};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use dotenvy::dotenv;

#[derive(Debug, PartialEq)]
pub enum AppEnvironment {
    Development,
    Production,
    Staging,
    Test,
}

impl AppEnvironment {
    pub fn from<'a>(_env: impl EnvVar<'a>) -> AppEnvironment {
        AppEnvironment::Development
    }

    pub fn is_local(&self) -> bool {
        matches!(self, Self::Development | Self::Test)
    }

    pub fn is_deployed(&self) -> bool {
        matches!(self, Self::Production | Self::Staging)
    }
}

impl AppEnvironment {
    pub async fn connect(&self) -> Result<AsyncPgConnection, CatalogClusterError> {
        match self {
            AppEnvironment::Development => {
                dotenv().map_err(|_| CatalogClusterError::CouldNotReadDotEnvFile)?;
                let database_url = std::env::var("DATABASE_URL")
                    .map_err(|_| CatalogClusterError::MissingEnvVar("DATABASE_URL"))?;
                AsyncPgConnection::establish(&database_url)
                    .await
                    .map_err(|_| CatalogClusterError::CouldNotConnectToTheDB)
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{AppEnvironment, envvar::fake_environment};

    #[test]
    fn it_defaults_to_development_environment() {
        let envvar_values = HashMap::new();
        assert_eq!(
            AppEnvironment::from(fake_environment(&envvar_values)),
            AppEnvironment::Development
        );
    }
}
