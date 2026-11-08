//! Lifecycle-State-Machine: INIT -> BOOT -> RUNNING -> DEGRADED -> SHUTDOWN -> STOPPED.
//! Illegale Übergänge werden zurückgewiesen (fail-closed). Jeder Übergang erzeugt
//! einen Audit-Event über die eingehängte Callback-Funktion.

use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Init,
    Boot,
    Running,
    Degraded,
    Shutdown,
    Stopped,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug)]
pub struct LifecycleError {
    pub from: State,
    pub to: State,
}

impl fmt::Display for LifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "illegal lifecycle transition {} -> {}",
            self.from, self.to
        )
    }
}

/// Kanonische Übergangstabelle. Alles Nicht-Gelistete ist illegal.
pub fn can_transition(from: State, to: State) -> bool {
    use State::*;
    matches!(
        (from, to),
        (Init, Boot)
            | (Init, Shutdown)
            | (Boot, Running)
            | (Boot, Degraded)
            | (Boot, Shutdown)
            | (Running, Degraded)
            | (Running, Shutdown)
            | (Degraded, Running)
            | (Degraded, Shutdown)
            | (Shutdown, Stopped)
    )
}

pub struct Lifecycle {
    current: State,
    history: Vec<(State, State)>,
    audit: Box<dyn FnMut(State, State) + Send>,
}

impl Lifecycle {
    pub fn new() -> Self {
        Self {
            current: State::Init,
            history: Vec::new(),
            audit: Box::new(|_, _| {}),
        }
    }

    /// Startzustand für Tests / Recovery (definiert, nicht prived).
    pub fn at(state: State) -> Self {
        Self {
            current: state,
            history: Vec::new(),
            audit: Box::new(|_, _| {}),
        }
    }

    pub fn with_audit(audit: Box<dyn FnMut(State, State) + Send>) -> Self {
        let mut lc = Self::new();
        lc.audit = audit;
        lc
    }

    pub fn current(&self) -> State {
        self.current
    }

    pub fn history(&self) -> &[(State, State)] {
        &self.history
    }

    /// Übergang ausführen; illegale Übergänge sind Fehler (fail-closed).
    pub fn transition(&mut self, to: State) -> Result<State, LifecycleError> {
        if !can_transition(self.current, to) {
            return Err(LifecycleError {
                from: self.current,
                to,
            });
        }
        let from = self.current;
        self.history.push((from, to));
        (self.audit)(from, to);
        self.current = to;
        Ok(self.current)
    }
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self::new()
    }
}
