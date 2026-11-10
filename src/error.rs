//! This module defines the errors that can happen in this application

#[derive(Debug, PartialEq)]
pub enum CatalogClusterError {
    CouldNotBindToPort,
    CouldNotReadDotEnvFile,
    CouldNotConnectToTheDB,
    CouldNotStartHTTPServer,
    MissingEnvVar(&'static str),
}
