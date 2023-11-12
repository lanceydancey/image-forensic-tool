use std::io;

// Import the Board struct from your library crate.
use chomp::Board;

fn main() {
    println!("Welcome to the Chomp game!");

    // Ask the user for the board size.
    let (rows, cols) = get_board_size_from_user();

    // Create a new Board instance based on user input.
    let mut board: Board = Board::new(rows, cols);

    // You can now use the 'board' instance to play the game.
    // Implement the game logic here.
    loop {
        // Display the current state of the chocolate bar.
        println!("Current chocolate bar:");
        board.display();

        // Ask the current player for a move.
        let (row, col) = get_player_move(&board);

        // Check if the move is valid (e.g., square is not chomped).
        if board.chocolate_bar.contains(&(row, col)) {
            board.chomper(row, col);
        } else {
            println!("Invalid move. Please choose an unchomped square.");
            continue;
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
}
