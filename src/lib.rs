use reqwest::header::{HeaderMap, AUTHORIZATION};

pub mod emails;
pub mod model;
pub mod sms;
pub mod users;

pub struct Client {
    pub token: String,
    base_url: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(token: String) -> Client {
        let auth_token = format!("Bearer {}", token);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, auth_token.parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let prod_url = "https://api.clerk.dev/v1/";
        Client {
            token,
            base_url: prod_url.to_string(),
            http_client: client,
        }
    }
}
