#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(i64);

impl Timestamp {
    pub const fn from_unix_seconds(value: i64) -> Self {
        Self(value)
    }

    pub const fn unix_seconds(self) -> i64 {
        self.0
    }
}
