use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use ops_core::{CheckResponse, Checker};

/// Associates a name with a [`Checker`](trait.Checker.html).
pub struct NamedChecker<T: Checker> {
    name: String,
    checker: T,
}

impl<T: Checker> Future for &NamedChecker<T> {
    type Output = CheckResponse;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.checker.check()).poll(cx)
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
