use std::env;
use std::fs::read_to_string;
use std::str;

// forgot to commit part one to keep them separate :(
// below is only part 2
// part 2 answer: 54418

#[derive(Debug)]
struct IntAsWords {
    ind: usize,
    word: String,
}

fn main() {
    println!("Analyzing calibration document");

    let mut sum: i32 = 0;
    let mut path: std::path::PathBuf = env::current_dir().unwrap();
    path.push("src/input.txt");

    for line in read_to_string(path).unwrap().lines() {
        let int: i32 = get_calibration_integer(&line);

        sum += int;
    }

    println!("Sum of calibration values: {}", sum);
}

fn get_calibration_integer(inp: &str) -> i32 {
    let words: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let word_int_indices: Vec<IntAsWords> = get_int_word_indices(words, inp);
    let chars: str::CharIndices<'_> = inp.char_indices();
    let mut char_ints = chars.filter(|x: &(usize, char)| x.1.is_digit(10));
    let mut first_int: Option<(usize, char)> = char_ints.next();
    let first_int_ind: Option<usize>;
    let mut last_int: Option<(usize, char)> = char_ints.last();
    let last_int_ind: Option<usize>;

    match first_int {
        Some((ind, _char)) => first_int_ind = Some(ind),
        None => first_int_ind = Some(99 as usize),
    }

    match last_int {
        Some((ind, _char)) => last_int_ind = Some(ind),
        None => last_int_ind = Some(0 as usize),
    }

    if first_int.is_some() && last_int.is_none() && word_int_indices.len() > 0 {
        if first_int_ind.unwrap() < word_int_indices.first().unwrap().ind {
            let int_ind: usize = words
                .into_iter()
                .position(|x: &str| x == word_int_indices.last().unwrap().word)
                .unwrap()
                + 1;

            return format!("{}{}", first_int.unwrap().1, int_ind)
                .parse::<i32>()
                .unwrap();
        } else if first_int_ind.unwrap() > word_int_indices.last().unwrap().ind {
            let int_ind: usize = words
                .into_iter()
                .position(|x: &str| x == word_int_indices.first().unwrap().word)
                .unwrap()
                + 1;

            return format!("{}{}", int_ind, first_int.unwrap().1)
                .parse::<i32>()
                .unwrap();
        }
    }

    if word_int_indices.first().is_some() && first_int_ind.is_some() {
        if word_int_indices.first().unwrap().ind < first_int_ind.unwrap() {
            let int_ind: usize = words
                .into_iter()
                .position(|x: &str| x == word_int_indices.first().unwrap().word)
                .unwrap()
                + 1;
            first_int = Some((int_ind, char::from_digit(int_ind as u32, 10).unwrap()));
        }
    }

    if word_int_indices.last().is_some() && last_int_ind.is_some() {
        if word_int_indices.last().unwrap().ind > last_int_ind.unwrap() {
            let int_ind: usize = words
                .into_iter()
                .position(|x: &str| x == word_int_indices.last().unwrap().word)
                .unwrap()
                + 1;
            last_int = Some((int_ind, char::from_digit(int_ind as u32, 10).unwrap()));
        }
    }

    let mut int_as_string: String = String::from(first_int.unwrap().1);

    match last_int {
        Some((_ind, char)) => int_as_string.push_str(&char.to_string()),
        None => int_as_string.push_str(&first_int.unwrap().1.to_string()),
    }

    return int_as_string.parse::<i32>().unwrap();
}

fn get_int_word_indices(words: [&str; 9], inp: &str) -> Vec<IntAsWords> {
    let mut word_int_indices: Vec<IntAsWords> = vec![];

    words.iter().for_each(|word: &&str| {
        let ind: Vec<(usize, &str)> = inp.match_indices(word).collect();
        if ind.len() > 0 {
            ind.iter().for_each(|occurence: &(usize, &str)| {
                word_int_indices.push(IntAsWords {
                    ind: occurence.0,
                    word: occurence.1.to_string(),
                });
            });
        }
    });
    word_int_indices.sort_by(|a: &IntAsWords, b: &IntAsWords| a.ind.cmp(&b.ind));

    return word_int_indices;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inp_int() {
        assert_eq!(13, get_calibration_integer("a1dd4g3gs"));
    }

    #[test]
    fn inp_one_int() {
        assert_eq!(44, get_calibration_integer("abc4def"));
    }

    #[test]
    fn inp_int_as_string() {
        assert_eq!(24, get_calibration_integer("abctwoefgfour"));
    }

    #[test]
    fn inp_int_mixed() {
        assert_eq!(81, get_calibration_integer("8four419eighteight1bpv"));
    }

    #[test]
    fn inp_int_mixed_repeat_word() {
        assert_eq!(85, get_calibration_integer("84qxbnxdpqppjfiveeightfive"));
    }

    #[test]
    fn inp_one_int_as_string() {
        assert_eq!(22, get_calibration_integer("abctwoefg"));
    }

    #[test]
    fn inp_one_int_one_string() {
        assert_eq!(
            63,
            get_calibration_integer("rrzbgtfrrqkspsix3rkpzddzrbcrzvxzstjbqhmqq")
        )
    }

    #[test]
    fn inp_int_mixed_one_int_at_end() {
        assert_eq!(55, get_calibration_integer("bztngjjhfivethreenineeight5"))
    }

    #[test]
    fn inp_int_mixed_one_int_between_words() {
        assert_eq!(58, get_calibration_integer("bztngjjhfivethree7nineeight"))
    }
}
