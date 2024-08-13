use reqwest::Client;
use std::fs;
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::config::{self, Config, Endpoint, Error, ProbeResult};

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::Timeout
        } else if err.is_redirect() {
            Error::TooManyRedirects
        } else if let Some(status) = err.status() {
            Error::UnexpectedStatusCode(status.as_u16())
        } else {
            Error::Unreachable
        }
    }
}

pub async fn all(file: Option<PathBuf>, _json: bool) -> anyhow::Result<String> {
    let config: Config = match file {
        Some(file) => toml::from_str(&fs::read_to_string(file)?)?,
        None => toml::from_str(&fs::read_to_string("hy.toml")?)?,
    };

    let probes: Vec<_> = config
        .endpoints
        .into_iter()
        .map(|e| tokio::task::spawn(probe(e)))
        .collect();
    let mut results = Vec::with_capacity(probes.len());
    for probe in probes {
        results.push(probe.await?);
    }
    let res: String = serde_json::to_string_pretty(&results)?.to_string();

    let errors: Vec<&config::ProbeResult> = results.iter().filter(|r| r.error.is_some()).collect();

    if !errors.is_empty() {
        return Err(anyhow::anyhow!(res));
    }

    Ok(res)
}

pub async fn probe(endpoint: Endpoint) -> ProbeResult {
    let req = request(&endpoint);
    let start_time = Instant::now();
    let resp = req.send().await;
    let duration = start_time.elapsed().as_millis();
    let error = resp
        .and_then(|r| r.error_for_status())
        .err()
        .map(Error::from);
    ProbeResult {
        endpoint,
        duration,
        error,
    }
}

fn request(endpoint: &Endpoint) -> reqwest::RequestBuilder {
    Client::new()
        .get(endpoint.url.clone())
        .timeout(Duration::from_millis(endpoint.timeout.into()))
}
