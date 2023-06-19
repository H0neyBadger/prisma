use reqwest::{header, Client, Error};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug)]
pub struct Session<'a> {
    api_endpoint: &'a str,
    access_key: &'a str,
    secret_key: &'a str,
    client: Client,
    token: Option<String>,
}

#[allow(dead_code)]
impl<'a> Session<'a> {
    pub fn new(api_endpoint: &'a str, access_key: &'a str, secret_key: &'a str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            "application/json; charset=UTF-8".parse().unwrap(),
        );
        headers.insert(
            header::ACCEPT,
            "application/json; charset=UTF-8".parse().unwrap(),
        );
        let client = Client::builder().default_headers(headers).build().unwrap();
        Self {
            api_endpoint: api_endpoint,
            access_key: access_key,
            secret_key: secret_key,
            client: client,
            token: None,
        }
    }

    pub async fn get_with_query<T, R>(&self, uri: &str, query: &T) -> Result<R, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}/{uri}", self.api_endpoint);
        let res = self
            .client
            .post(url)
            .header("x-redlock-auth", self.token.as_deref().unwrap())
            .query(query)
            .send()
            .await?
            .json::<R>()
            .await?;
        Ok(res)
    }

    pub async fn get<R>(&self, uri: &str) -> Result<R, Error>
    where
        R: DeserializeOwned,
    {
        self.get_with_query(uri, &()).await
    }

    pub async fn post<T, R>(&self, uri: &str, json: &T) -> Result<R, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}/{uri}", self.api_endpoint);
        let res = self
            .client
            .post(url)
            .header("x-redlock-auth", self.token.as_deref().unwrap())
            .json::<T>(json)
            .send()
            .await?
            .json::<R>()
            .await?;
        Ok(res)
    }

    pub async fn login(mut self) -> Result<Session<'a>, Error> {
        // https://pan.dev/prisma-cloud/api/cspm/app-login/
        let url = format!("{}/login", self.api_endpoint);

        #[derive(Serialize)]
        struct LoginRequest<'a> {
            username: &'a str,
            password: &'a str,
        }
        #[derive(Deserialize)]
        struct LoginResponse {
            token: String,
        }

        let req = LoginRequest {
            username: self.access_key,
            password: self.secret_key,
        };

        let res = self
            .client
            .post(url)
            .json(&req)
            .send()
            .await?
            .json::<LoginResponse>()
            .await?;
        self.token = Some(res.token);
        Ok(self)
    }
}
