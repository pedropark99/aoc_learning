use std::fs;

struct MultiplyOp {
    left_operand: i32,
    right_operand: i32,
}

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

fn get_transitions(state: &State) -> Vec<State> {
    match state {
        State::StM => {
            let states = vec![State::StU];
            return states;
        }
        State::StU => {
            let states = vec![State::StL];
            return states;
        }
        State::StL => {
            let states = vec![State::StParOpen];
            return states;
        }
        State::StParOpen => {
            let states = vec![State::StNum];
            return states;
        }
        State::StParClose => {
            let states = vec![State::StSuccessMatch];
            return states;
        }
        State::StNum => {
            let states = vec![State::StNum, State::StCom, State::StParClose];
            return states;
        }
        State::StCom => {
            let states = vec![State::StNum];
            return states;
        }
        State::StSuccessMatch => {
            let states = vec![State::StM];
            return states;
        }
    }
}

fn is_transition_possible(input: &Vec<u8>, idx: usize, transition: &State) -> bool {
    let next_chr = input[idx + 1];
    match transition {
        State::StM => {
            if next_chr == b'u' {
                return true;
            }
            return false;
        }
        State::StU => {
            if next_chr == b'l' {
                return true;
            }
            return false;
        }
        State::StL => {
            if next_chr == b'(' {
                return true;
            }
            return false;
        }
        State::StParOpen => {
            let nums = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
            if nums.contains(&next_chr) {
                return true;
            }
            return false;
        }
        State::StNum => {
            let nums = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
            if (nums.contains(&next_chr)) | (next_chr == b',') {
                return true;
            }
            return false;
        }
        State::StCom => {
            let nums = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
            if nums.contains(&next_chr) {
                return true;
            }
            return false;
        }
        State::StParClose | State::StSuccessMatch => {
            return true;
        }
    }
}

fn try_find_end_index(input: &Vec<u8>, idx: usize, state: &State) -> Result<usize, &'static str> {
    for transition in get_transitions(state) {
        if (is_transition_possible(input, idx, &transition)) & matches!(state, State::StParClose) {
            // End index found! Success!
            return Ok(idx);
        }
        if is_transition_possible(input, idx, &transition) {
            return try_find_end_index(input, idx + 1, &transition);
        }
    }
    return Err("Unable to find end index.");
}

fn parse_input(input: Vec<u8>) -> Vec<MultiplyOp> {
    let multipliers: Vec<MultiplyOp> = Vec::new();
    // let mut components: Vec<u8> = Vec::new();
    let mut index: usize = 0;
    for chr in &input {
        if *chr != b'm' {
            index += 1;
            continue;
        }

        let state = State::StM;
        let end_index = try_find_end_index(&input, index, &state);
        match end_index {
            Ok(idx) => println!("Found end index: {idx:?}"),
            Err(_) => {
                continue;
            }
        }
    }
    return multipliers;
}

fn main() {
    const PATH: &str = "ex3-2024/src/input.txt";
    let input_text = fs::read_to_string(PATH).expect("File not found!");
    parse_input(input_text.as_bytes().to_vec());
    println!("Hello, world!");
}
