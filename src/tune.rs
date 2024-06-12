use std::{convert::From, fmt::Display, str::FromStr};

use clap::{Args, Subcommand, ValueEnum};
use sysinfo::{Components, Disks, Networks, System};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = false)]
pub struct TuneCommand {
    #[arg(short, long, help = "Number of cores to provide postgres")]
    cpu_limit: Option<u16>,
    #[arg(
        short,
        long,
        value_parser = storage_unit_parser,
        help = "Amount of memory to provide postgres. ie: '32GB'. (default system memory)"
    )]
    memory_limit: Option<StorageUnit>,
    #[arg(long, help = "Indicates an abnormally high write load expected.")]
    high_write_load: bool,
    #[arg(
        long,
        help = "Indicates that a high number of dead tuples are expected from above average update/delete operations."
    )]
    update_delete_heavy: bool,
    #[arg(long, default_value = ".25")]
    shared_buffer_frac: f64,
}

impl TuneCommand {
    // pub fn exec(self) {
    //     let mut sys = System::new_all();

    //     // First we update all information of our `System` struct.
    //     sys.refresh_all();

    //     println!("=> system:");
    //     // RAM and swap information:
    //     println!("total memory: {} MiB", sys.total_memory() / 1024_u64.pow(2));
    //     println!("used memory : {} MiB", sys.used_memory() / 1024_u64.pow(2));
    //     println!("total swap  : {} MiB", sys.total_swap() / 1024_u64.pow(2));
    //     println!("used swap   : {} MiB", sys.used_swap() / 1024_u64.pow(2));

    //     // Display system information:
    //     println!(
    //         "System name:             {}",
    //         System::name().unwrap_or("".to_string())
    //     );
    //     println!(
    //         "System kernel version:   {}",
    //         System::kernel_version().unwrap_or("".to_string())
    //     );
    //     println!(
    //         "System OS version:       {}",
    //         System::os_version().unwrap_or("".to_string())
    //     );
    //     println!(
    //         "System host name:        {}",
    //         System::host_name().unwrap_or("".to_string())
    //     );

    //     // Number of CPUs:
    //     println!("NB CPUs: {}", sys.cpus().len());
    // }
}

fn storage_unit_parser(s: &str) -> Result<StorageUnit, String> {
    Ok(StorageUnit::from_str(s).map_err(|_| "Invalid storage unit")?)
}

#[derive(Debug, Clone)]
pub enum StorageUnit {
    B(u64),
    KiB(u64),
    MiB(u64),
    GiB(u64),
    TiB(u64),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseStorageUnitError;

impl Display for ParseStorageUnitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid storage unit")
    }
}

impl FromStr for StorageUnit {
    type Err = ParseStorageUnitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u64 = s
            .trim_end_matches(char::is_alphabetic)
            .parse()
            .map_err(|_| ParseStorageUnitError)?;
        let unit = s.trim_start_matches(char::is_numeric);
        match unit {
            "" => Ok(StorageUnit::B(value)),
            "kB" => Ok(StorageUnit::KiB(value)),
            "MB" => Ok(StorageUnit::MiB(value)),
            "GB" => Ok(StorageUnit::GiB(value)),
            "TB" => Ok(StorageUnit::TiB(value)),
            _ => Err(ParseStorageUnitError),
        }
    }
}

impl From<&str> for StorageUnit {
    fn from(s: &str) -> Self {
        let value: u64 = s.trim_end_matches(char::is_alphabetic).parse().unwrap();
        let unit = s.trim_start_matches(char::is_numeric);
        match unit {
            "" => StorageUnit::B(value),
            "kB" => StorageUnit::KiB(value),
            "MB" => StorageUnit::MiB(value),
            "GB" => StorageUnit::GiB(value),
            "TB" => StorageUnit::TiB(value),
            _ => panic!("Invalid amount of storage"),
        }
    }
}

impl Display for StorageUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::B(v) => f.write_fmt(format_args!("{v}")),
            Self::KiB(v) => f.write_fmt(format_args!("{v}kB")),
            Self::MiB(v) => f.write_fmt(format_args!("{v}MB")),
            Self::GiB(v) => f.write_fmt(format_args!("{v}GB")),
            Self::TiB(v) => f.write_fmt(format_args!("{v}TB")),
        }
    }
}

impl StorageUnit {
    pub fn n_bytes(&self) -> u64 {
        match self {
            Self::B(v) => *v,
            Self::KiB(v) => v * 1024_u64,
            Self::MiB(v) => v * 1024_u64.pow(2),
            Self::GiB(v) => v * 1024_u64.pow(3),
            Self::TiB(v) => v * 1024_u64.pow(4),
        }
    }
}

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

impl clap::ValueEnum for TimeUnit {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Microseconds(0),
            Self::Milliseconds(0),
            Self::Seconds(0),
            Self::Minutes(0),
            Self::Hours(0),
            Self::Days(0),
        ]
    }
    fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue> {
        match self {
            Self::Microseconds(..) => Some(clap::builder::PossibleValue::new("us")),
            Self::Milliseconds(..) => Some(clap::builder::PossibleValue::new("ms")),
            Self::Seconds(..) => Some(clap::builder::PossibleValue::new("s")),
            Self::Minutes(..) => Some(clap::builder::PossibleValue::new("min")),
            Self::Hours(..) => Some(clap::builder::PossibleValue::new("h")),
            Self::Days(..) => Some(clap::builder::PossibleValue::new("d")),
            _ => None,
        }
    }
}

pub struct Config {
    max_connections: u64,                  // = 100
    shared_buffers: StorageUnit,           // = 32GB
    effective_cache_size: StorageUnit,     // = 96GB
    maintenance_work_mem: StorageUnit,     // = 2GB
    checkpoint_completion_target: f64,     // = 0.9
    wal_buffers: StorageUnit,              // = 16MB
    default_statistics_target: u64,        // = 100
    random_page_cost: f64,                 // = 1.1
    effective_io_concurrency: u64,         // = 200
    work_mem: StorageUnit,                 // = 83886kB
    huge_pages: String,                    // = try
    min_wal_size: StorageUnit,             // = 1GB
    max_wal_size: StorageUnit,             // = 4GB
    max_worker_processes: u64,             // = 16
    max_parallel_workers_per_gather: u64,  // = 4
    max_parallel_workers: u64,             // = 16
    max_parallel_maintenance_workers: u64, // = 4
}
