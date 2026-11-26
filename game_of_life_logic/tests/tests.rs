#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use game_of_life_logic::*;


    //Tests for the game_of_life_logic module
    #[test]
    fn test_update_game() {
        let board = vec![
            vec![true, false, true],
            vec![false, true, false],
            vec![true, false, true],
        ];
        let mut game = create_game(1);
        game.use_custom_board(board.clone(), 3);
        game.update_game();
    
        assert_eq!(game.board, vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ]);
    }

    #[test]
    fn test_insert_cell() {
        // Create a game with a 3x3 board
        let board = vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ];
        let mut game = create_game(1);
        game.use_custom_board(board.clone(), 3);

        // Insert a cell at position (1, 1)
        game.insert_cell(1, 1);

        // Assert that the updated game has the correct board
        assert_eq!(game.board, vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, false],
        ]);
    }

    #[test]
    fn check_stuck_reports_generation_limit() {
        let mut game = create_game(5);
        game.use_random_board(5);

        assert!(!game.check_stuck(0, 10));
        assert!(game.check_stuck(10, 10));
        assert!(game.check_stuck(11, 10));
    }

    #[test]
    fn check_stuck_only_borrows_the_game() {
        let mut game = create_game(3);
        game.use_random_board(3);

        // Inspecting the state must not consume the game: we still own it
        // afterwards and can advance it another generation.
        let _ = game.check_stuck(1, 5);
        game.update_game();
        assert_eq!(game.board.len(), 3);
        assert_eq!(game.game_history.len(), 1);
    }

    #[test]
    fn rotate_stick() {
        // Create a game with a 5x5 board
        let board = vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ];
        let mut game = create_game(1);
        game.use_custom_board(board.clone(), 5);

        // Update the game
        game.update_game();

        // Assert that the updated game has the correct board
        assert_eq!(game.board, vec![
            vec![false, false, false, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, false, false, false],
        ]);
    }
}