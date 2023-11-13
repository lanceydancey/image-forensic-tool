use std::io;

use chomp::Board;
use std::thread;
use std::time;

fn main() {
    println!("Welcome to the Chomp game!");

    let (rows, cols) = get_board_size_from_user();
    let mut board: Board = Board::new(rows, cols);
    let mut player_turn = true;

    loop {
        println!("Current chocolate bar:");
        board.display();

        if player_turn {
            let (row, col) = get_player_move(&board);

            if board.chocolate_bar.contains(&(row, col)) {
                board.chomper(row, col);
            } else {
                println!("Invalid move. Please choose an unchomped square.");
                continue;
            }
        } else {
            println!("The computer is thinking...");
            thread::sleep(time::Duration::from_secs(2)); // Pause for 2 seconds

            match board.winning_move() {
                Some((row, col)) => {
                    board.chomper(row, col);
                }
                None => {
                    let bottom_right_square = board
                        .chocolate_bar
                        .iter()
                        .max_by(|&&(r1, c1), &&(r2, c2)| r1.cmp(&r2).then(c1.cmp(&c2)));

                    if let Some(&(row, col)) = bottom_right_square {
                        board.chomper(row, col);
                    }
                }
            }
        }
        if board.chocolate_bar.len() == 1 && board.chocolate_bar.contains(&(1, 1)) {
            println!("Current chocolate bar:");
            board.display();
            if player_turn {
                println!("Computer loses! The player wins.");
            } else {
                println!("Player loses! The computer wins.");
            }
            break;
        }
        player_turn = !player_turn;
    }
}

fn get_board_size_from_user() -> (usize, usize) {
    loop {
        println!("Please enter the number of rows and columns for the board (e.g., '4 5'): ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let dimensions: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if dimensions.len() != 2 {
            println!("Invalid input. Please enter two numbers separated by a space.");
            continue;
        }

        let (rows, cols) = (dimensions[0], dimensions[1]);

        if rows < 1 || cols < 1 {
            println!("Invalid dimensions. Please enter positive integers.");
            continue;
        }

        return (rows, cols);
    }
}

fn get_player_move(board: &Board) -> (usize, usize) {
    loop {
        println!("Player's turn - Enter row and column (e.g., '2 3'): ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let coords: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if coords.len() != 2 {
            println!("Invalid input. Please enter row and column.");
            continue;
        }

        let (row, col) = (coords[0], coords[1]);

        if board.chocolate_bar.contains(&(row, col)) {
            return (row, col);
        } else {
            println!("Invalid move. Please choose an unchomped square.");
        }
    }
}
