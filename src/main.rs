mod age_calculator;
use age_calculator::guess_birth_year;
mod guess_game;
use guess_game::guess_number;

fn main() {
    guess_birth_year();
    guess_number();
}