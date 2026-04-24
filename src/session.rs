use std::time::Duration;

pub enum Phase {
    Settling,
    Sleeping,
}

pub struct Session {
    phase: Phase,
}

impl Session {
    pub fn settling() -> Self {
        Self {
            phase: Phase::Settling,
        }
    }

    pub fn phase(&self) -> &Phase {
        &self.phase
    }

    pub fn mark_sleeping(&mut self, elapsed: Duration) -> Option<Duration> {
        if matches!(self.phase, Phase::Sleeping) {
            return None;
        }

        self.phase = Phase::Sleeping;
        Some(elapsed)
    }

    pub fn finish(&self, current_phase_elapsed: Duration) -> Option<Duration> {
        matches!(self.phase, Phase::Sleeping).then_some(current_phase_elapsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_in_settling_phase() {
        let session = Session::settling();

        assert!(matches!(session.phase(), Phase::Settling));
    }

    #[test]
    fn marks_sleeping_once() {
        let mut session = Session::settling();

        assert_eq!(
            session.mark_sleeping(Duration::from_secs(125)),
            Some(Duration::from_secs(125))
        );
        assert!(matches!(session.phase(), Phase::Sleeping));

        assert_eq!(session.mark_sleeping(Duration::from_secs(250)), None);
    }

    #[test]
    fn finish_before_sleep_has_no_sleep_duration() {
        let session = Session::settling();

        assert_eq!(session.finish(Duration::from_secs(42)), None);
    }

    #[test]
    fn finish_after_sleep_records_sleep_duration() {
        let mut session = Session::settling();
        session.mark_sleeping(Duration::from_secs(90));

        assert_eq!(
            session.finish(Duration::from_secs(3_600)),
            Some(Duration::from_secs(3_600))
        );
    }
}
