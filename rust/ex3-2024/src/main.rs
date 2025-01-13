use std::fs;
// ;({where()+'what()mul(445,324)#what()select()(+mul(430,603)

struct MultiplyOp {
    left_operand: u64,
    right_operand: u64,
}

#[derive(Clone)]
enum State {
    StM,
    StU,
    StL,
    StParOpen,
    StParClose,
    StNum,
    StCom,
    StSuccessMatch,
}

struct Stack<T> {
    elems: Vec<T>,
}

impl<T> Stack<T> {
    fn push(&mut self, val: T) {
        self.elems.push(val);
    }

    fn pop(&mut self) -> Option<T> {
        return self.elems.pop();
    }

    fn last(&self) -> Option<&T> {
        return self.elems.last();
    }
}

fn is_transition_possible(input: &Vec<u8>, idx: usize, states: &mut Stack<State>) -> (bool, State) {
    let next_chr = input[idx + 1];
    let last_state = states.last().unwrap();
    match last_state {
        State::StM => {
            if next_chr == b'u' {
                return (true, State::StU);
            }
            return (false, State::StU);
        }
        State::StU => {
            if next_chr == b'l' {
                return (true, State::StL);
            }
            return (false, State::StL);
        }
        State::StL => {
            if next_chr == b'(' {
                return (true, State::StParOpen);
            }
            return (false, State::StParOpen);
        }
        State::StParOpen => {
            let nums = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
            if nums.contains(&next_chr) {
                return (true, State::StNum);
            }
            return (false, State::StNum);
        }
        State::StNum => {
            let nums = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
            if nums.contains(&next_chr) {
                return (true, State::StNum);
            }
            if next_chr == b',' {
                return (true, State::StCom);
            }
            if next_chr == b')' {
                for state in &states.elems {
                    if matches!(state, State::StCom) {
                        return (true, State::StParClose);
                    }
                }
            }

            return (false, State::StNum);
        }
        State::StCom => {
            let nums = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
            if nums.contains(&next_chr) {
                return (true, State::StNum);
            }
            return (false, State::StNum);
        }
        State::StParClose | State::StSuccessMatch => {
            return (true, State::StNum);
        }
    }
}

fn try_find_end_index(
    input: &Vec<u8>,
    idx: usize,
    states: &mut Stack<State>,
) -> Result<usize, &'static str> {
    let result = is_transition_possible(input, idx, states);
    let is_possible = result.0;
    let target_transition = result.1;

    if (is_possible) & (matches!(states.last().unwrap(), State::StParClose)) {
        // End index found! Success!
        return Ok(idx);
    }

    if is_possible {
        states.push(target_transition);
        return try_find_end_index(input, idx + 1, states);
    }

    return Err("Unable to find end index.");
}

fn parse_multipliers(input: &[u8]) -> MultiplyOp {
    let mut comma_index: usize = 0;
    for chr in input {
        if *chr == b',' {
            break;
        }
        comma_index += 1;
    }

    let left_op = String::from_utf8((input[0..comma_index]).to_vec()).unwrap();
    let right_op = String::from_utf8((input[(comma_index + 1)..]).to_vec()).unwrap();
    return MultiplyOp {
        left_operand: left_op.parse::<u64>().unwrap(),
        right_operand: right_op.parse::<u64>().unwrap(),
    };
}

fn parse_input(input: Vec<u8>) -> Vec<MultiplyOp> {
    let mut multipliers: Vec<MultiplyOp> = Vec::new();
    let mut index: usize = 0;
    for chr in &input {
        if *chr != b'm' {
            index += 1;
            continue;
        }

        let mut states_vec: Vec<State> = Vec::new();
        states_vec.push(State::StM);
        let mut states = Stack { elems: states_vec };
        let end_index = try_find_end_index(&input, index, &mut states);
        match end_index {
            Ok(idx) => {
                println!(
                    "Parsing: {}",
                    String::from_utf8((&input[(index)..idx]).to_vec()).unwrap()
                );
                multipliers.push(parse_multipliers(&input[(index + 4)..idx]));
                index += 1;
            }
            Err(_) => {
                index += 1;
                continue;
            }
        }
    }
    return multipliers;
}

fn main() {
    const PATH: &str = "ex3-2024/src/input.txt";
    let input_text = fs::read_to_string(PATH).expect("File not found!");
    let multipliers = parse_input(input_text.as_bytes().to_vec());

    let mut total_value: u64 = 0;
    for mult in multipliers {
        total_value += mult.left_operand * mult.right_operand;
    }

    println!("Total value: {}", total_value);
}
