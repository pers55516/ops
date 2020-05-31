pub(crate) const HEALTH_STATUSES: &[Health; 3] =
    &[Health::Healthy, Health::Degraded, Health::Unhealthy];

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
