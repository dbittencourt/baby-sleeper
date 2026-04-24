use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub enum KeyAction {
    MarkSleeping,
    Quit,
}

pub fn key_action(key: KeyEvent) -> Option<KeyAction> {
    match key.code {
        KeyCode::Char('s' | 'S') => Some(KeyAction::MarkSleeping),
        KeyCode::Char('q' | 'Q') => Some(KeyAction::Quit),
        KeyCode::Char('c' | 'C') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            Some(KeyAction::Quit)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent::new(code, modifiers)
    }

    #[test]
    fn maps_sleep_keys_to_mark_sleeping() {
        assert!(matches!(
            key_action(key(KeyCode::Char('s'), KeyModifiers::NONE)),
            Some(KeyAction::MarkSleeping)
        ));
        assert!(matches!(
            key_action(key(KeyCode::Char('S'), KeyModifiers::SHIFT)),
            Some(KeyAction::MarkSleeping)
        ));
    }

    #[test]
    fn maps_quit_keys_to_quit() {
        assert!(matches!(
            key_action(key(KeyCode::Char('q'), KeyModifiers::NONE)),
            Some(KeyAction::Quit)
        ));
        assert!(matches!(
            key_action(key(KeyCode::Char('Q'), KeyModifiers::SHIFT)),
            Some(KeyAction::Quit)
        ));
        assert!(matches!(
            key_action(key(KeyCode::Char('c'), KeyModifiers::CONTROL)),
            Some(KeyAction::Quit)
        ));
    }

    #[test]
    fn ignores_unmapped_keys() {
        assert!(key_action(key(KeyCode::Char('x'), KeyModifiers::NONE)).is_none());
    }
}
