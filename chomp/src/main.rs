use std::io;

// Import the Board struct from your library crate.
use chomp::Board;
use std::thread;
use std::time;

fn main() {
    println!("Welcome to the Chomp game!");

    // Ask the user for the board size.
    let (rows, cols) = get_board_size_from_user();

    // Create a new Board instance based on user input.
    let mut board: Board = Board::new(rows, cols);
    let mut player_turn = true;

    // You can now use the 'board' instance to play the game.
    // Implement the game logic here.
    loop {
        // Display the current state of the chocolate bar.
        println!("Current chocolate bar:");
        board.display();

        if player_turn {
            // Ask the current player for a move.
            let (row, col) = get_player_move(&board);

            // Check if the move is valid (e.g., square is not chomped).
            if board.chocolate_bar.contains(&(row, col)) {
                board.chomper(row, col);
            } else {
                println!("Invalid move. Please choose an unchomped square.");
                continue;
            }
        } else {
            //computer turn
            println!("The computer is thinking...");
            thread::sleep(time::Duration::from_secs(2)); // Pause for 2 seconds

            match board.winning_move() {
                Some((row, col)) => {
                    board.chomper(row, col);
                }
                None => {
                    // Find the most bottom-right square in the chocolate_bar
                    let bottom_right_square = board
                        .chocolate_bar
                        .iter()
                        .max_by(|&&(r1, c1), &&(r2, c2)| r1.cmp(&r2).then(c1.cmp(&c2)));

                    if let Some(&(row, col)) = bottom_right_square {
                        board.chomper(row, col);
                    } else {
                        // Handle the case where there are no squares left to chomp
                        // This might be an end-game scenario
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

        // Implement validation logic to ensure the move is valid (e.g., square is not chomped).
        // You can also add additional checks for the validity of the move.

        if board.chocolate_bar.contains(&(row, col)) {
            return (row, col);
        } else {
            println!("Invalid move. Please choose an unchomped square.");
        }
    }
}
