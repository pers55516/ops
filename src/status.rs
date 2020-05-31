use std::fmt;

use crate::check::{CheckResponse, Checker, NamedChecker};
use crate::health::Health;

use once_cell::sync::Lazy;
use prometheus::{opts, register_gauge_vec, GaugeVec};
use rayon::prelude::*;
use serde_json::{json, Value};

const HEALTHCHECK_NAME: &str = "healthcheck_name";
const HEALTHCHECK_RESULT: &str = "healthcheck_result";
const HEALTHCHECK_STATUS: &str = "healthcheck_status";

static CHECK_RESULT_GAUGE: Lazy<GaugeVec> = Lazy::new(|| {
    register_gauge_vec!(
        opts!(
            HEALTHCHECK_STATUS,
            "Meters the healthcheck status based for each check and for each result"
        ),
        &[HEALTHCHECK_NAME, HEALTHCHECK_RESULT]
    )
    .unwrap()
});

enum Ready {
    Always,
    Never,
}

pub trait Status: Send + Sync {
    /// Details of the application, as JSON.
    fn about(&self) -> Value;

    /// Determines the readiness of the application.
    fn ready(&self) -> Option<bool>;

    /// Checks the health of the application.
    fn check(&self) -> Option<HealthResult>;
}

#[derive(Debug)]
/// Converts the health result entry to JSON.
pub struct HealthResult {
    name: String,
    description: String,
    health: Health,
    checks: Vec<HealthResultEntry>,
}

impl HealthResult {
    fn new(
        name: String,
        description: String,
        health: Health,
        checks: Vec<HealthResultEntry>,
    ) -> HealthResult {
        HealthResult {
            name,
            description,
            health,
            checks,
        }
    }

    pub(crate) fn to_json(&self) -> Value {
        let health: &'static str = self.health.into();

        json!({
            "name": self.name,
            "description": self.description,
            "health": health,
            "checks": self.checks.iter().map(|c| c.to_json()).collect::<Vec<_>>(),
        })
    }
}

#[derive(Debug)]
struct HealthResultEntry {
    name: String,
    health: Health,
    output: String,
    action: Option<String>,
    impact: Option<String>,
}

impl HealthResultEntry {
    fn new(
        name: String,
        health: Health,
        output: String,
        action: Option<String>,
        impact: Option<String>,
    ) -> HealthResultEntry {
        HealthResultEntry {
            name,
            health,
            output,
            action,
            impact,
        }
    }

    fn to_json(&self) -> Value {
        let health: &'static str = self.health.into();

        json!({
            "name": self.name,
            "health": health,
            "output": self.output,
            "action": self.action,
            "impact": self.impact,
        })
    }
}

/// Builds a status object.
#[derive(Debug)]
pub struct StatusBuilder {}

impl StatusBuilder {
    /// Always returns a status that is always ready.
    pub fn always(name: &str, description: &str) -> StatusNoChecks {
        StatusNoChecks {
            name: name.to_owned(),
            description: description.to_owned(),
            ready: Some(Ready::Always),
            revision: None,
            owners: Vec::new(),
            links: Vec::new(),
        }
    }

    /// Never returns a status that is never ready.
    pub fn never(name: &str, description: &str) -> StatusNoChecks {
        StatusNoChecks {
            name: name.to_owned(),
            description: description.to_owned(),
            ready: Some(Ready::Never),
            revision: None,
            owners: Vec::new(),
            links: Vec::new(),
        }
    }

    /// None returns a status has no concept of readiness.
    pub fn none(name: &str, description: &str) -> StatusNoChecks {
        StatusNoChecks {
            name: name.to_owned(),
            description: description.to_owned(),
            ready: None,
            revision: None,
            owners: Vec::new(),
            links: Vec::new(),
        }
    }

    /// Healthchecks returns a status that expects one or more [`NamedChecker`](struct.NamedChecker.html).
    pub fn healthchecks<T: Checker>(name: &str, description: &str) -> StatusWithChecks<T> {
        StatusWithChecks {
            name: name.to_owned(),
            description: description.to_owned(),
            checkers: Vec::new(),
            revision: None,
            owners: Vec::new(),
            links: Vec::new(),
        }
    }
}

pub struct StatusNoChecks {
    name: String,
    description: String,
    ready: Option<Ready>,
    revision: Option<String>,
    owners: Vec<Owner>,
    links: Vec<Link>,
}

impl fmt::Debug for StatusNoChecks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StatusNoChecks")
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}

impl StatusNoChecks {
    /// Sets the revision, this should be a version control ref.
    pub fn revision(mut self, revision: &str) -> Self {
        self.revision = Some(revision.to_owned());
        self
    }

    /// Adds an owner.
    pub fn owner(mut self, name: &str, slack: &str) -> Self {
        self.owners.push(Owner::new(name, slack));
        self
    }

    /// Adds a link.
    pub fn link(mut self, description: &str, url: &str) -> Self {
        self.links.push(Link::new(description, url));
        self
    }
}

impl Status for StatusNoChecks {
    fn about(&self) -> Value {
        json!({
            "name": self.name,
            "description": self.description,
            "links": self.links.iter().map(|l| l.to_json()).collect::<Vec<_>>(),
            "owners": self.owners.iter().map(|o| o.to_json()).collect::<Vec<_>>(),
            "build-info": {
                "revision": self.revision,
            },
        })
    }

    fn ready(&self) -> Option<bool> {
        match self.ready {
            Some(Ready::Always) => Some(true),
            Some(Ready::Never) => Some(false),
            None => None,
        }
    }

    fn check(&self) -> Option<HealthResult> {
        None
    }
}

pub struct StatusWithChecks<T: Checker> {
    name: String,
    description: String,
    checkers: Vec<NamedChecker<T>>,
    revision: Option<String>,
    owners: Vec<Owner>,
    links: Vec<Link>,
}

impl<T: Checker> fmt::Debug for StatusWithChecks<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StatusWithChecks")
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}

impl<T: Checker> StatusWithChecks<T> {
    /// Adds a [`NamedChecker`](`struct.NamedChecker.html`).
    pub fn checker(mut self, checker: NamedChecker<T>) -> Self {
        self.checkers.push(checker);
        self
    }

    /// Sets the revision, this should be a version control ref.
    pub fn revision(mut self, revision: &str) -> Self {
        self.revision = Some(revision.to_owned());
        self
    }

    /// Adds an owner.
    pub fn owner(mut self, name: &str, slack: &str) -> Self {
        self.owners.push(Owner::new(name, slack));
        self
    }

    /// Adds a link.
    pub fn link(mut self, description: &str, url: &str) -> Self {
        self.links.push(Link::new(description, url));
        self
    }

    fn use_health_check(&self) -> bool {
        match self.check().unwrap().health {
            Health::Healthy => true,
            Health::Degraded => true,
            Health::Unhealthy => false,
        }
    }

    fn update_check_metrics(&self, checker: &NamedChecker<T>, response: &CheckResponse) {
        use std::collections::HashMap;

        let res = response.health();

        let map = [
            (HEALTHCHECK_NAME, checker.name()),
            (HEALTHCHECK_RESULT, res.into()),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<&str, &str>>();

        crate::health::HEALTH_STATUSES.iter().for_each(|hs| {
            if &response.health() == hs {
                CHECK_RESULT_GAUGE.with(&map).set(1.0);
            } else {
                CHECK_RESULT_GAUGE.with(&map).set(0.0);
            }
        });
    }
}

impl<T: Checker> Status for StatusWithChecks<T> {
    fn about(&self) -> Value {
        json!({
            "name": self.name,
            "description": self.description,
            "links": self.links.iter().map(|l| l.to_json()).collect::<Vec<_>>(),
            "owners": self.owners.iter().map(|o| o.to_json()).collect::<Vec<_>>(),
            "build-info": {
                "revision": self.revision,
            },
        })
    }

    fn ready(&self) -> Option<bool> {
        Some(self.use_health_check())
    }

    fn check(&self) -> Option<HealthResult> {
        let mut health_result = HealthResult::new(
            self.name.to_owned(),
            self.description.to_owned(),
            Health::Unhealthy,
            self.checkers
                .par_iter()
                .map(|c| {
                    let resp = c.checker().check();
                    self.update_check_metrics(&c, &resp);
                    HealthResultEntry::new(
                        c.name().to_owned(),
                        resp.health().to_owned(),
                        resp.output().to_owned(),
                        resp.action().map(str::to_string),
                        resp.impact().map(str::to_string),
                    )
                })
                .collect(),
        );

        // Finds the highest enum value in the list of checker responses
        health_result.health = match health_result
            .checks
            .iter()
            .max_by(|x, y| x.health.cmp(&y.health))
        {
            Some(status) => status.health,
            None => Health::Unhealthy,
        };

        Some(health_result)
    }
}

struct Owner {
    name: String,
    slack: String,
}

impl Owner {
    fn new(name: &str, slack: &str) -> Self {
        Self {
            name: name.to_owned(),
            slack: slack.to_owned(),
        }
    }

    pub(crate) fn to_json(&self) -> Value {
        json!({
            "name": self.name,
            "slack": self.slack,
        })
    }
}

struct Link {
    description: String,
    url: String,
}

impl Link {
    fn new(description: &str, url: &str) -> Self {
        Self {
            description: description.to_owned(),
            url: url.to_owned(),
        }
    }

    pub(crate) fn to_json(&self) -> Value {
        json!({
            "description": self.description,
            "url": self.url,
        })
    }
}
