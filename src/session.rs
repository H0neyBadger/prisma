use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::{header, Client, Error};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::str;

#[derive(Debug)]
pub struct Session<'a> {
    api_endpoint: &'a str,
    access_key: &'a str,
    secret_key: &'a str,
    client: Client,
    pub token: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

#[allow(dead_code)]
impl<'a> Session<'a> {
    pub fn new(
        api_endpoint: &'a str,
        access_key: &'a str,
        secret_key: &'a str,
        token: Option<String>,
    ) -> Self {
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
            token: token,
            expires_at: None,
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
            .get(url)
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
            .error_for_status()?
            .json::<R>()
            .await?;
        Ok(res)
    }

    pub async fn login_or_refresh(self) -> Result<Session<'a>, Error> {
        if self.token.is_none() {
            return self.login().await;
        };

        if !self.is_expired() {
            return Ok(self);
        };

        match self.refresh().await {
            Ok(value) => Ok(value),
            Err((err, session)) => {
                eprintln!("Refresh token failed with error: {err:?}");
                session.login().await
            }
        }
    }

    pub async fn refresh(mut self) -> Result<Session<'a>, (Error, Session<'a>)> {
        // https://docs.paloaltonetworks.com/prisma/prisma-cloud/prisma-cloud-admin/get-started-with-prisma-cloud/access-the-prisma-cloud-api
        let url = format!("{}/auth_token/extend", self.api_endpoint);

        #[derive(Deserialize)]
        struct RefreshResponse {
            token: String,
        }

        let res = match self
            .client
            .get(url)
            .header("x-redlock-auth", self.token.as_deref().unwrap())
            .send()
            .await
        {
            Ok(value) => value,
            Err(value) => return Err((value, self)),
        };
        let res = match res.error_for_status() {
            Ok(value) => value,
            Err(value) => return Err((value, self)),
        };
        let res = match res.json::<RefreshResponse>().await {
            Ok(value) => value,
            Err(value) => return Err((value, self)),
        };
        // println!("Replace token:\n{}\n{}", self.token.as_deref().unwrap(), res.token.as_str());
        self.token = Some(res.token);
        let token = self.token.as_deref().unwrap();
        self.expires_at = Session::parse_expire_timestamp(token).ok();
        Ok(self)
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

        let token = self.token.as_deref().unwrap();
        self.expires_at = Session::parse_expire_timestamp(token).ok();
        Ok(self)
    }

    fn parse_expire_timestamp(token: &str) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct JwtExpires {
            exp: i64,
        }

        let raw_payload = token
            .split(".")
            .nth(1)
            .ok_or(Box::<dyn std::error::Error>::from(format!(
                "Invalid jwt token {token}"
            )))?;
        let data = general_purpose::URL_SAFE_NO_PAD.decode(raw_payload)?;
        let payload: JwtExpires = serde_json::from_slice(&data)?;
        let naive =
            NaiveDateTime::from_timestamp_opt(payload.exp, 0).ok_or(
                Box::<dyn std::error::Error>::from(format!("Invalid jwt token timestamp {token}")),
            )?;
        let ret = DateTime::from_utc(naive, Utc);
        Ok(ret)
    }

    pub fn is_expired(&self) -> bool {
        if let Some(exp) = self.expires_at.as_ref() {
            let now: DateTime<Utc> = Utc::now();
            println!("compare exp: {exp}, {now}");
            if exp > &now {
                return false;
            }
        };
        true
    }
}
