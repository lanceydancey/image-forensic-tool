use std::collections::HashSet;

// Define the Board struct as a HashSet of tuples representing the chocolate bar.
pub struct Board {
    pub chocolate_bar: HashSet<(usize, usize)>,
}

impl Board {
    // Constructor to create a new Board with a specified size.
    pub fn new(rows: usize, cols: usize) -> Self {
        // Initialize the chocolate_bar with tuples representing each square.
        let mut chocolate_bar = HashSet::new();
        for row in 0..rows {
            for col in 0..cols {
                chocolate_bar.insert((row, col));
            }
        }

        Board { chocolate_bar }
    }

    pub fn display(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let symbol = if self.chocolate_bar.contains(&(row, col)) {
                    'X' // Represented by 'X' if not chomped
                } else {
                    ' ' // Represented by space if chomped
                };
                print!("{} ", symbol);
            }
            println!();
        }
    }
}
