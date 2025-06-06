use crate::{models::Uptime, platforms::unix::utils};
use std::path::Path;

pub fn new() -> Option<Uptime> {
    let content = utils::read_file("/proc/uptime")?;
    let (uptime_secs, idle_secs) = get_uptime(content)?;

    Some(Uptime {
        uptime_secs,
        idle_secs,
    })
}

pub(crate) fn get_uptime(content: String) -> Option<(u64, u64)> {
    let uptime: Vec<&str> = content.split_whitespace().collect();

    let uptime_secs = uptime.first()?.split('.').next()?.parse().ok()?;
    let idle_secs = uptime.get(1)?.split('.').next()?.parse().ok()?;

    Some((uptime_secs, idle_secs))
}

#[test]
fn uptime() {
    let content = String::from("3323.71 36380.42\n");
    let (uptime_secs, idle_secs) = get_uptime(content).unwrap();

    assert_eq!(uptime_secs, 3323);
    assert_eq!(idle_secs, 36380);
}
