use std::io::{self, Read};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug, Copy, Clone)]
struct Transition {
    pub write_symbol: bool,
    pub direction: Direction,
    pub next_state: usize,
}

struct TuringMachine {
    left_tape: Vec<bool>,
    right_tape: Vec<bool>,
    head_position: i64,
    current_state: usize,
    final_state: usize,
    transistions: Vec<(Transition, Transition)>,
}

impl TuringMachine {
    pub fn new(transitions: Vec<(Transition, Transition)>) -> TuringMachine {
        TuringMachine {
            left_tape: Vec::new(),
            right_tape: vec![false],
            head_position: 0,
            current_state: 0,
            final_state: 'h' as usize - 'A' as usize,
            transistions: transitions,
        }
    }

    pub fn get_state_n(&self) -> usize {
        return self.transistions.len();
    }

    fn get_current_sign(&self) -> bool {
        if self.head_position >= 0 {
            self.right_tape[self.head_position as usize]
        } else {
            self.left_tape[(-self.head_position - 1) as usize]
        }
    }

    fn write_sign(&mut self, position: i64, sign: bool) {
        if self.head_position >= 0 {
            let _ = std::mem::replace(&mut self.right_tape[position as usize], sign);
        } else {
            let _ = std::mem::replace(&mut self.left_tape[(-position - 1) as usize], sign);
        }
    }

    fn count_non_blank(&self) -> u64 {
        self.left_tape
            .iter()
            .map(|&x| if x { 1 } else { 0 })
            .sum::<u64>()
            + self
                .right_tape
                .iter()
                .map(|&x| if x { 1 } else { 0 })
                .sum::<u64>()
    }

    fn check_tape_size(&mut self) {
        if self.head_position >= 0 {
            let position = self.head_position as usize;
            if self.right_tape.len() == position {
                self.right_tape.push(false);
            }
        } else {
            let position = (-self.head_position - 1) as usize;
            if self.left_tape.len() == position {
                self.left_tape.push(false);
            }
        }
    }

    fn make_transition(&mut self) {
        let current_sign = self.get_current_sign();
        let transistions = self.transistions
                                .get(self.current_state)
                                .unwrap_or_else(|| panic!("All states should have delcared transitions. No transition for state number {}", self.current_state));
        let transition = if current_sign {
            transistions.1
        } else {
            transistions.0
        };

        self.write_sign(self.head_position, transition.write_symbol);

        self.head_position += match transition.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        self.current_state = transition.next_state;
    }

    pub fn run(&mut self) -> (u64, u64) {
        let mut step_counter: u64 = 0;
        loop {
            if self.current_state == self.final_state {
                return (step_counter, self.count_non_blank());
            }

            self.make_transition();
            self.check_tape_size();
            step_counter += 1;

            if step_counter % 10_000_000 == 0 {
                println!("Steps taken {}", step_counter);
            }
        }
    }
}

fn get_transition_sign(sign: Option<char>) -> bool {
    match sign {
        Some('1') => true,
        Some('0') => false,
        Some(_) => panic!("Incorrect write sign for transition. Must be either 0 or 1"),
        _ => panic!("Incorrect write sign for transition. Must exist"),
    }
}

fn get_transition_direction(direction: Option<char>) -> Direction {
    match direction {
        Some('R') => Direction::Right,
        Some('L') => Direction::Left,
        Some(_) => panic!("Incorrect direction for transition. Must be either L or R"),
        _ => panic!("Incorrect direction for transition. Must exist"),
    }
}

fn get_transition_next_state(next_state: Option<char>) -> usize {
    match next_state {
        Some(state) => match state {
            'A'..='Z' | 'h' => state as usize - 'A' as usize,
            _ => panic!("Incorrect direction for transition. Must be a letter from A to Z"),
        },
        _ => panic!("Incorrect direction for transition. Must exist"),
    }
}

fn get_transition(transition: String) -> Transition {
    let mut chars = transition.chars();
    Transition {
        write_symbol: get_transition_sign(chars.next()),
        direction: get_transition_direction(chars.next()),
        next_state: get_transition_next_state(chars.next()),
    }
}

fn read_transitions() -> io::Result<Vec<(Transition, Transition)>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input: Vec<String> = buffer.split_whitespace().map(|s| s.to_string()).collect();

    Ok(input
        .chunks(2)
        .map(|chunk| {
            if chunk.len() < 2 {
                panic!(
                    "Incorrect input. Number of transitions should be even (2 transitions per state)"
                );
            }
            (
                get_transition(chunk[0].clone()),
                get_transition(chunk[1].clone()),
            )
        })
        .collect())
}

fn main() {
    let mut turing_machine = TuringMachine::new(read_transitions().unwrap());
    let (step_n, non_blank_n) = turing_machine.run();
    println!(
        "Finished running busy beaver for {} states",
        turing_machine.get_state_n()
    );
    println!("Non blank symbols: {}", non_blank_n);
    println!("Steps taken: {}", step_n);
}
