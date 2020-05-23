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

pub struct RustBirthday {
    date: DateTime<Utc>,
}

impl Default for RustBirthday {
    fn default() -> Self {
        Self::new()
    }
}

impl RustBirthday {
    pub fn new() -> RustBirthday {
        RustBirthday { date: Utc::now() }
    }

    pub fn set_birthday(y: i32, m: u32, d: u32) -> RustBirthday {
        RustBirthday {
            date: Utc.ymd(y, m, d).and_hms(0, 0, 0),
        }
    }

    pub fn is_now(&mut self) -> bool {
        self.date.month() == 5 && self.date.day() == 15
    }
}
