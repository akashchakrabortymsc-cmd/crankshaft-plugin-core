//! Shared types and contracts for the Crankshaft plugin system.

use std::collections::HashMap;
use std::time::Duration;

// ─── JobId ───────────────────────────────────────────────

/// A unique identifier for a submitted job.
#[derive(Debug, Clone, PartialEq)]
pub struct JobId(String);

impl JobId {
    /// Creates a new JobId.
    pub fn new(id: String) -> Self {
        JobId(id)
    }
}

impl std::fmt::Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ─── JobStatus ───────────────────────────────────────────

/// The current state of a job.
#[derive(Debug, Clone, PartialEq)]
pub enum JobStatus {
    /// Submitted but not yet running.
    Pending,
    /// Actively executing.
    Running,
    /// Finished successfully.
    Completed,
    /// Finished with an error.
    Failed(String),
    /// Stopped before completion.
    Cancelled,
}

// ─── Job ─────────────────────────────────────────────────

/// The unit of work sent to a plugin for execution.
#[derive(Debug, Clone)]
pub struct Job {
    /// Unique identifier for this job.
    pub id: JobId,
    /// Shell command to execute.
    pub command: String,
    /// Environment variables.
    pub environment: HashMap<String, String>,
    /// Optional execution timeout.
    pub timeout: Option<Duration>,
}

impl Job {
    /// Creates a new Job with the given id and command.
    pub fn new(id: JobId, command: String) -> Self {
        Job {
            id,
            command,
            environment: HashMap::new(),
            timeout: None,
        }
    }
}

// ─── PluginError ─────────────────────────────────────────

/// All the ways a plugin interaction can fail.
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    /// Could not reach the plugin process.
    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    /// No job with this ID exists.
    #[error("job not found: {0}")]
    JobNotFound(String),

    /// Plugin sent unexpected data.
    #[error("invalid response: {0}")]
    InvalidResponse(String),

    /// Plugin did not respond in time.
    #[error("timeout")]
    Timeout,

    /// Catch-all for unexpected errors.
    #[error("unknown error: {0}")]
    Unknown(String),
}

/// Shorthand Result type for plugin operations.
pub type PluginResult<T> = Result<T, PluginError>;

// ─── PluginBackend trait ──────────────────────────────────

/// The contract every plugin must implement.
///
/// If you are building a new execution backend,
/// implement this trait.
pub trait PluginBackend {
    /// Submit a job for execution. Returns a JobId.
    fn submit(&self, job: Job) -> PluginResult<JobId>;

    /// Get the current status of a job.
    fn status(&self, id: &JobId) -> PluginResult<JobStatus>;

    /// Cancel a running job.
    fn cancel(&self, id: &JobId) -> PluginResult<()>;
}

// ─── Tests ───────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_id_display() {
        let id = JobId::new("job-123".to_string());
        assert_eq!(format!("{}", id), "job-123");
    }

    #[test]
    fn test_job_id_equality() {
        let a = JobId::new("abc".to_string());
        let b = JobId::new("abc".to_string());
        assert_eq!(a, b);
    }

    #[test]
    fn test_job_status_failed_carries_message() {
        let status = JobStatus::Failed("out of memory".to_string());
        assert!(matches!(status, JobStatus::Failed(_)));
    }

    #[test]
    fn test_job_creation() {
        let id = JobId::new("job-001".to_string());
        let job = Job::new(id.clone(), "echo hello".to_string());
        assert_eq!(job.command, "echo hello");
        assert_eq!(job.id, id);
    }

    #[test]
    fn test_job_environment_empty_by_default() {
        let id = JobId::new("job-002".to_string());
        let job = Job::new(id, "ls".to_string());
        assert!(job.environment.is_empty());
    }

    #[test]
    fn test_plugin_error_message() {
        let err = PluginError::ConnectionFailed("port 7878".to_string());
        assert_eq!(err.to_string(), "connection failed: port 7878");
    }
}
