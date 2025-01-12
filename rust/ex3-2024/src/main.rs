use std::{fs, io::BufRead};
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

        let state = State::StM;
        let end_index = try_find_end_index(&input, index, &state);
        match end_index {
            Ok(idx) => {
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
