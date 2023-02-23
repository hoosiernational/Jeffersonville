use std::time::{SystemTime, UNIX_EPOCH};

///A Unix timestamp (a container of a u64, which is itself the number of seconds since
/// UTC midnight on New Years 1970)
#[derive(Debug)]
pub struct UnixTimestamp(u64);

impl UnixTimestamp {
    ///Returns the current Unix Timestamp
    pub fn current() -> Self {
        Self(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
    }

    ///Gets this timestamp as simply a u64
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    ///Creates a timestamp instance from this u64
    pub fn from_u64(input: u64) -> Self {
        Self(input)
    }

}
