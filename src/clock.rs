use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time_of_day() -> String {
    time_of_day(SystemTime::now()).unwrap_or_else(|| "unknown time".to_owned())
}

#[cfg(unix)]
fn time_of_day(now: SystemTime) -> Option<String> {
    let timestamp = libc::time_t::try_from(now.duration_since(UNIX_EPOCH).ok()?.as_secs()).ok()?;
    let mut local_time = std::mem::MaybeUninit::<libc::tm>::uninit();

    // SAFETY: `timestamp` points to a valid `time_t`, and `local_time` points to
    // writable memory for `localtime_r` to initialize before `assume_init`.
    let local_time = unsafe {
        let result = libc::localtime_r(&timestamp, local_time.as_mut_ptr());
        if result.is_null() {
            return None;
        }
        local_time.assume_init()
    };

    Some(format_time_of_day(local_time.tm_hour, local_time.tm_min))
}

#[cfg(not(unix))]
fn time_of_day(now: SystemTime) -> Option<String> {
    let total_seconds = now.duration_since(UNIX_EPOCH).ok()?.as_secs() % 86_400;
    let hour = total_seconds / 3_600;
    let minute = (total_seconds % 3_600) / 60;

    Some(format_time_of_day(hour, minute))
}

fn format_time_of_day(hour: impl std::fmt::Display, minute: impl std::fmt::Display) -> String {
    format!("{hour:0>2}:{minute:0>2}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_time_of_day_as_24_hour_clock() {
        assert_eq!(format_time_of_day(7, 5), "07:05");
        assert_eq!(format_time_of_day(21, 15), "21:15");
    }
}
