use anyhow::Context;
use config::Config;
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Deserialize)]
pub struct Configuration {
    pub port: u16,
}

impl Configuration {
    pub fn load() -> anyhow::Result<Self> {
        let config = Config::builder()
            .add_source(
                config::Environment::default()
                    .try_parsing(true)
                    .separator("__"),
            )
            .build()
            .context("Failed to load app configuration")?
            .try_deserialize()
            .context("Cannot deserialize configuration")?;

        Ok(config)
    }

    pub fn address(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), self.port)
    }
}
