use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::exit;

fn main() {
    let protoboard_file = "protoboards.txt";
    let protoboards = parse_protoboards(protoboard_file);

    println!("Available t-values:");
    let mut sorted_keys: Vec<_> = protoboards.keys().copied().collect();
    sorted_keys.sort();
    for t in &sorted_keys {
        println!("t = {}", t);
    }

    println!("Enter t value:");
    let t: u32 = read_input().trim().parse().expect("Invalid t value");

    let boards = protoboards.get(&t).expect("Invalid t selected");

    // Calculate the starting global ID
    let mut start_id = 1;
    for key in sorted_keys.iter().filter(|&&key| key < t) {
        if let Some(prev_boards) = protoboards.get(key) {
            start_id += prev_boards.len();
        }
    }
    let end_id = start_id + boards.len() - 1;

    println!(
        "Valid global IDs for t = {}: [{}..={}] ({} boards)",
        t, start_id, end_id, boards.len()
    );

    // println!("Available global IDs for t = {}: {} boards", t, boards.len());
    println!("Enter global ID:");
    let global_id: usize = read_input().trim().parse().expect("Invalid global ID");

    let (_, proto_matrix) = boards.iter().find(|(id, _)| *id == global_id)
        .unwrap_or_else(|| {
            println!("Invalid global ID.");
            exit(1);
        });

    let filled_tiles = count_filled(proto_matrix);

    println!("Selected board ({} filled tiles):", filled_tiles);
    render_board(proto_matrix);

    println!("Enter base-11 local ID ({} digits expected):", filled_tiles);
    let base11 = read_input().trim().to_uppercase();

    if base11.len() != filled_tiles as usize {
        println!("Invalid base-11 ID length.");
        exit(1);
    }

    let tile_values = parse_base11(&base11);

    let final_board = fill_board(proto_matrix, &tile_values);

    println!("Generated 2048 board:");
    render_board_with_values(&final_board);
}

fn parse_protoboards(filename: &str) -> HashMap<u32, Vec<(usize, Vec<Vec<char>>)>> {
    let mut protoboards = HashMap::new();
    let file = File::open(Path::new(filename)).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut current_t = 0;
    let mut current_id = 0;
    let mut current_board = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.contains("Boards with t =") {
            if !current_board.is_empty() {
                protoboards.entry(current_t).or_insert(Vec::new()).push((current_id, current_board.clone()));
                current_board.clear();
            }
            if let Some(t_val) = line.split('=').nth(1).and_then(|s| s.trim().split(' ').next()).and_then(|s| s.parse::<u32>().ok()) {
                current_t = t_val;
            }
        } else if line.starts_with("Board #") {
            if !current_board.is_empty() {
                protoboards.entry(current_t).or_insert(Vec::new()).push((current_id, current_board.clone()));
                current_board.clear();
            }
            if let Some(id_val) = line.split('#').nth(1).and_then(|s| s.split(' ').next()).and_then(|s| s.parse::<usize>().ok()) {
                current_id = id_val;
            }
        } else if !line.trim().is_empty() {
            let row: Vec<char> = line.chars().filter(|&c| c == 'X' || c == '.').collect();
            if !row.is_empty() {
                current_board.push(row);
            }
        }
    }

    if !current_board.is_empty() {
        protoboards.entry(current_t).or_insert(Vec::new()).push((current_id, current_board.clone()));
    }

    protoboards
}

fn read_input() -> String {
    use std::io::Write;
    let mut input = String::new();
    print!("> ");
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn count_filled(board: &Vec<Vec<char>>) -> usize {
    board.iter().flatten().filter(|&&c| c == 'X').count()
}

fn parse_base11(s: &str) -> Vec<u32> {
    if s.chars().filter(|&c| c == 'B').count() > 1 {
        panic!("Invalid base-11 ID: more than one 'B'");
    }

    s.chars().map(|c| match c {
        '1'..='9' => c.to_digit(11).unwrap(),
        'A' => 10,
        'B' => 11,
        _ => panic!("Invalid base-11 digit"),
    }).collect()
}

fn fill_board(proto: &Vec<Vec<char>>, tiles: &[u32]) -> Vec<Vec<u32>> {
    let mut filled = vec![vec![0; 4]; 4];
    let mut tile_iter = tiles.iter();
    for i in 0..4 {
        for j in 0..4 {
            if proto[i][j] == 'X' {
                filled[i][j] = 2u32.pow(*tile_iter.next().unwrap());
            }
        }
    }
    filled
}

fn render_board(board: &Vec<Vec<char>>) {
    for row in board {
        for &cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn render_board_with_values(board: &Vec<Vec<u32>>) {
    for row in board {
        for &cell in row {
            if cell == 0 {
                print!(". ");
            } else {
                print!("{} ", cell);
            }
        }
        println!();
    }
}
