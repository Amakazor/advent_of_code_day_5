use std::collections::VecDeque;
use std::fs::read_to_string;

mod test;

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: usize
}

impl Move {
    fn new(data: &str) -> Move {
        let parts = data.split_ascii_whitespace().collect::<Vec<_>>();
        return Move {
            from: parts[3].parse::<usize>().unwrap_or(0), 
            to: parts[5].parse::<usize>().unwrap_or(0), 
            amount: parts[1].parse::<usize>().unwrap_or(0) 
        }
    }
}

fn split_file_into_sections(data: String) -> (Vec<String>, String, Vec<String>) {
    let result = data.lines().collect::<Vec<_>>();
    let split_index = result.iter().position(|&el| el == "").unwrap_or(0);
    let split = result.split_at(split_index);

    let crate_lines = split.0[0..split.0.len()-1].iter().map(|&el| el.to_owned()).collect::<Vec<_>>();
    let column_line = split.0[split.0.len()-1].to_owned();
    let operation_lines = split.1[1..split.1.len()].iter().map(|&el| el.to_owned()).collect::<Vec<_>>();
    
    return (crate_lines, column_line, operation_lines)
}

fn get_amount_of_columns(data: &str) -> usize {
    return data.chars().nth_back(1).unwrap_or('0').to_string().parse::<usize>().unwrap_or(0)
}

fn get_crate_stacks(data: &Vec<String>, columns: usize)->Vec<VecDeque<char>> {
    let crates = data.iter()
        .map(|datum| datum.chars().collect::<Vec<_>>().chunks(4).map(|chars| chars.iter().collect::<String>()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .iter()
        .map(|strings| strings.iter().map(|str| str.chars().filter(|char| char.is_ascii_alphabetic()).next().unwrap_or(' ')).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut crate_stacks:Vec<VecDeque<char>> = Vec::with_capacity(columns);
    for index in 0..columns {
        crate_stacks.insert(index, VecDeque::with_capacity(crates.len()));
    }

    for line in crates {
        for (column_index, &column) in line.iter().enumerate() {
            if column != ' ' {
                crate_stacks[column_index].push_back(column)
            }
        }
    }
    
    return crate_stacks;
}

fn get_moves(data: &Vec<String>) -> Vec<Move> {
    return data.iter().map(|datum| Move::new(datum)).collect()
}

fn move_cargo_one(movement: &Move, crate_stacks: &mut Vec<VecDeque<char>>) {
    for _ in 0..movement.amount {
        let popped_crate = crate_stacks[movement.from-1].pop_front().unwrap();
        crate_stacks[movement.to-1].push_front(popped_crate)
    }
}

fn move_cargo_multiple(movement: &Move, crate_stacks: &mut Vec<VecDeque<char>>) {
    let mut popped_crates: VecDeque<char> = VecDeque::with_capacity(movement.amount);
    
    for _ in 0..movement.amount {
        popped_crates.push_front(crate_stacks[movement.from-1].pop_front().unwrap());
    }

    for popped_crate in popped_crates {
        crate_stacks[movement.to-1].push_front(popped_crate)
    }
}

fn move_cargo(movements: &Vec<Move>, crate_stacks: &mut Vec<VecDeque<char>>, mover:fn(&Move, &mut Vec<VecDeque<char>>)) {
    for movement in movements {
        mover(movement, crate_stacks)
    }
}

fn get_tops(crate_stacks: &Vec<VecDeque<char>>) -> String {
    return crate_stacks.iter().map(|item| item.iter().next().unwrap_or(&' ')).filter(|&&item| item != ' ').collect();
}

fn main() {
    let data = read_to_string("final-data.txt").unwrap();
    
    let (crate_lines, column_line, operation_lines) = split_file_into_sections(data);
    let columns = get_amount_of_columns(column_line.as_str());
    let mut crate_stacks = get_crate_stacks(&crate_lines, columns);
    let mut crate_stacks_multiple = crate_stacks.clone();
    let moves = get_moves(&operation_lines);
    
    println!("{}", format!("{moves:?}"));

    println!("{:?}", crate_stacks);

    move_cargo(&moves, &mut crate_stacks, move_cargo_one);
    move_cargo(&moves, &mut crate_stacks_multiple, move_cargo_multiple);

    println!("With singular moves: {:?}", crate_stacks);
    println!("With multiple moves: {:?}", crate_stacks_multiple);
    
    println!("With singular moves: {}", get_tops(&crate_stacks));
    println!("With multiple moves: {}", get_tops(&crate_stacks_multiple));
}
