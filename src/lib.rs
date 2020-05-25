// Copyright 2020 Mark Sta Ana.
// Licensed under the WTFPL License, Version 2.0.

//! A crate for determining if it's Rust's birthday (15th of May).
//! ## usage
//!
//! This crate is [crates.io](https://crates.io/crates/rust_birthday) and can be
//! used by adding `rust_birthday` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! rust_birthday = "0.1.*"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate rust_birthday;
//! ```
//!
//! ### example
//!
//! ```rust
//! use rust_birthday::*;
//! let mut rust_birthday = RustBirthday::new();
//! println!("{:?}", rust_birthday.is_now());
//! ```

use chrono::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
    #[test]
    fn returns_a_rust_birthday_type() {
        let rust_birthday = RustBirthday::new();
        assert_eq!(get_type_of(&rust_birthday), "rust_birthday::RustBirthday",);
    }

    #[test]
    fn is_birthday() {
        let mut rust_birthday = RustBirthday::set_birthday(2020, 5, 15);
        assert_eq!(rust_birthday.is_now(), true);
    }
}

/// RustBirthday struct used to compute Rust's birthday from `date` provided
pub struct RustBirthday {
    /// date to compare against Rust's birthday
    date: DateTime<Utc>,
}

impl Default for RustBirthday {
    fn default() -> Self {
        Self::new()
    }
}

impl RustBirthday {
    /// Returns a RustBirthday instance initialised comparison date of today.
    pub fn new() -> RustBirthday {
        RustBirthday { date: Utc::now() }
    }

    /// Returns a RustBirthday with a manually specified comparison date. Primarily used for testing.
    pub fn set_birthday(y: i32, m: u32, d: u32) -> RustBirthday {
        RustBirthday {
            date: Utc.ymd(y, m, d).and_hms(0, 0, 0),
        }
    }

    /// Returns a bool depending on if RustBirthday.date is equal to Rust's birtjday.
    pub fn is_now(&mut self) -> bool {
        self.date.month() == 5 && self.date.day() == 15
    }
}
