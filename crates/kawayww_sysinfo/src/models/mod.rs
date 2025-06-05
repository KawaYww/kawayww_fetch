#[derive(Debug)]
pub struct CPUInfo {
    pub(crate) brand: String,
    pub(crate) core_num: (u64, u64), // (physical-cores number, logical-cores number)
    pub(crate) frequency: f64,       // MHz
}

impl CPUInfo {
    pub fn brand(&self) -> &str {
        self.brand.as_str()
    }

    pub fn core_num(&self) -> (u64, u64) {
        self.core_num
    }

    pub fn frequency(&self) -> f64 {
        self.frequency
    }
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
    const ONE_YEAR: u64 = Self::ONE_MONTH * 12;

    fn time_format(secs: u64, max_units_len: Option<usize>) -> String {
        let max_units_len = max_units_len.unwrap_or(6);
        assert!(
            (1..=6).contains(&max_units_len),
            "`max_units_len` should be one in [1, 2, 3, 4, 5, 6]"
        );

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
pub fn uptime_format() {
    let uptime = Uptime {
        uptime_secs: 133799999,
        idle_secs: 56600,
    };

    let time_format = uptime.uptime_format(Some(6));
    assert_eq!(
        time_format,
        "4 years, 3 months, 18 days, 14 hours, 39 minutes, 59 seconds"
    );

    let time_format = uptime.uptime_format(Some(3));
    assert_eq!(time_format, "4 years, 3 months, 18 days");

    let time_format = uptime.uptime_format(Some(1));
    assert_eq!(time_format, "4 years");
}

#[test]
#[should_panic]
pub fn uptime_format_over_max_units_len() {
    let uptime = Uptime {
        uptime_secs: 133799999,
        idle_secs: 56600,
    };

    let time_format = uptime.uptime_format(Some(100));
    assert_eq!(
        time_format,
        "4 years, 3 months, 18 days, 14 hours, 39 minutes, 59 seconds"
    );
}
