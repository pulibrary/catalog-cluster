//! This module defines the errors that can happen in this application

#[derive(Debug, PartialEq)]
pub enum CatalogClusterError {
    CouldNotReadDotEnvFile,
    CouldNotConnectToTheDB,
    MissingEnvVar(&'static str),
}
