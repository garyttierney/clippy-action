use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use failure::{Error, Fail};
use log;
use serde_json::Value;

use crate::clippy::CompilerMessage;
use crate::github::check_run::{Annotation, CheckDetails, CheckRun, Conclusion};
use crate::github::Client;

#[derive(Debug, Fail)]
pub enum ActionError {
    #[fail(display = "Required environment variable not found: {}", 0)]
    MissingEnvironmentVar(String),
}

pub struct ActionOptions {
    name: String,
    title: String,
    input_path: String,
    ignore_parse_errors: bool,
}

impl ActionOptions {
    pub fn new(name: String, title: String, input_path: String, ignore_parse_errors: bool) -> Self {
        ActionOptions {
            name,
            title,
            input_path,
            ignore_parse_errors,
        }
    }
}

pub fn run(options: ActionOptions) -> Result<(), Error> {
    fn required_env_var<S>(name: S) -> Result<String, ActionError>
    where
        S: AsRef<str>,
    {
        env::var(name.as_ref())
            .map_err(|_| ActionError::MissingEnvironmentVar(name.as_ref().to_owned()))
    }

    let token = required_env_var("GITHUB_TOKEN")?;
    let sha = required_env_var("GITHUB_SHA")?;
    let repository = required_env_var("GITHUB_REPOSITORY")?;
    let workspace = required_env_var("GITHUB_WORKSPACE")?;

    let mut gh_client = Client::new(token);
    let mut details =
        CheckDetails::create(options.name, sha, options.title, "In Progress".to_owned());
    let new_check_run = CheckRun::new(&details);
    let check_run_id = gh_client.create_check_run(&repository, &new_check_run)?;

    let check_file = File::open(&options.input_path)?;
    let check_file_reader = BufReader::new(check_file);

    let mut annotation_buffer: Vec<Annotation> = Vec::with_capacity(50);
    let mut total_issues: usize = 0;

    for line in check_file_reader.lines() {
        let line = line?;
        let data: Value = serde_json::from_str(&line)?;

        let is_clippy_message = data["reason"]
            .as_str()
            .map(|r| r.eq("compiler-message"))
            .unwrap_or(false);

        if is_clippy_message {
            let msg: Result<CompilerMessage, _> = serde_json::from_value(data);
            match msg {
                Ok(msg) => {
                    total_issues += 1;
                    annotation_buffer.push(Annotation::from_clippy_message(&workspace, msg)?);
                }
                Err(ref e) if options.ignore_parse_errors => {
                    log::debug!("Failed to parse compiler message: {}", &e)
                }
                Err(e) => Err(e)?,
            };
        }

        if annotation_buffer.len() == 50 {
            let annotations = annotation_buffer.drain(0..50).collect();
            let updated_check_run = CheckRun::update(&details, annotations);

            gh_client.update_check_run(&repository, &check_run_id, &updated_check_run)?;
        }
    }

    details.update_summary(format!("{} issues found", total_issues));
    let conclusion = if total_issues > 0 {
        Conclusion::Failure
    } else {
        Conclusion::Success
    };

    let completed_check_run = CheckRun::complete(&details, conclusion, annotation_buffer);
    gh_client.update_check_run(&repository, &check_run_id, &completed_check_run)?;

    Ok(())
}
