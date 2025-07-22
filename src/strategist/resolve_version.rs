use crate::config::Version;

pub(crate) fn resolve_version(version: Version) -> String {
    match version {
        Version::ZeroZeroTimestamp => format!("0.0.{}", chrono::Utc::now().timestamp()),
        Version::Specific(version) => version.clone(),
    }
}
