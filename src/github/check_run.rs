use std::path::PathBuf;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use failure::Error;
use serde::{Deserialize, Serialize};

use crate::clippy::CompilerMessage;

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckRunId {
    pub id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckRun {
    #[serde(flatten)]
    details: CheckDetails,
    status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    completed_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    conclusion: Option<Conclusion>,

    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<Output>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Conclusion {
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "neutral")]
    Neutral,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    title: String,
    summary: String,
    annotations: Vec<Annotation>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CheckDetails {
    name: String,
    head_sha: String,
    title: String,
    summary: String,
}

impl CheckDetails {
    pub fn create(name: String, sha: String, title: String, summary: String) -> Self {
        CheckDetails {
            name,
            head_sha: sha,
            title,
            summary,
        }
    }

    pub fn update_summary(&mut self, summary: String) {
        self.summary = summary;
    }
}

impl CheckRun {
    pub fn new(details: &CheckDetails) -> Self {
        CheckRun {
            details: details.clone(),
            status: Status::InProgress,
            completed_at: None,
            conclusion: None,
            output: None,
        }
    }

    pub fn update(details: &CheckDetails, annotations: Vec<Annotation>) -> Self {
        let title = details.title.clone();
        let summary = details.summary.clone();

        CheckRun {
            details: details.clone(),
            status: Status::InProgress,
            completed_at: None,
            conclusion: None,
            output: Some(Output {
                title,
                summary,
                annotations,
            }),
        }
    }

    pub fn complete(
        details: &CheckDetails,
        conclusion: Conclusion,
        annotations: Vec<Annotation>,
    ) -> Self {
        let title = details.title.clone();
        let summary = details.summary.clone();

        CheckRun {
            details: details.clone(),
            status: Status::InProgress,
            completed_at: Some(Utc::now()),
            conclusion: Some(conclusion),
            output: Some(Output {
                title,
                summary,
                annotations,
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotation {
    annotation_level: AnnotationLevel,
    message: String,
    path: String,
    start_line: u64,
    end_line: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_column: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_column: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AnnotationLevel {
    #[serde(rename = "notice")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "failure")]
    Error,
}

impl Annotation {
    pub fn from_clippy_message(ws_root: &str, data: CompilerMessage) -> Result<Annotation, Error> {
        let level = match data.message.level.as_str() {
            "warning" => AnnotationLevel::Warning,
            "error" => AnnotationLevel::Error,
            _ => AnnotationLevel::Info,
        };
        let path = PathBuf::from_str(&data.target.src_path)?;
        let fixed_path = path.strip_prefix(&ws_root)?;

        let primary_span = data
            .message
            .spans
            .iter()
            .find(|s| s.is_primary)
            .unwrap_or(&data.message.spans[0]);
        let (start_column, end_column) = if primary_span.line_start == primary_span.line_end {
            (
                Some(primary_span.column_start),
                Some(primary_span.column_end),
            )
        } else {
            (None, None)
        };

        Ok(Annotation {
            message: data.message.rendered,
            annotation_level: level,
            path: fixed_path.to_str().unwrap().to_owned(),
            start_line: primary_span.line_start,
            end_line: primary_span.line_end,
            start_column,
            end_column,
        })
    }
}
