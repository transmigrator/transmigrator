use reqwest::Error as ReqwestError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum FetchError {
    Reqwest(ReqwestError),
    Io(io::Error),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            FetchError::Io(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for FetchError {}

impl From<ReqwestError> for FetchError {
    fn from(err: ReqwestError) -> FetchError {
        FetchError::Reqwest(err)
    }
}

impl From<io::Error> for FetchError {
    fn from(err: io::Error) -> FetchError {
        FetchError::Io(err)
    }
}

pub async fn fetch_proxies_util(url: &str, callback: impl Fn(Vec<String>)) -> Result<(), FetchError> {
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let proxies: Vec<String> = response.text().await?
            .lines()
            .map(|line| line.to_string())
            .collect();
        callback(proxies);
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to fetch proxies").into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_fetch_proxies_util() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let url = "http://example.com/proxies.txt";
            let callback = |proxies: Vec<String>| {
                assert!(!proxies.is_empty());
            };

            let result = fetch_proxies_util(url, callback).await;
            assert!(result.is_ok());
        });
    }
}
