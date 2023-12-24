use std::env;
use std::fs::read_to_string;

// part 1: 24542

fn main() {
    println!("Finding sum of scratchcards");
    let mut path: std::path::PathBuf = env::current_dir().unwrap();
    path.push("src/input.txt");
    let mut sum: i32 = 0;

    let binding: String = read_to_string(path).unwrap();
    for line in binding.lines() {
        let line_as_arr: Vec<&str> = line.split([':', '|']).collect();

        println!("Winning numbers: {}", line_as_arr[1].trim());
        println!("Card: {}", line_as_arr[2].trim());

        sum += get_sum_of_scratchcard(line_as_arr[1].trim(), line_as_arr[2].trim());
    }

    println!("Sum of scratchcard values: {}", sum);
}

fn get_sum_of_scratchcard(winning_nums: &str, card_nums: &str) -> i32 {
    let winning_as_arr: Vec<&str> = winning_nums.split(" ").collect();
    let card_as_arr: Vec<&str> = card_nums.split(" ").collect();
    let mut winning_count: u32 = 0;
    let base: i32 = 2;

    for num in card_as_arr {
        if !num.is_empty() && winning_as_arr.contains(&num) {
            winning_count += 1;
        }
    }

    if winning_count == 0 {
      return 0;
    }

    return base.pow(winning_count - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_sum_of_scratchcard_test() {
        let test_card: &str = "1 2  3 4";
        let test_winning_numbers: &str = "1 3";

        assert_eq!(2, get_sum_of_scratchcard(test_winning_numbers, test_card));
    }

    #[test]
    fn get_sum_of_scratchcard_no_winner() {
        let test_card: &str =
            "94 14 76 52 15 18 38 41 69 28 16 31 73 32 47 37 71 23 82 90 33 75 24 85 11";
        let test_winning_numbers: &str = "54 17 93 26 35  9 61 49 81 42";

        assert_eq!(0, get_sum_of_scratchcard(test_winning_numbers, test_card));
    }

    #[test]
    fn get_sum_of_scratchcard_winner() {
        let test_card: &str =
            "40 39  8 99 23 90  2 41 31 75 32 83  9 82 33 76 35 95  5 84 67  7 48 55 98";
        let test_winning_numbers: &str = "64 57 12  2 31 81 51 92 58  5";

        assert_eq!(4, get_sum_of_scratchcard(test_winning_numbers, test_card));
    }
}
