use crate::data::{read_input, DATA};

pub fn run() {
    let games = read_input(DATA);

    let mut sum = 0u32;

    for game in games.iter() {
        let limits = game.minumum_limits();
        sum += limits.red * limits.green * limits.blue;
    }

    println!("Part 2: Sum = {sum}");
}

#[cfg(test)]
mod tests {
    use crate::data::{read_input, Limits};

    #[test]
    fn test_minimum_limits() {
        let games_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = read_input(games_input);

        assert_eq!(Limits::new(4, 2, 6), games[0].minumum_limits());
        assert_eq!(Limits::new(1, 3, 4), games[1].minumum_limits());
        assert_eq!(Limits::new(20, 13, 6), games[2].minumum_limits());
        assert_eq!(Limits::new(14, 3, 15), games[3].minumum_limits());
        assert_eq!(Limits::new(6, 3, 2), games[4].minumum_limits());
    }
}
