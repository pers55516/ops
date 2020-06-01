use ops_core::Health;

pub(crate) const HEALTH_STATUSES: &[Health; 3] =
    &[Health::Healthy, Health::Degraded, Health::Unhealthy];
