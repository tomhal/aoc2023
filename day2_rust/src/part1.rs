use crate::data::{read_input, Limits, DATA};

pub fn run() {
    let games = read_input(DATA);

    let limits = Limits::new(12, 13, 14);

    let mut sum = 0u32;

    for game in games.iter() {
        if game.is_possible(&limits) {
            sum += game.id;
        }
    }

    println!("Part 1: Sum = {sum}");
}

#[cfg(test)]
mod tests {
    use crate::data::read_input;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sum_possible() {
        let games_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = read_input(games_input);

        let limits = Limits::new(12, 13, 14);

        assert_eq!(true, games[0].is_possible(&limits));
        assert_eq!(true, games[1].is_possible(&limits));
        assert_eq!(false, games[2].is_possible(&limits));
        assert_eq!(false, games[3].is_possible(&limits));
        assert_eq!(true, games[4].is_possible(&limits));
    }
}
