use std::cmp::{max, Ordering};
use std::fmt;
use std::str::FromStr;
use crate::rule::{Move, Rule};


const SPACE: char = '_';

#[derive(Clone)]
pub struct Tape {
    data: Vec<char>,
    index: isize,
    head_offset: isize,
}

impl Tape {
    pub fn new(data: Vec<char>, head: isize, data_start_at: isize) -> Tape {
        // head0 is the head position relative to the data_start_at.
        let head0 = head - data_start_at;
        let data_len = data.len();
        // head_offset is the head position relative to the data. It is used to calculate the index of the data.
        // head_offset = min(head, data_start_at)
        match head0.cmp(&0) {
            // padding right with spaces when head is equal to data_start_at and data is empty.
            Ordering::Equal => Tape { data: if data.is_empty() { vec![SPACE] } else { data }, index: 0, head_offset: head },
            // padding left with spaces when head is less than data_start_at.
            // ex: head is 0, data_start_at is 3, head0 is -3. Then we need to add 3 spaces to the left of the data.
            // So, head is still 0 and points to the data at index 0 (head - head_offset(which inited as head) = 3 - 3).
            Ordering::Less => Tape { data: [vec![SPACE; -head0 as usize], data].concat(), index: 0, head_offset: head },
            // padding right with spaces when head is greater than data_start_at + len(data).
            // ex: head is 6, data_start_at is 3, len(data) is 1, head0 is 3.
            // Then we need to add max(0, head0 + 1 - len(data)) = max(0, 3+1-1) = 3 spaces to the right of the data. new len(data) is 4
            // So, head is still 6 and points to the data at index 3 (head - head_offset(which is data_start_at) = 6 - 3 = 3).
            Ordering::Greater => Tape { data: [data, vec![SPACE; max(0, head0 + 1 - data_len as isize) as usize]].concat(), index: head0, head_offset: data_start_at },
        }
    }

    pub fn read(&self) -> char {
        self.data.get(self.index as usize).unwrap_or(&SPACE).clone()
    }

    pub(crate) fn apply_rule(&mut self, rule: &Rule) {
        self.write(rule.write);
        self.move_head(&rule.mov);
        self.extend();
    }

    pub fn head(&self) -> isize {
        self.index - self.head_offset
    }

    pub fn index(&self) -> usize {
            self.index as usize
    }

    pub fn data(&self) -> &Vec<char> {
        &self.data
    }

    pub fn set_head(&mut self, head: isize) {
        self.index = head + self.head_offset;
        self.extend();
    }

    fn extend(&mut self) {
        match (self.index.cmp(&0), self.data.len().cmp(&((self.index + 1) as usize))) {
            // padding left with spaces when index is less than 0.
            // And increment head_offset by 1 (index must be gt 0 always).
            (Ordering::Less, _) => {
                self.data = [vec![SPACE; -self.index as usize], self.data.clone()].concat();
                self.head_offset += self.index;
                self.index = 0;
            }
            // padding right with spaces when index is greater than data.len()-1.
            (Ordering::Greater, Ordering::Less) => {
                self.data = [self.data.clone(), vec![SPACE; (self.index - self.data.len() as isize + 1) as usize]].concat();
            }
            // do nothing when index is between 0 and data.len()-1.
            (_, _) => (),
        }
    }

    fn write(&mut self, c: char) {
        self.data[self.index as usize] = c;
    }

    fn move_head(&mut self, mov: &Move) {
        match mov {
            Move::Right => self.index += 1,
            Move::Left => self.index -= 1,
            Move::Stop => (),
        }
    }
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tape = self.data.iter().enumerate().map(|(i, c)| {
            if i == self.index as usize {
                format!("[{}]", c)
            } else {
                format!(" {} ", c)
            }
        }).collect::<Vec<String>>().join("");
        write!(f, "{}", tape)
    }
}

impl FromStr for Tape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tape::new(s.trim().chars().collect(), 0, 0))
    }
}

#[cfg(test)]
mod test {
    use crate::{Move, Rule, Tape};
    use crate::tape::SPACE;

    #[test]
    fn test_tape_filled() {
        let mut tape = Tape::new("12345678".chars().collect(), 4, 0);
        assert_eq!(tape.read(), '5');
        tape.apply_rule(&Rule::new('9', Move::Right, 0));
        assert_eq!(tape.read(), '6');
        tape.apply_rule(&Rule::new('0', Move::Left, 0));
        assert_eq!(tape.read(), '9');
        tape.apply_rule(&Rule::new('-', Move::Stop, 0));
        assert_eq!(tape.read(), '-');
    }

    #[test]
    fn test_tape_empty() {
        test_empty_tape(Tape::new("".chars().collect(), 4, -1));
        test_empty_tape(Tape::new("A".chars().collect(), 4, -1));
        test_empty_tape(Tape::new("".chars().collect(), -10, 1));
        test_empty_tape(Tape::new("A".chars().collect(), -10, 1));
        test_empty_tape(Tape::new("".chars().collect(), 0, 0));
        let mut tape = Tape::new("".chars().collect(), -10, -1);
        tape.set_head(15);
        test_empty_tape(tape);
    }

    fn test_empty_tape(mut tape: Tape) {
        assert_eq!(tape.read(), SPACE);
        tape.apply_rule(&Rule::new('h', Move::Right, 0));
        assert_eq!(tape.read(), SPACE);
        tape.apply_rule(&Rule::new('i', Move::Right, 0));
        assert_eq!(tape.read(), SPACE);
        tape.apply_rule(&Rule::new(SPACE, Move::Left, 0));
        assert_eq!(tape.read(), 'i');
        tape.apply_rule(&Rule::new('i', Move::Left, 0));
        assert_eq!(tape.read(), 'h');
        tape.apply_rule(&Rule::new('h', Move::Left, 0));
        assert_eq!(tape.read(), SPACE);
        tape.apply_rule(&Rule::new(SPACE, Move::Stop, 0));
        assert_eq!(tape.read(), SPACE);
    }
}
