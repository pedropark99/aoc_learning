use std::fs;
// ;({where()+'what()mul(445,324)#what()select()(+mul(430,603)

struct MultiplyOp {
    left_operand: i32,
    right_operand: i32,
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

fn is_transition_possible(input: &Vec<u8>, idx: usize, state: State) -> (bool, State) {
    let next_chr = input[idx + 1];
    match state {
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
                return (true, State::StParClose);
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

fn try_find_end_index(input: &Vec<u8>, idx: usize, state: &State) -> Result<usize, &'static str> {
    let result = is_transition_possible(input, idx, state.clone());
    let is_possible = result.0;
    let target_transition = result.1;

    if (is_possible) & (matches!(state, State::StParClose)) {
        // End index found! Success!
        return Ok(idx);
    }

    if is_possible {
        return try_find_end_index(input, idx + 1, &target_transition);
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
            Ok(idx) => {
                println!("Found end index: {idx:?}");
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
    parse_input(input_text.as_bytes().to_vec());
    println!("Hello, world!");
}
