
//use std::process::{ExitCode, Termination};

pub fn create_grid() -> Result<i8, String> {
    // Call Create grid method

    return Ok(0);
}

pub fn compute_next_generation() -> Result<i8, String> {
    // Run the game and get the return value of the run_game()

    return Ok(0);
}

pub fn get_no_of_live_neighbours() -> Result<i8, String> {

    return Ok(0);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_grid_creation() {
        let result = create_grid();
        assert_eq(result, 0);
    }

    #[test]
    fn check_run_game() {
        let result = compute_next_generation();
        assert_eq(result, 0);
    }

    #[test]
    fn check_get_no_of_live_neighbours() {
        let result = run_game();
        assert_eq(result, 0);
    }
}
