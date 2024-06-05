#[cfg(feature = "trial")]
use chrono::{DateTime, TimeDelta, Utc};

#[cfg(feature = "trial")]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/build_time.rs"));

#[cfg(feature = "trial")]
pub fn is_trial_over() -> bool {
    let build_time = DateTime::parse_from_rfc3339(BUILD_TIME).expect("Failed to parse build time");
    let build_duration = Utc::now().signed_duration_since(build_time);
    build_duration > TimeDelta::hours(2)
}
