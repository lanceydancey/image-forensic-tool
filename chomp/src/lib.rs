use std::collections::HashSet;
#[derive(Clone)]
pub struct Board {
    pub chocolate_bar: HashSet<(usize, usize)>,
    pub rows: usize,
    pub cols: usize,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut chocolate_bar: HashSet<(usize, usize)> = HashSet::new();
        for row in 1..=rows {
            for col in 1..=cols {
                chocolate_bar.insert((row, col));
            }
        }

        Board {
            chocolate_bar,
            rows,
            cols,
        }
    }

    /// Displays the current state of the chocolate bar.
    ///
    /// This function prints the board to the console with row and column numbers.
    /// Each square is represented by 'X' if it has not been chomped, or a space if it has.
    ///
    /// # Examples
    ///
    /// ```
    /// let board = Board::new(5, 5);
    /// board.display();
    /// ```
    pub fn display(&self) {
        print!("  ");
        for col in 1..=self.cols {
            print!("{} ", col);
        }
        println!();

        for row in 1..=self.rows {
            print!("{} ", row);
            for col in 1..=self.cols {
                let symbol: char = if self.chocolate_bar.contains(&(row, col)) {
                    'X'
                } else {
                    ' '
                };
                print!("{} ", symbol);
            }
            println!();
        }
    }

    /// Performs a "chomp" at the specified row and column on the chocolate bar.
    ///
    /// Will remove the specified square and all squares to the right and below
    ///
    /// # Arguments
    ///
    /// * `row` - The row number of the square to chomp.
    /// * `col` - The column number of the square to chomp.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Board::new(5, 5);
    /// board.chomper(3, 3);
    /// ```    
    pub fn chomper(&mut self, row: usize, col: usize) {
        let mut squares_to_remove = HashSet::new();

        for &(r, c) in &self.chocolate_bar {
            if r >= row && c >= col {
                squares_to_remove.insert((r, c));
            }
        }

        for square in &squares_to_remove {
            self.chocolate_bar.remove(square);
        }
    }

/// Determines the winning move for the current state of the board, if one exists.
///
/// Uses negamax algorithm to find the best possible move by iterating over all possible moves.
///
/// # Returns
///
/// An `Option<(usize, usize)>` indicating the winning move. It returns `Some((row, col))`
/// if a winning move is found, and `None` if there is no winning move.
///
/// # Examples
///
/// ```
/// let mut board = Board::new(5, 5);
/// if let Some((row, col)) = board.winning_move() {
///     println!("Winning move: ({}, {})", row, col);
/// } else {
///     println!("No winning move available.");
/// }
/// ```
    pub fn winning_move(&mut self) -> Option<(usize, usize)> {
        if self.chocolate_bar.len() == 1 && self.chocolate_bar.contains(&(1, 1)) {
            return None;
        }

        for r in 1..=self.rows {
            for c in 1..=self.cols {
                if (r, c) == (1, 1) || !self.chocolate_bar.contains(&(r, c)) {
                    continue;
                }

                let mut new_board = self.clone();
                new_board.chomper(r, c);

                if new_board.winning_move().is_none() {
                    return Some((r, c)); //
                }
            }
        }

        None
    }
}
