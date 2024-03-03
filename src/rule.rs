use fmt::Display;
use std::fmt;
use std::str::FromStr;

pub type RuleState = u32;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Right,
    Left,
    Stop,
}
#[derive(Debug, PartialEq, Eq)]
pub enum RuleParseError {
    InvalidRule,
    InvalidMove,
    InvalidState,
}
#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub struct Rule {
    pub(crate) write: char,
    pub(crate) mov: Move,
    pub(crate) next_state: RuleState,
}
impl Rule {
    pub fn new(write: char, mov: Move, next_state: RuleState) -> Rule {
        Rule {
            write,
            mov,
            next_state,
        }
    }

    pub fn write(&self) -> char {
        self.write
    }
    pub fn mov(&self) -> Move {
        self.mov
    }
    pub fn next_state(&self) -> RuleState {
        self.next_state
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Right => write!(f, ">"),
            Move::Left => write!(f, "<"),
            Move::Stop => write!(f, "!"),
        }
    }
}
impl Move {
    pub fn is_terminal(&self) -> bool {
        return *self == Move::Stop;
    }
}

impl Display for RuleParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuleParseError::InvalidRule => write!(f, "Invalid rule"),
            RuleParseError::InvalidMove => write!(f, "Invalid move"),
            RuleParseError::InvalidState => write!(f, "Invalid state"),
        }
    }
}
impl FromStr for Rule {
    type Err = RuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let write = chars.next().ok_or(RuleParseError::InvalidRule)?;
        let mov = match chars.next().ok_or(RuleParseError::InvalidMove)? {
            '>' => Move::Right,
            '<' => Move::Left,
            '!' => Move::Stop,
            _ => return Err(RuleParseError::InvalidRule),
        };
        let next_state = chars.as_str().parse().map_err(|_| RuleParseError::InvalidState)?;
        Ok(Rule::new(write, mov, next_state))
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.write, self.mov, self.next_state)
    }
}

#[cfg(test)]
mod test {
    use crate::{Move, Rule};
    use crate::rule::RuleParseError;

    #[test]
    fn test_rule_from_str() {
        assert_eq!("a>1".parse::<Rule>().unwrap(), Rule::new('a', Move::Right, 1));
        assert_eq!("a>1123".parse::<Rule>().unwrap(), Rule::new('a', Move::Right, 1123));
        assert_eq!("a!1123".parse::<Rule>().unwrap(), Rule::new('a', Move::Stop, 1123));
        assert_eq!(" <0".parse::<Rule>().unwrap(), Rule::new(' ', Move::Left, 0));
        assert_eq!(" <0".parse::<Rule>().unwrap(), Rule::new(' ', Move::Left, 0));
        assert_eq!("".parse::<Rule>().unwrap_err(), RuleParseError::InvalidRule);
        assert_eq!("qwe".parse::<Rule>().unwrap_err(), RuleParseError::InvalidRule);
        assert_eq!("<".parse::<Rule>().unwrap_err(), RuleParseError::InvalidMove);
        assert_eq!("<<".parse::<Rule>().unwrap_err(), RuleParseError::InvalidState);
        assert_eq!("a< 123".parse::<Rule>().unwrap_err(), RuleParseError::InvalidState);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Rule::new('a', Move::Right, 1)), "a>1");
        assert_eq!(format!("{}", Rule::new(' ', Move::Left, 0)), " <0");
        assert_eq!(format!("{}", Rule::new(' ', Move::Stop, 0)), " !0");
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(format!("{}", RuleParseError::InvalidRule), "Invalid rule");
        assert_eq!(format!("{}", RuleParseError::InvalidMove), "Invalid move");
        assert_eq!(format!("{}", RuleParseError::InvalidState), "Invalid state");
    }
}

