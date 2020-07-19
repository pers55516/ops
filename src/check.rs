use std::fmt;

use ops_core::{async_trait, CheckResponse, Checker};

/// Associates a name with a [`Checker`](trait.Checker.html).
pub struct NamedChecker {
    name: String,
    checker: Box<dyn Checker>,
}

#[async_trait]
impl Checker for NamedChecker {
    async fn check(&self) -> CheckResponse {
        self.checker.check().await
    }
}

impl fmt::Debug for NamedChecker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NamedChecker")
            .field("name", &self.name)
            .finish()
    }
}

impl NamedChecker {
    /// Creates a new [`NamedChecker`](struct.NamedChecker.html).
    pub fn new(name: &str, checker: Box<dyn Checker>) -> Self {
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
    #[allow(clippy::borrowed_box)]
    pub fn checker(&self) -> &Box<dyn Checker> {
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
