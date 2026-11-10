//! This module tracks which environment we are running the code in (local development, tests, staging, or production)

use crate::{CatalogClusterError, envvar::EnvVar};
use diesel::{Connection, PgConnection};
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
}

impl TryFrom<AppEnvironment> for PgConnection {
    type Error = CatalogClusterError;

    fn try_from(environment: AppEnvironment) -> Result<Self, Self::Error> {
        match environment {
            AppEnvironment::Development => {
                dotenv().map_err(|_| Self::Error::CouldNotReadDotEnvFile)?;
                let database_url = std::env::var("DATABASE_URL")
                    .map_err(|_| Self::Error::MissingEnvVar("DATABASE_URL"))?;
                PgConnection::establish(&database_url)
                    .map_err(|_| Self::Error::CouldNotConnectToTheDB)
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
