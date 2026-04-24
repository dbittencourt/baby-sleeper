use std::time::Duration;

use crate::session::Phase;

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3_600;
    let minutes = (total_seconds % 3_600) / 60;
    let seconds = total_seconds % 60;

    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

pub fn status_line(phase: &Phase, elapsed: Duration) -> String {
    match phase {
        Phase::Settling => format!(
            "Settling | time to sleep: {} | s = baby asleep, q = quit",
            format_duration(elapsed)
        ),
        Phase::Sleeping => format!(
            "Sleeping | sleep time: {} | q = finish session",
            format_duration(elapsed)
        ),
    }
}

pub fn fell_asleep_line(time_to_sleep: Duration, time_of_day: &str) -> String {
    format!(
        "Baby fell asleep after {} ({time_of_day})",
        format_duration(time_to_sleep)
    )
}

pub fn slept_line(sleep_duration: Duration, time_of_day: &str) -> String {
    format!(
        "Baby slept for {} ({time_of_day})",
        format_duration(sleep_duration)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_durations_as_hours_minutes_seconds() {
        assert_eq!(format_duration(Duration::from_secs(0)), "00:00:00");
        assert_eq!(format_duration(Duration::from_secs(59)), "00:00:59");
        assert_eq!(format_duration(Duration::from_secs(60)), "00:01:00");
        assert_eq!(format_duration(Duration::from_secs(3_661)), "01:01:01");
        assert_eq!(format_duration(Duration::from_secs(36_000)), "10:00:00");
    }

    #[test]
    fn formats_status_line_for_each_phase() {
        assert_eq!(
            status_line(&Phase::Settling, Duration::from_secs(65)),
            "Settling | time to sleep: 00:01:05 | s = baby asleep, q = quit"
        );
        assert_eq!(
            status_line(&Phase::Sleeping, Duration::from_secs(600)),
            "Sleeping | sleep time: 00:10:00 | q = finish session"
        );
    }

    #[test]
    fn formats_output_lines() {
        assert_eq!(
            fell_asleep_line(Duration::from_secs(576), "13:35"),
            "Baby fell asleep after 00:09:36 (13:35)"
        );
        assert_eq!(
            slept_line(Duration::from_secs(4_440), "14:49"),
            "Baby slept for 01:14:00 (14:49)"
        );
    }
}
