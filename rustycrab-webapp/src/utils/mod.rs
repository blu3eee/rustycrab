use reqwasm::http::Request;
use serde::de::DeserializeOwned;

pub async fn api_fetch<T: DeserializeOwned>(url: &str) -> Result<T, Error> {
    Request::get(&format!("http://localhost:8080/api{url}"))
        .send().await
        .map_err(|_| Error::RequestError)?
        .json::<T>().await
        .map_err(|_| Error::DeserializeError)
}

/// You can use reqwest or other crates to fetch your api.
// pub async fn api_fetch<T>(url: String) -> Result<T, Error> where T: DeserializeOwned {
//     let response = reqwest::get(url).await;
//     if let Ok(data) = response {
//         data.json::<T>().await.map_or(Err(Error::DeserializeError), |repo| Ok(repo))
//     } else {
//         Err(Error::RequestError)
//     }
// }

// You can use thiserror to define your errors.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    RequestError,
    DeserializeError,
    // etc.
}
