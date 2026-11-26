use text_io::scan;
use text_to_ascii_art::to_art;
use game_of_life_logic as gol;

// Function to print the board in the terminal, using the Grid library
pub fn get_grid_string(game: &gol::GameOfLife) -> String {
    let mut result = String::new();
    for row in game.board.iter() {
        for &cell in row.iter() {
            if cell {
                result.push('🟩');
            } else {
                result.push('⬜');
            }
        }
        result.push('\n');
    }
    result
}

// Function to print a headline as ASCII art with text_to_ascii_art
pub fn print_headline(headline: &str) {
    let ascii_art = to_art(headline.to_string(), "standard", 0, 1, 0);
    match ascii_art {
        Ok(art) => println!("{}", art),
        Err(e) => eprintln!("Error while converting to ASCII art: {}", e),
    }
}

// Function that allows the user to create a custom board with x and o's
pub fn user_creates_custom_board(size: usize) -> Vec<Vec<bool>> {
    let mut board = vec![vec![false; size]; size];
    println!("Please enter {} characters for each row. Use 'x' for a living cell and 'o' for a dead cell.", size);
    println!("You will have to enter {} rows. EXAMPLE: 'ooooo' ENTER 'oxxxo' ENTER 'ooooo' ENTER", size);
    for i in 0..size {
        let row: String;
        scan!("{}", row);
        for (j, c) in row.chars().enumerate() {
            if c == 'x' {
                board[i][j] = true;
            }
        }
    }
    return board;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_grid_string_only_borrows_the_game() {
        let mut game = gol::create_game(2);
        game.insert_cell(0, 1);

        let rendered = get_grid_string(&game);

        // Rendering must not consume the game: we still own it afterwards.
        assert_eq!(game.board.len(), 2);
        assert!(game.board[0][1]);

        let lines: Vec<&str> = rendered.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "⬜🟩");
    }
}
