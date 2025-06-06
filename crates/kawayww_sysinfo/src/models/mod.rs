#[derive(Debug)]
pub struct CPUInfo {
    pub(crate) brand: String,
    pub(crate) physical_core_num: usize,
    pub(crate) logical_core_num: usize,
    pub(crate) freq_khz: u64,       // KHz
}

impl CPUInfo {
    const ONE_KHZ: f64 = 1.0;
    const ONE_MHZ: f64 = Self::ONE_KHZ * 1000.0;
    const ONE_GHZ: f64 = Self::ONE_MHZ * 1000.0;

    pub fn new(brand: String, physical_core_num: usize, logical_core_num: usize, freq_khz: u64) -> Self {
        Self {
            brand,
            physical_core_num,
            logical_core_num,
            freq_khz,
        }
    }


    pub fn brand(&self) -> &str {
        self.brand.as_str()
    }

    pub fn core_num(&self) -> (usize, usize) {
        (self.physical_core_num, self.logical_core_num)
    }

    pub fn physical_core_num(&self) -> usize {
        self.physical_core_num
    }

    pub fn logical_core_num(&self) -> usize {
        self.logical_core_num
    }

    pub fn freq_khz(&self) -> u64 {
        self.freq_khz
    }

    pub fn freq_mhz(&self) -> f64 {
        (self.freq_khz as f64 / Self::ONE_MHZ * 10.0).round() / 10.0
    }

    pub fn freq_ghz(&self) -> f64 {
        (self.freq_khz as f64 / Self::ONE_GHZ * 10.0).round() / 10.0
    }
}

#[test]
fn cpu_freq_hz() {
    let cpu_info = CPUInfo {
        brand: String::from("Testing"),
        physical_core_num: 100000,
        logical_core_num: 200000,
        freq_khz: 1776664
    };

    assert_eq!(cpu_info.freq_khz(), 1776664);
    assert_eq!(cpu_info.freq_mhz(), 1776.7);
    assert_eq!(cpu_info.freq_ghz(), 1.8);
}


#[derive(Debug)]
pub struct Uptime {
    pub(crate) uptime_secs: u64,
    pub(crate) idle_secs: u64,
}

impl Uptime {
    const ONE_MINUTE: u64 = 60;
    const ONE_HOUR: u64 = Self::ONE_MINUTE * 60;
    const ONE_DAY: u64 = Self::ONE_HOUR * 24;
    const ONE_MONTH: u64 = Self::ONE_DAY * 30; // almost
    const ONE_YEAR: u64 = Self::ONE_MONTH * 12; // alomost

    fn time_format(secs: u64, max_units_len: Option<usize>) -> String {
        let max_units_len = max_units_len.unwrap_or(6).clamp(1, 6);

        let mut remaining = secs;

        let years = remaining / Self::ONE_YEAR;
        remaining %= Self::ONE_YEAR;

        let months = remaining / Self::ONE_MONTH;
        remaining %= Self::ONE_MONTH;

        let days = remaining / Self::ONE_DAY;
        remaining %= Self::ONE_DAY;

        let hours = remaining / Self::ONE_HOUR;
        remaining %= Self::ONE_HOUR;

        let minutes = remaining / Self::ONE_MINUTE;
        let seconds = remaining % Self::ONE_MINUTE;

        let parts = [
            (years, "year"),
            (months, "month"),
            (days, "day"),
            (hours, "hour"),
            (minutes, "minute"),
            (seconds, "second"),
        ];
        parts
            .iter()
            .filter(|&&(value, _)| value > 0) // 跳过值为0的单位
            .map(|&(value, unit)| {
                format!("{} {}{}", value, unit, if value == 1 { "" } else { "s" })
            })
            .take(max_units_len)
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn uptime_secs(&self) -> u64 {
        self.uptime_secs
    }

    pub fn uptime_mins(&self) -> u64 {
        self.uptime_secs / Self::ONE_MINUTE
    }

    pub fn uptime_hours(&self) -> u64 {
        self.uptime_secs / Self::ONE_HOUR
    }

    pub fn uptime_days(&self) -> u64 {
        self.uptime_secs / Self::ONE_DAY
    }

    pub fn uptime_months(&self) -> u64 {
        self.uptime_secs / Self::ONE_MONTH
    }

    pub fn uptime_years(&self) -> u64 {
        self.uptime_secs / Self::ONE_YEAR
    }

    pub fn uptime_format(&self, max_units_len: Option<usize>) -> String {
        Self::time_format(self.uptime_secs, max_units_len)
    }

    pub fn idle_secs(&self) -> u64 {
        self.idle_secs
    }

    pub fn idle_mins(&self) -> u64 {
        self.idle_secs / Self::ONE_MINUTE
    }

    pub fn idle_hours(&self) -> u64 {
        self.idle_secs / Self::ONE_HOUR
    }

    pub fn idle_days(&self) -> u64 {
        self.idle_secs / Self::ONE_DAY
    }

    pub fn idle_months(&self) -> u64 {
        self.idle_secs / Self::ONE_MONTH
    }

    pub fn idle_years(&self) -> u64 {
        self.idle_secs / Self::ONE_YEAR
    }

    pub fn idle_format(&self, max_units_len: Option<usize>) -> String {
        Self::time_format(self.idle_secs, max_units_len)
    }
}

#[test]
pub fn time_format() {
    let uptime = Uptime {
        uptime_secs: 133799999,
        idle_secs: 56600,
    };

    let uptime_format = uptime.uptime_format(Some(6));
    let idle_format = uptime.idle_format(Some(6));
    assert_eq!(
        uptime_format,
        "4 years, 3 months, 18 days, 14 hours, 39 minutes, 59 seconds"
    );
    assert_eq!(
        idle_format,
        "15 hours, 43 minutes, 20 seconds"
    );

    let uptime_format = uptime.uptime_format(Some(3));
    let idle_format = uptime.idle_format(Some(3));
    assert_eq!(uptime_format, "4 years, 3 months, 18 days");
    assert_eq!(idle_format, "15 hours, 43 minutes, 20 seconds");

    let uptime_format = uptime.uptime_format(Some(1));
    let idle_format = uptime.idle_format(Some(1));
    assert_eq!(uptime_format, "4 years");
    assert_eq!(idle_format, "15 hours");
}

#[test]
pub fn time_format_over_max_units_len() {
    let uptime = Uptime {
        uptime_secs: 133799999,
        idle_secs: 56600,
    };

    let uptime_format = uptime.uptime_format(Some(100));
    let idle_format = uptime.idle_format(Some(100));
    assert_eq!(
        uptime_format,
        "4 years, 3 months, 18 days, 14 hours, 39 minutes, 59 seconds"
    );
    assert_eq!(
        idle_format,
        "15 hours, 43 minutes, 20 seconds"
    );
}


#[derive(Debug)]
pub struct OSRelease {
    pub(crate) name: String,
    pub(crate) pretty_name: String,
    pub(crate) version: Option<String>, // `None` for rolling system
    pub(crate) version_id: Option<String>,
    pub(crate) version_codename: Option<String>,
}

impl OSRelease {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}
