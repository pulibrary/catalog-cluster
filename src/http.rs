//! This module is responsible for listening for and responding to TCP/HTTP requests

use crate::AppEnvironment;
use crate::{CatalogClusterError, envvar::EnvVar};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

mod routes;

/// Start an HTTP server
pub async fn start_http_server(
    envvars: impl EnvVar<'_>,
    environment: &AppEnvironment,
) -> Result<(), CatalogClusterError> {
    axum::serve(
        new_tcp_listener(envvars, environment).await?,
        routes::router(),
    )
    .await
    .map_err(|_| CatalogClusterError::CouldNotStartHTTPServer)
}

impl AppEnvironment {
    pub fn default_port(&self) -> u16 {
        if self.is_local() { 3000 } else { 80 }
    }
}

impl From<&AppEnvironment> for IpAddr {
    /// The address to listen on.  Note that if you are deploying to staging
    /// or production, it should be on a private network beind a firewall,
    /// so that malicious actors who know the IP address of the box can't connect
    /// to it
    fn from(environment: &AppEnvironment) -> Self {
        if environment.is_local() {
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
        } else {
            IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
        }
    }
}

/// Start a new tcp listener based on app environment and environment variables
async fn new_tcp_listener<'a>(
    envvars: impl EnvVar<'a>,
    environment: &AppEnvironment,
) -> Result<TcpListener, CatalogClusterError> {
    TcpListener::bind(&listen_address(envvars, environment))
        .await
        .map_err(|_| CatalogClusterError::CouldNotBindToPort)
}

fn listen_address<'a>(envvars: impl EnvVar<'a>, environment: &AppEnvironment) -> SocketAddr {
    let port = envvars("CATALOG_CLUSTER_PORT")
        .ok()
        .and_then(|raw| raw.parse::<u16>().ok())
        .unwrap_or(environment.default_port());
    SocketAddr::new(environment.into(), port)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::envvar::fake_environment;
    use std::collections::HashMap;

    #[tokio::test]
    async fn it_returns_error_when_port_in_use() {
        // find a port that is not in use yet
        let reserved = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = reserved.local_addr().unwrap().port().to_string();

        let envvar_values = HashMap::from([("CATALOG_CLUSTER_PORT", port.as_str())]);
        let result = new_tcp_listener(
            fake_environment(&envvar_values),
            &AppEnvironment::Development,
        )
        .await;
        assert!(matches!(
            result,
            Err(CatalogClusterError::CouldNotBindToPort)
        ));
    }

    #[tokio::test]
    async fn it_binds_to_the_port_if_it_exists() {
        // find a port that is not in use yet
        let reserved = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = reserved.local_addr().unwrap().port().to_string();

        // close the listener, so that we know that this port is available
        drop(reserved);
        let envvar_values = HashMap::from([("CATALOG_CLUSTER_PORT", port.as_str())]);
        let listener = new_tcp_listener(
            fake_environment(&envvar_values),
            &AppEnvironment::Development,
        )
        .await
        .unwrap();
        assert_eq!(listener.local_addr().unwrap().port().to_string(), port);
    }

    #[test]
    fn it_binds_to_localhost_in_dev() {
        assert_eq!(
            IpAddr::from(&AppEnvironment::Development).to_string(),
            "127.0.0.1"
        );
    }

    #[test]
    fn it_binds_to_localhost_in_test() {
        assert_eq!(IpAddr::from(&AppEnvironment::Test).to_string(), "127.0.0.1");
    }

    #[test]
    fn it_binds_on_any_network_in_staging() {
        assert_eq!(
            IpAddr::from(&AppEnvironment::Staging).to_string(),
            "0.0.0.0"
        );
    }

    #[test]
    fn it_binds_on_any_network_in_prod() {
        assert_eq!(
            IpAddr::from(&AppEnvironment::Production).to_string(),
            "0.0.0.0"
        );
    }
}
