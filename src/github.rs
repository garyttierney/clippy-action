use failure::Error;
use reqwest;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json;

use check_run::{CheckRun, CheckRunId};

pub mod check_run;

pub struct Client {
    base_url: String,
    http_client: reqwest::Client,
    token: String,
}

impl Client {
    pub fn new(token: String) -> Self {
        Client {
            base_url: "https://api.github.com".to_owned(),
            http_client: reqwest::Client::new(),
            token,
        }
    }

    fn configure(&mut self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        builder
            .bearer_auth(&self.token)
            .header(header::CONTENT_TYPE, "application/json")
            .header(
                header::ACCEPT,
                "application/vnd.github.antiope-preview+json",
            )
    }

    pub fn create_check_run<S>(
        &mut self,
        repository: S,
        check: &CheckRun,
    ) -> Result<CheckRunId, Error>
    where
        S: AsRef<str>,
    {
        let url = format!(
            "{}/repos/{}/check-runs",
            &self.base_url,
            repository.as_ref()
        );
        let mut response = self
            .configure(self.http_client.post(&url))
            .json(check)
            .send()?;

        Ok(response.json()?)
    }

    pub fn update_check_run<S>(
        &mut self,
        repository: S,
        id: &CheckRunId,
        check: &CheckRun,
    ) -> Result<(), Error>
    where
        S: AsRef<str>,
    {
        let url = format!(
            "{}/repos/{}/check-runs/{}",
            &self.base_url,
            repository.as_ref(),
            id.id
        );
        let mut response = self
            .configure(self.http_client.patch(&url))
            .json(check)
            .send()?;

        Ok(())
    }
}
