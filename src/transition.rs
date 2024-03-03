use crate::rule::{Rule, RuleState};
use crate::tape::Tape;

#[derive(Clone)]
pub struct Transition {
    state: RuleState,
    tape: Tape,
    pub(crate) rule: Rule,
}

impl Transition {
    pub(crate) fn new(state: RuleState, tape: Tape, rule: Rule) -> Transition {
        Transition {
            state,
            tape,
            rule,
        }
    }

    pub fn state(&self) -> &RuleState {
        &self.state
    }

    pub fn tape(&self) -> &Tape {
        &self.tape
    }

    pub fn rule(&self) -> &Rule {
        &self.rule
    }
}
