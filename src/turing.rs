use std::error::Error;
use std::fmt::Display;
use crate::rule::{RuleState};
use crate::tape::Tape;
use crate::transition::Transition;
use crate::turing::TuringError::RuleNotFound;
use crate::ruleset::{Ruleset, RulesetError};

pub struct Turing {
    state: RuleState,
    tape: Tape,
    rules: Ruleset,
}

#[derive(Debug)]
pub enum TuringError {
    RuleNotFound {
        rule_error: RulesetError,
    },
}


impl Turing {
    pub fn new(tape: Tape, state: RuleState, rules: Ruleset) -> Turing {
        Turing { state, tape, rules }
    }

    pub fn next_transition(&self) -> Result<Transition, TuringError> {
        let current_symbol = self.tape.read();
        let rule = self.rules.find(&self.state, &current_symbol).map_err(|e| RuleNotFound { rule_error: e})?;
        Ok(Transition::new(self.state, self.tape.clone(), rule))
    }

    pub fn apply_transition(&mut self, transition: &Transition) {
        self.state = transition.rule.next_state;
        self.tape.apply_rule(&transition.rule);
    }

    pub fn tape (&self) -> &Tape {
        &self.tape
    }

    pub fn ruleset (&self) -> &Ruleset {
        &self.rules
    }

    pub fn state(&self) -> RuleState {
        self.state
    }
}

impl Display for TuringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuleNotFound { rule_error } => write!(f, "Rule not found: {}", rule_error),
        }
    }
}

impl Error for TuringError {}

#[cfg(test)]
mod test {
    use crate::ruleset::Ruleset;
    use crate::tape::Tape;
    use crate::turing::Turing;
    use std::str::FromStr;

    #[test]
    fn test_turing() {
        // calculating 5x+y. On the tape in writes as "x+y"
        let rules = Ruleset::from_str(
            "\
|  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 1 | 5<0 | 6<0 | 7<0 | 8<0 | 9<0 | 0<6 | 1<6 | 2>8 | 1>8 | 1>8 | 1!0 |
| 2 | 0<1 | 1<1 | 2<1 | 3<1 | 4<1 | 1<6 | 2<6 | 3>8 | 2>8 | 2>8 | 2!0 |
| 3 | 5<1 | 6<1 | 7<1 | 8<1 | 9<1 | 2<6 | 3<6 | 4>8 | 3>8 | 3>8 | 3!0 |
| 4 | 0<2 | 1<2 | 2<2 | 3<2 | 4<2 | 3<6 | 4<6 | 5>8 | 4>8 | 4>8 | 4!0 |
| 5 | 5<2 | 6<2 | 7<2 | 8<2 | 9<2 | 4<6 | 5<6 | 6>8 | 5>8 | 5>8 | 5!0 |
| 6 | 0<3 | 1<3 | 2<3 | 3<3 | 4<3 | 5<6 | 6<6 | 7>8 | 6>8 | 6>8 | 6!0 |
| 7 | 5<3 | 6<3 | 7<3 | 8<3 | 9<3 | 6<6 | 7<6 | 8>8 | 7>8 | 7>8 | 7!0 |
| 8 | 0<4 | 1<4 | 2<4 | 3<4 | 4<4 | 7<6 | 8<6 | 9>8 | 8>8 | 8>8 | 8!0 |
| 9 | 5<4 | 6<4 | 7<4 | 8<4 | 9<4 | 8<6 | 9<6 | 0<7 | 9>8 | 9>8 | 9!0 |
| 0 | 0<0 | 1<0 | 2<0 | 3<0 | 4<0 | 9<5 | 0<6 | 1>8 | 0>8 | +>9 | 0!0 |
| + | +!0 | +!0 | +!0 | +!0 | +!0 | _<10 | +<7 | +<7 | +>9 | +>9 | _<10 |
| _ | _>8 | 1<0 | 2<0 | 3<0 | 4<0 | _!0 | _!0 | 1>8 | _<5 | _<5 | _!0 |\
").expect("Invalid ruleset");
        let tape = Tape::new("123+19".chars().collect(), 2, 0);
        let mut turing = Turing::new(tape, 0, rules);
        let mut limit = 1000;
        while limit > 0 {
            let transition = turing.next_transition().expect("Error in transition");
            turing.apply_transition(&transition);
            if transition.rule().mov().is_terminal() {
                break;
            }
            limit -= 1;
        }
        let tape = turing.tape();
        assert_eq!(tape.data().iter().collect::<String>(), "_634____");
        assert_eq!(limit, 1000-170);
    }
}