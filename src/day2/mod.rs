use std::collections::HashMap;

use crate::common::get_data_from_file_or_str;

#[derive(Debug)]
struct CubeCounts {
    red: usize,
    green: usize,
    blue: usize,
}

// it turns out you need this for tests
impl PartialEq for CubeCounts {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

pub fn part1(path_or_data: String) -> usize {
    let data = get_data_from_file_or_str(path_or_data);
    // @@@ Personal challenge: no regex! @@@

    let limits: HashMap<&str, usize> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    // Okay so id is always idx 1, num. is always even idx, type is always odd idx
    let valid_games: Vec<_> = data
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let is_valid = line
                .split(';')
                .map(|subset| validate_subset(subset, &limits))
                .all(|result| result.is_some());

            if is_valid {
                Some(idx + 1)
            } else {
                None
            }
        })
        .collect();

    let final_num = valid_games.iter().sum();
    println!("The final sum is {}", final_num);
    final_num
}

pub fn part2(path_or_data: String) -> usize {
    let data = get_data_from_file_or_str(path_or_data);
    // @@@ Personal challenge: no regex! @@@

    // Okay so id is always idx 1, num. is always even idx, type is always odd idx
    let game_sums = data
        .lines()
        .enumerate()
        .filter_map(|(_, game)| {
            let mut minimum_amounts_needed = CubeCounts {
                red: 0,
                green: 0,
                blue: 0,
            };

            game
                .split(';')
                .for_each(|round| {
                    if let Some(count) = parse_round(round) {
                        // Update the minimum_amounts_needed only if the count is lower; there is probably a better way to do this
                        if count.red != 0 && count.red > minimum_amounts_needed.red || minimum_amounts_needed.red == 0 {
                            minimum_amounts_needed.red = count.red;
                        }
                        if count.green != 0 && count.green > minimum_amounts_needed.green || minimum_amounts_needed.green == 0 {
                            minimum_amounts_needed.green = count.green;
                        }
                        if count.blue != 0 && count.blue > minimum_amounts_needed.blue || minimum_amounts_needed.blue == 0 {
                            minimum_amounts_needed.blue = count.blue;
                        }
                    }
                });

            // Only return Some if any of the counts is non-zero
            if minimum_amounts_needed.red > 0 || minimum_amounts_needed.green > 0 || minimum_amounts_needed.blue > 0 {
                Some(minimum_amounts_needed.red * minimum_amounts_needed.green * minimum_amounts_needed.blue)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    let final_num = game_sums.iter().sum();
    println!("The final product is {}", final_num);
    final_num
}

fn parse_round(round: &str) -> Option<CubeCounts> {
    let mut counts = CubeCounts {
        red: 0,
        green: 0,
        blue: 0,
    };
    round
        .split(',')
        .filter_map(|color| {
            let parts: Vec<&str> = if color.contains(':') {
                // split_once returns a before and after
                color.split_once(':').map(|x| x.1).unwrap_or("")
            } else {
                // default to the whole string in something like "12 red"
                color
            }
                .split_whitespace()
                // remove game if the color is "game X: Y color"
                .filter(|s| !s.contains("game"))
                .collect();

            if parts.len() != 2 {
                unreachable!("Expected each sequence to have two parts, but got sequence '{:#?}' with parts '{:#?}'", color, parts)
            }

            let cube_amount = match parts[0].parse::<usize>() {
                Ok(n) => n,
                Err(_) => return None,
            };

            Some((parts[1], cube_amount))
        })
        .for_each(|(color, cube_amount)| {
            match color {
                "red" => counts.red += cube_amount,
                "green" => counts.green += cube_amount,
                "blue" => counts.blue += cube_amount,
                _ => unreachable!("Unknown color '{}'", color),
            }
        });

    Some(counts)
}

fn validate_subset(subset: &str, limits: &HashMap<&str, usize>) -> Option<()> {
    if let Some(parsed_subset) = parse_round(subset) {
        if parsed_subset.red > limits["red"] || parsed_subset.green > limits["green"] || parsed_subset.blue > limits["blue"] {
            return None;
        }
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_round() {
        let round = "3 blue, 4 red";
        let result = parse_round(round);
        assert_eq!(result, Some(CubeCounts { red: 4, green: 0, blue: 3 }));
    }

    #[test]
    fn valid_round_with_game_id() {
        let round = "game 1: 3 blue, 4 red";
        let result = parse_round(round);
        assert_eq!(result, Some(CubeCounts { red: 4, green: 0, blue: 3 }));
    }

    #[test]
    fn valid_game() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = part2(game.to_string());
        assert_eq!(result, 48);
    }

    #[test]
    fn valid_games() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part2(game.to_string());
        assert_eq!(result, 2286);
    }
}

// Part 1 tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn valid_game() {
//         let subset = "3 blue, 4 red, 1 red";
//         let limits: HashMap<&str, usize> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
//         let result = validate_subset(subset, &limits);
//         assert_eq!(result, Some(()))
//     }
//
//     #[test]
//     fn invalid_subset() {
//         let subset = "3 blue, 4 red, 1 red";
//         let limits: HashMap<&str, usize> = HashMap::from([("red", 3), ("green", 13), ("blue", 14)]);
//         let result = validate_subset(subset, &limits);
//         assert_eq!(result, None)
//     }
//
//     #[test]
//     fn subset_with_one_color() {
//         let subset = "14 green";
//         let limits: HashMap<&str, usize> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
//         let result = validate_subset(subset, &limits);
//         assert_eq!(result, None)
//     }
//
//     #[test]
//     fn subset_wth_all_colors() {
//         let subset = "9 green, 3 blue, 8 red";
//         let limits: HashMap<&str, usize> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
//         let result = validate_subset(subset, &limits);
//         assert_eq!(result, Some(()))
//     }
//
//     #[test]
//     fn long_game() {
//         let game = "Game 66: 10 blue, 10 green, 5 red; 10 green, 3 blue, 5 red; 1 red, 1 green, 10 blue; 2 green, 5 red, 20 blue; 8 blue, 11 green, 13 red; 2 green, 18 blue, 2 red
// ";
//         let result = part1(game.to_string());
//         assert_eq!(result, 0)
//     }
// }
