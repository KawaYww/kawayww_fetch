use crate::models::Uptime;
use std::path::Path;

impl Uptime {
    pub fn new() -> Option<Self> {
        let content = Self::read_file("/proc/uptime")?;
        let (uptime_secs, idle_secs) = Self::parse_content(content)?;

        Some(Self {
            uptime_secs,
            idle_secs,
        })
    }

    fn read_file(path: impl AsRef<Path>) -> Option<String> {
        std::fs::read_to_string(path).ok()
    }

    fn parse_content(content: String) -> Option<(u64, u64)> {
        let uptime: Vec<&str> = content.split_whitespace().collect();

        let uptime_secs = uptime.first()?.split('.').next()?.parse().ok()?;
        let idle_secs = uptime.get(1)?.split('.').next()?.parse().ok()?;

        Some((uptime_secs, idle_secs))
    }
}

#[test]
fn uptime() {
    let content = String::from("3323.71 36380.42\n");
    let (uptime_secs, idle_secs) = Uptime::parse_content(content).unwrap();
    assert_eq!(uptime_secs, 3323);
    assert_eq!(idle_secs, 36380);
}
