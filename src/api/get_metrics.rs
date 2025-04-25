use crate::metric::Metric;

/// Response of [`crate::client::Client::get_metrics`]
pub type GetMetricsRes = Vec<Metric>;
