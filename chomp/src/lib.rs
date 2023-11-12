use std::collections::HashSet;

// Define the Board struct as a HashSet of tuples representing the chocolate bar.
pub struct Board {
    pub chocolate_bar: HashSet<(usize, usize)>,
    pub rows: usize,
    pub cols: usize,
}

impl Board {
    // Constructor to create a new Board with a specified size.
    pub fn new(rows: usize, cols: usize) -> Self {
        // Initialize the chocolate_bar with tuples representing each square.
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

    pub fn display(&self) {
        for row in 1..=self.rows {
            for col in 1..=self.cols {
                let symbol: char = if self.chocolate_bar.contains(&(row, col)) {
                    'X' // Represented by 'X' if not chomped
                } else {
                    ' ' // Represented by space if chomped
                };
                print!("{} ", symbol);
            }
            println!();
        }
    }

    pub fn chomper(&mut self, row: usize, col: usize) {
        // Create a new HashSet to store the squares to remove.
        let mut squares_to_remove = HashSet::new();

        // Iterate through the chocolate_bar and collect squares to remove.
        for &(r, c) in &self.chocolate_bar {
            if r >= row && c >= col {
                squares_to_remove.insert((r, c));
            }
        }

        // Remove the collected squares from the chocolate_bar.
        for square in &squares_to_remove {
            self.chocolate_bar.remove(square);
        }
    }
}
