use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use crate::rule::{Rule, RuleState};

#[derive(Debug, PartialEq, Eq)]
pub enum RulesetError {
    RuleNotFound {
        state: RuleState,
        symbol: char,
    },
}
#[derive(Debug, PartialEq, Eq)]
pub enum RulesetParseError {
    InvalidRuleset,
    InvalidState {state: String},
    InvalidSymbol { row: usize},
    DuplicateState {state: RuleState},
    DuplicateSymbol {symbol: char},
    InvalidFormat { row: usize, col: usize},
    InvalidRule { row: usize, col: usize, format: String},
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ruleset {
    rules: HashMap<RuleState, HashMap<char, Rule>>,
    alphabet: Vec<char>,
    states: Vec<RuleState>
}

impl Ruleset {
    pub fn find(&self, state: &RuleState, symbol: &char) -> Result<Rule, RulesetError> {
        self.rules
            .get(state)
            .and_then(|m| m.get(symbol))
            .cloned()
            .ok_or(RulesetError::RuleNotFound {state: *state, symbol: *symbol })
    }
    pub fn new(rules: HashMap<RuleState, HashMap<char, Rule>>, alphabet: Vec<char>, states: Vec<RuleState>) -> Ruleset {
        Ruleset {
            rules,
            alphabet,
            states,
        }
    }

    pub fn states(&self) -> &Vec<RuleState> {
        &self.states
    }

    pub fn alphabet(&self) -> &Vec<char> {
        &self.alphabet
    }
}

impl Display for RulesetParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RulesetParseError::InvalidRuleset => write!(f, "Invalid ruleset"),
            RulesetParseError::InvalidState {state} => write!(f, "Invalid state: {}", state),
            RulesetParseError::InvalidSymbol {row} => write!(f, "Invalid symbol in row {}", row),
            RulesetParseError::DuplicateState {state} => write!(f, "Duplicate state: {}", state),
            RulesetParseError::DuplicateSymbol {symbol} => write!(f, "Duplicate symbol: {}", symbol),
            RulesetParseError::InvalidFormat {row, col} => write!(f, "Invalid format in cell [{}, {}]", row, col),
            RulesetParseError::InvalidRule {row, col, format} => write!(f, "Invalid rule format in cell [{}, {}]: {}", row, col, format),
        }
    }
}

impl Display for RulesetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RulesetError::RuleNotFound {state, symbol} => write!(f, "Rule for state \"{}\" and symbol \"{}\" not found", state, symbol),
        }
    }
}



impl FromStr for Ruleset {
    type Err = RulesetParseError;

    /// input is Markdown table, like this:
    /// example:
    /// |   | 0 | 1 | 2 | 3 |
    /// |:-:|:-:|:-:|:-:|:-:|:-:|
    /// | a | a>1 | a<2 | b>3 | a<0 |
    /// | b | _<1 | a>2 | a<3 | a>0 |
    /// | _ | b>2 | _<3 | _>0 | _<1 |
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = HashMap::new();
        let mut alphabet = Vec::new();
        let mut lines = s.lines().skip_while(|l| l.trim().is_empty());
        let header = lines.next().ok_or(RulesetParseError::InvalidRuleset)?;
        let mut states = Vec::new();
        for state in header.trim_end_matches(|c| c == '|').split('|').skip(2).map(|s| s.trim()).collect::<Vec<&str>>() {
            let state = state.parse().map_err(|_| RulesetParseError::InvalidState { state: state.to_string() })?;
            if rules.contains_key(&state) {
                return Err(RulesetParseError::DuplicateState {state});
            }
            rules.insert(state, HashMap::new());
            states.push(state);
        }
        for (ind, line) in lines.enumerate() {
            // skip line after header if it separates header from body
            if ind == 0 && (line.contains(":-") || line.contains("--")) {
                continue;
            }
            let mut cells = line.trim_end_matches(|c| c == '|').split('|').skip(1).take(states.len() + 1);
            let symbol = cells
                .next()
                .ok_or(RulesetParseError::InvalidFormat{ row: ind, col: 0})?
                .trim()
                .parse()
                .map_err(|_| RulesetParseError::InvalidSymbol { row: ind} )?;
            if alphabet.contains(&symbol) {
                return Err(RulesetParseError::DuplicateSymbol { symbol });
            }
            alphabet.push(symbol);
            for (i, cell) in cells.enumerate() {
                let state = states.get(i).ok_or_else(|| RulesetParseError::InvalidFormat { row: ind, col: i})?.clone();
                let rule = cell.trim().parse().map_err(|_| RulesetParseError::InvalidRule { row: ind, col: i, format: cell.to_string() })?;
                rules.get_mut(&state).unwrap().insert(symbol, rule);
            }
        }
        Ok(Ruleset::new(rules, alphabet, states))
    }
}

impl Display for Ruleset {
    /// display rules in the Markdown table format. in every cell format: {write}{move}{next_state}. first column contains char from alphabet, first row contains states (numbers).
    /// example:
    /// |   | 0     | 1     | 2     | 3     |
    /// |:-:|:-:|:-:|:-:|:-:|:-:|
    /// | a | a>1 | a<2 | b>3 | a<0 |
    /// | b | _<1 | a>2 | a<3 | a>0 |
    /// | _ | b>2 | _<3 | _>0 | _<1 |
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut table = String::new();
        table.push_str("|   | ");
        table.push_str(self.rules.keys().map(|s| s.to_string()).collect::<Vec<String>>().join(" | ").as_str());
        table.push_str("|\n|:-:|");
        table.push_str(self.rules.keys().map(|_| ":-:|").collect::<Vec<&str>>().join("").as_str());
        table.push_str("\n");
        for symbol in self.alphabet.iter() {
            table.push_str(&format!("| {} | ", symbol));
            table.push_str(self.rules.keys().map(|state| {
                if let Some(rule) = self.rules.get(state).and_then(|m| m.get(symbol)) {
                    rule.to_string()
                } else {
                    "     ".to_string()
                }
            }).collect::<Vec<String>>().join(" | ").as_str());
            table.push_str("|\n");
        }
        write!(f, "{}", table)
    }
}


#[cfg(test)]
mod test {
    use crate::{Move, Rule, Ruleset, RulesetError};
    use crate::ruleset::RulesetParseError;

    #[test]
    fn test_ruleset_from_str_with_separator1() {
        let ruleset =
"|   | 0     | 1     | 2     | 3   |
 |:-:|:-:    | :---: |:-:    |:-:  |
 | a | a>1   | a<2   | b>3   | a<0 |
 | b | _<1   | a>2   | a<3   | a!0 |";
        check_ruleset(ruleset.parse().unwrap())
    }
    #[test]
    fn test_ruleset_from_str_with_separator2() {
        let ruleset =
"|   | 0     | 1     | 2     | 3   |
 |---|---    | --- |---    |---  |
 | a | a>1   | a<2   | b>3   | a<0 |
 | b | _<1   | a>2   | a<3   | a!0 |";
        check_ruleset(ruleset.parse().unwrap())
    }
    #[test]
    fn test_ruleset_from_str_with_extra_col() {
        let ruleset =
"|   | 0     | 1     | 2     | 3   |
 |---|---    | --- |---    |---  |
 | a | a>1   | a<2   | b>3   | a<0 | a<0 |
 | b | _<1   | a>2   | a<3   | a!0 |";
        check_ruleset(ruleset.parse().unwrap())
    }
    #[test]
    fn test_ruleset_from_str_without_col() {
        let ruleset =
"|   | 0     | 1     | 2     | 3   |
 |---|---    | --- |---    |---  |
 | a | a>1   | a<2   | b>3   |
 | b | _<1   | a>2   | a<3   | a!0 |";
        assert!(ruleset.parse::<Ruleset>().is_ok())
    }
    #[test]
    fn test_ruleset_serde() {
        let ruleset =
"|   | 0     | 1     | 2     | 3   |
 |---|---    | --- |---    |---  |
 | a | a>1   | a<2   | b>3   | a<0 |
 | b | _<1   | a>2   | a<3   | a!0 |";
        let ruleset = ruleset.parse::<Ruleset>().unwrap();
        let out = ruleset.to_string();
        check_ruleset(out.parse().unwrap())
    }

    #[test]
    fn test_ruleset_from_str_without_separator() {
        let ruleset =
"|   | 0     | 1     | 2     | 3   |
 | a | a>1   | a<2   | b>3   | a<0 |
 | b | _<1   | a>2   | a<3   | a!0 |";
        check_ruleset(ruleset.parse().unwrap())
    }
    #[test]
    fn test_ruleset_with_error() {
        let ruleset =
            "|   | 0     | 1     | 2     | 1   |";
        assert_eq!(ruleset.parse::<Ruleset>().unwrap_err(), RulesetParseError::DuplicateState {state: 1});
        let ruleset =
            "|   | 0     | 1     | 2     | 3   |
             | a | aa1   | a<2   | b>3   | a<0 | a<0 |";
        assert_eq!(ruleset.parse::<Ruleset>().unwrap_err(), RulesetParseError::InvalidRule {row: 0, col: 0, format: " aa1   ".to_string()});
    }

    fn check_ruleset(ruleset: Ruleset) {
        assert_eq!(ruleset.rules.len(), 4);
        assert_eq!(ruleset.alphabet, vec!['a', 'b']);
        assert_eq!(ruleset.find(&0, &'a').unwrap(), Rule::new('a', Move::Right, 1));
        assert_eq!(ruleset.find(&0, &'a').unwrap(), Rule::new('a', Move::Right, 1));
        assert_eq!(ruleset.find(&0, &'b').unwrap(), Rule::new('_', Move::Left, 1));
        assert_eq!(ruleset.find(&1, &'a').unwrap(), Rule::new('a', Move::Left, 2));
        assert_eq!(ruleset.find(&1, &'b').unwrap(), Rule::new('a', Move::Right, 2));
        assert_eq!(ruleset.find(&2, &'a').unwrap(), Rule::new('b', Move::Right, 3));
        assert_eq!(ruleset.find(&2, &'b').unwrap(), Rule::new('a', Move::Left, 3));
        assert_eq!(ruleset.find(&3, &'a').unwrap(), Rule::new('a', Move::Left, 0));
        assert_eq!(ruleset.find(&3, &'b').unwrap(), Rule::new('a', Move::Stop, 0));
        assert_eq!(ruleset.find(&4, &'b').unwrap_err(), RulesetError::RuleNotFound {state: 4, symbol: 'b'});
        assert_eq!(ruleset.find(&1, &'c').unwrap_err(), RulesetError::RuleNotFound {state: 1, symbol: 'c'});
    }
}

impl Error for RulesetParseError {}
impl Error for RulesetError {}