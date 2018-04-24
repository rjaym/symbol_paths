

use std::time::{SystemTime};

pub fn time_difference (
  reference_time : & SystemTime
) -> u64 {
  let elapsed = reference_time.elapsed().unwrap();

  (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64
}

