use std::env;
use std::fs::read_to_string;
use std::str;

// answer: 2207
// answer pt2: 62241

#[derive(PartialEq, Debug)]
struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

impl Cubes {
    pub fn set_color(&self, num: i32, color: &str) -> Self {
        match color {
            "red" => Cubes {
                red: num,
                blue: self.blue,
                green: self.green,
            },
            "blue" => Cubes {
                red: self.red,
                blue: num,
                green: self.green,
            },
            "green" => Cubes {
                red: self.red,
                blue: self.blue,
                green: num,
            },
            _ => panic!("Invalid property value {}", color),
        }
    }

    pub fn get_color(&self, color: &str) -> i32 {
      match color {
        "red" => self.red,
        "blue" => self.blue,
        "green" => self.green,
        _ => panic!("Invalid property value {}", color),
      }
    }

    pub fn get_power(&self) -> i32 {
      return self.red * self.blue * self.green;
    }
}

fn main() {
    println!("Evalutating games...");
    let mut sum: i32 = 0;
    let mut pwr_sum: i32 = 0;
    let mut path: std::path::PathBuf = env::current_dir().unwrap();
    path.push("src/input.txt");

    for line in read_to_string(path).unwrap().lines() {
        let valid_game_num: i32 = analyze_game(&line);
        let min_game_pwr: i32 = get_power_of_min(&line);

        sum += valid_game_num;
        pwr_sum += min_game_pwr;
    }

    println!("Sum of valid games: {}", sum);
    println!("Sum of the power of the minimum sets: {}", pwr_sum);
}

fn analyze_game(inp: &str) -> i32 {
    let game_num_as_str: Option<&str> = inp.split(":").next();
    let sets: Option<&str> = inp.split(":").last();
    let game_num: i32;
    let game_is_valid: bool;

    match sets {
        Some(d) => game_is_valid = verify_game(d.trim()),
        None => panic!("Could not find dice for game"),
    }

    match game_num_as_str {
        Some(n) => game_num = n.split(" ").last().unwrap().parse::<i32>().unwrap(),
        None => panic!("Could not get game num"),
    }

    if game_is_valid {
        return game_num;
    }

    return 0;
}

fn verify_game(game: &str) -> bool {
    for set in game.split(";") {
        let set_colors: str::Split<'_, &str> = set.split(",");
        let mut cube: Cubes = Cubes {
            red: 0,
            blue: 0,
            green: 0,
        };

        for dice in set_colors {
            let mut dice_split: str::Split<'_, &str> = dice.trim().split(" ");
            let dice_count: i32 = dice_split.next().unwrap().parse::<i32>().unwrap();
            let dice_color: &str = dice_split.next().unwrap();

            cube = cube.set_color(dice_count, dice_color);
        }

        if !set_is_valid(cube) {
            return false;
        }
    }

    return true;
}

fn set_is_valid(game_dice: Cubes) -> bool {
    const TOTAL_CUBES: Cubes = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    if TOTAL_CUBES.red < game_dice.red {
        return false;
    }

    if TOTAL_CUBES.green < game_dice.green {
        return false;
    }

    if TOTAL_CUBES.blue < game_dice.blue {
        return false;
    }

    return true;
}

fn get_power_of_min(inp: &str) -> i32 {
  let sets: Option<&str> = inp.split(":").last();
  let min_cube: Cubes;

  match sets {
    Some(d) => min_cube = find_min_dice(d.trim()),
    None => panic!("Could not find dice for game"),
  }

  return min_cube.get_power();
}

fn find_min_dice(game: &str) -> Cubes {
  let mut min_cube: Cubes = Cubes {
    red: 0,
    blue: 0,
    green: 0,
  };

  for set in game.split(";") {
    let set_colors: str::Split<'_, &str> = set.split(",");

    for dice in set_colors {
      let mut dice_split: str::Split<'_, &str> = dice.trim().split(" ");
      let dice_count: i32 = dice_split.next().unwrap().parse::<i32>().unwrap();
      let dice_color: &str = dice_split.next().unwrap();
      let current_color = min_cube.get_color(dice_color);

      if current_color < dice_count {
        min_cube = min_cube.set_color(dice_count, dice_color);
      }
    }
  }

  return min_cube;
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_CUBES: Cubes = Cubes {
        red: 3,
        green: 8,
        blue: 2,
    };

    #[test]
    fn verify_game_valid() {
        assert_eq!(
            true,
            verify_game("3 green, 10 red, 14 blue; 1 green, 9 red; 9 blue, 2 green, 12 red")
        )
    }

    #[test]
    fn verify_game_invalid() {
        assert_eq!(
            false,
            verify_game("3 green, 10 red, 15 blue; 1 green, 9 red; 9 blue, 2 green, 12 red")
        )
    }

    #[test]
    fn set_is_valid_valid() {
        assert_eq!(true, set_is_valid(MOCK_CUBES));
    }

    #[test]
    fn set_is_valid_invalid() {
        let mock_cubes_invalid: Cubes = Cubes {
            red: 20,
            green: 1,
            blue: 1,
        };

        assert_eq!(false, set_is_valid(mock_cubes_invalid));
    }

    #[test]
    fn find_min_dice_valid() {
      let expected: Cubes = Cubes {
        red: 2,
        blue: 5,
        green: 4
      };

      assert_eq!(expected, find_min_dice("3 blue, 4 green; 5 blue, 1 green, 2 red"));
    }

    #[test]
    fn get_power_of_min_valid() {
      assert_eq!(120, get_power_of_min("Game 5: 6 blue, 1 green; 2 blue, 4 red, 5 green"));
    }
}
