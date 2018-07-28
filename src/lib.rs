//! This crate defines several `Duration` constants commonly (or rarely) used
//! in practice.
//!
//! # Usage example
//! ```
//! extern crate durations;
//! 
//! use durations::{SECOND as S, MILLISECOND as MS};
//! 
//! std::thread::sleep(2*S + 200*MS);
//! // or alternatively
//! std::thread::sleep(2.2*S);
//! ```
#![no_std]
use core::time::Duration;

/// 1 nanosecond (10<sup>-9</sup> seconds).
pub const NANOSECOND: Duration = Duration::from_nanos(1);
/// 1 microsecond (10<sup>-6</sup> seconds).
pub const MICROSECOND: Duration = Duration::from_micros(1);
/// 1 millisecond (10<sup>-3</sup> seconds).
pub const MILLISECOND: Duration = Duration::from_millis(1);
/// 1 second.
pub const SECOND: Duration = Duration::from_secs(1);
/// 1 kilosecond (10<sup>3</sup> seconds = 16 minutes 40 seconds).
pub const KILOSECOND: Duration = Duration::from_secs(1_000);
/// 1 megasecond (10<sup>6</sup> seconds = aprox. 11.6 days).
pub const MEGASECOND: Duration = Duration::from_secs(1_000_000);
/// 1 gigasecond (10<sup>9</sup> seconds = aprox. 31.7 years).
pub const GIGASECOND: Duration = Duration::from_secs(1_000_000_000);

/// 1 minute (60 seconds).
///
/// Note that it's different from UTC minute, which can be 59-61 seconds long.
pub const MINUTE: Duration = Duration::from_secs(60);
/// 1 hour (60\*60 = 3 600 seconds).
///
/// Note that it's different from UTC hour, which can be 3559-3661 seconds long.
pub const HOUR: Duration = Duration::from_secs(3_600);
/// 1 day (24\*60\*60 = 86 400 seconds).
///
/// Note that it's different from UTC day, length of which can vary from plus
/// minus second (leap seconds) and up to plus minus an hour (Daylight Save Time).
pub const DAY: Duration = Duration::from_secs(86_400);
/// 1 week (7\*24\*60\*60 = 604 800 seconds).
///
/// Note that it's different from UTC week, length of which can vary due to the
/// existence of leap seconds and Daylight Save Time.
pub const WEEK: Duration = Duration::from_secs(604_800);
/// 1 Julian year (365.25\*24\*60\*60 = 31 557 600 seconds).
pub const YEAR: Duration = Duration::from_secs(31_557_600);
