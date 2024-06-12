use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub enum TimeUnit {
    Microseconds(u64),
    Milliseconds(u64),
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTimeUnitError;

impl FromStr for TimeUnit {
    type Err = ParseTimeUnitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u64 = s
            .trim_end_matches(char::is_alphabetic)
            .parse()
            .map_err(|_| ParseTimeUnitError)?;
        let unit = s.trim_start_matches(char::is_numeric);
        match unit {
            "us" => Ok(Self::Microseconds(value)),
            "ms" => Ok(Self::Milliseconds(value)),
            "s" => Ok(Self::Seconds(value)),
            "min" => Ok(Self::Minutes(value)),
            "h" => Ok(Self::Hours(value)),
            "d" => Ok(Self::Days(value)),
            _ => Err(ParseTimeUnitError),
        }
    }
}

impl From<&str> for TimeUnit {
    fn from(s: &str) -> Self {
        let value: u64 = s.trim_end_matches(char::is_alphabetic).parse().unwrap();
        let unit = s.trim_start_matches(char::is_numeric);
        match unit {
            "us" => Self::Microseconds(value),
            "ms" => Self::Milliseconds(value),
            "s" => Self::Seconds(value),
            "min" => Self::Minutes(value),
            "h" => Self::Hours(value),
            "d" => Self::Days(value),
            _ => panic!("Invalid amount of storage"),
        }
    }
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Microseconds(v) => f.write_fmt(format_args!("{v}us")),
            Self::Milliseconds(v) => f.write_fmt(format_args!("{v}ms")),
            Self::Seconds(v) => f.write_fmt(format_args!("{v}s")),
            Self::Minutes(v) => f.write_fmt(format_args!("{v}min")),
            Self::Hours(v) => f.write_fmt(format_args!("{v}h")),
            Self::Days(v) => f.write_fmt(format_args!("{v}d")),
        }
    }
}
