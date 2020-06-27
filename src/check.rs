use std::fmt;

use ops_core::{async_trait, CheckResponse, Checker};

/// Associates a name with a [`Checker`](trait.Checker.html).
pub struct NamedChecker<T: Checker> {
    name: String,
    checker: T,
}

#[async_trait]
impl<T: Checker> Checker for NamedChecker<T> {
    async fn check(&self) -> CheckResponse {
        self.checker.check().await
    }
}

impl<T: Checker> fmt::Debug for NamedChecker<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NamedChecker")
            .field("name", &self.name)
            .finish()
    }
}

impl<T: Checker> NamedChecker<T> {
    /// Creates a new [`NamedChecker`](struct.NamedChecker.html).
    pub fn new(name: &str, checker: T) -> Self {
        Self {
            name: safe_metric_name(name),
            checker,
        }
    }

    /// The name of the checker.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The actual checker itself
    pub fn checker(&self) -> &T {
        &self.checker
    }
}

fn safe_metric_name(metric_name: &str) -> String {
    metric_name
        .chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '_' | ':' | '0'..='9' => c,
            _ => '_',
        })
        .collect::<String>()
}
