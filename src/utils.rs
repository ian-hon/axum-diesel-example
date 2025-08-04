use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub fn fetch_unix_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards (???)")
        .as_secs() as u128
}
