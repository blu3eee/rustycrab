use crate::models::{ AuthRequest, AuthResponse };
use std::time::{ SystemTime, UNIX_EPOCH };
use reqwest::Error as ReqwestError;

pub struct SpotifyTokenManager {
    access_token: Option<String>,
    expires_at: Option<u64>,
    client_id: String,
    client_secret: String,
}

impl SpotifyTokenManager {
    // Initializes the token manager with client credentials
    pub fn new(client_id: String, client_secret: String) -> Self {
        SpotifyTokenManager {
            access_token: None,
            expires_at: None,
            client_id,
            client_secret,
        }
    }

    // Checks if the current token is valid
    fn is_token_valid(&self) -> bool {
        match self.expires_at {
            Some(expiry) => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                now < expiry
            }
            None => false,
        }
    }

    // Requests a new token from Spotify
    async fn request_new_token(&mut self) -> Result<(), ReqwestError> {
        let client = reqwest::Client::new();
        let params = AuthRequest {
            grant_type: "client_credentials".to_string(),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
        };

        let res = client
            .post("https://accounts.spotify.com/api/token")
            .form(&params)
            .send().await?
            .json::<AuthResponse>().await?;

        // Assuming the expires_in field contains the number of seconds the token is valid for
        self.access_token = Some(res.access_token);
        let expires_at =
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + res.expires_in - 60; // Subtract 60 seconds as a buffer
        self.expires_at = Some(expires_at);

        Ok(())
    }

    // Returns a valid access token, requesting a new one if necessary
    pub async fn get_valid_token(&mut self) -> Result<String, ReqwestError> {
        if !self.is_token_valid() {
            self.request_new_token().await?;
        }
        Ok(self.access_token.clone().unwrap()) // Safe unwrap because request_new_token() ensures access_token is Some
    }
}
