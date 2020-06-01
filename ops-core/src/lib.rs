/// An interface for something that can be periodically checked.
pub trait Checker: Send + Sync {
    /// Runs the check and returns a [`CheckResponse`](struct.CheckResponse.html).
    fn check(&self) -> CheckResponse;
}

/// The response of a check.
#[derive(Debug)]
pub struct CheckResponse {
    health: Health,
    output: String,
    action: Option<String>,
    impact: Option<String>,
}

impl CheckResponse {
    /// Creates a healthy [`CheckResponse`](struct.CheckResponse.html).
    pub fn healthy(output: &str) -> Self {
        CheckResponse {
            health: Health::Healthy,
            output: output.to_owned(),
            action: None,
            impact: None,
        }
    }

    /// Creates a degraded [`CheckResponse`](struct.CheckResponse.html).
    pub fn degraded(output: &str, action: &str) -> Self {
        CheckResponse {
            health: Health::Degraded,
            output: output.to_owned(),
            action: Some(action.to_owned()),
            impact: None,
        }
    }

    /// Creates an unhealthy [`CheckResponse`](struct.CheckResponse.html).
    pub fn unhealthy(output: &str, action: &str, impact: &str) -> Self {
        CheckResponse {
            health: Health::Unhealthy,
            output: output.to_owned(),
            action: Some(action.to_owned()),
            impact: Some(impact.to_owned()),
        }
    }

    /// Health status of the check.
    pub fn health(&self) -> Health {
        self.health
    }

    /// Text representation of the current status.
    pub fn output(&self) -> &str {
        &self.output
    }

    /// Action to resolve the issue if non-healthy.
    pub fn action(&self) -> Option<&str> {
        self.action.as_ref().map(String::as_ref)
    }

    /// Impact of not fixing the issue.
    pub fn impact(&self) -> Option<&str> {
        self.impact.as_ref().map(String::as_ref)
    }
}

/// Health statuses.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Health {
    Healthy,
    Degraded,
    Unhealthy,
}

impl Into<&'static str> for Health {
    fn into(self) -> &'static str {
        match self {
            Health::Healthy => "healthy",
            Health::Degraded => "degraded",
            Health::Unhealthy => "unhealthy",
        }
    }
}
