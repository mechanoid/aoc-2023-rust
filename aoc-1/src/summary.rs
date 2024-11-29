static NUMBER_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

enum ParsingDirection {
    LTR,
    RTL,
}

fn first_number_in_string(text_line: &String) -> Result<usize, String> {
    for char in text_line.chars() {
        if NUMBER_CHARS.contains(&char) {
            let number = (char.to_string()).parse::<usize>().unwrap();
            return Ok(number);
        }
    }

    return Err(format!("failed to find number in string: {}", text_line));
}

fn is_number_word(word: &str) -> Option<usize> {
    return match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    };
}

fn walk_from_left_to_right_and_replace(text: &str) -> String {
    let mut text = text.to_string();
    let mut leading = 0;

    while text.len() > 2 && leading < (text.len() - 2) {
        let mut trailing = leading + 3;
        while trailing < text.len() + 1 {
            let slice = &text[leading..trailing];
            // println!("slice {}", slice);
            if let Some(number) = is_number_word(slice) {
                text.replace_range(leading..trailing, number.to_string().as_str());
                return text;
            }

            trailing += 1;
        }
        leading += 1;
    }

    return text;
}

fn walk_from_right_to_left_and_replace(text: &str) -> String {
    let mut text = text.to_string();
    let mut leading = text.len();

    while text.len() > 2 && leading > 2 {
        let mut trailing = leading - 3;

        while trailing > 0 {
            let slice = &text[trailing..leading];
            // println!("slice {}", slice);
            if let Some(number) = is_number_word(slice) {
                text.replace_range(trailing..leading, number.to_string().as_str());
                return text;
            }

            trailing -= 1;
        }
        leading -= 1;
    }

    return text;
}

fn replace_number_words_with_direction(text: &str, direction: ParsingDirection) -> String {
    return match direction {
        ParsingDirection::LTR => walk_from_left_to_right_and_replace(&text),
        ParsingDirection::RTL => walk_from_right_to_left_and_replace(&text),
    };
}

fn replace_number_words(line: &str) -> String {
    return replace_number_words_with_direction(line, ParsingDirection::LTR);
}

fn replace_number_words_from_right_to_left(line: &str) -> String {
    return replace_number_words_with_direction(line, ParsingDirection::RTL);
}

fn parse_value_from_line(line: &str) -> Result<usize, &str> {
    if line == "" {
        return Ok(0);
    };

    let reversed = replace_number_words_from_right_to_left(&line);
    let reversed = reversed.chars().rev().collect::<String>();

    let line = replace_number_words(&line);

    let first_value = match first_number_in_string(&line) {
        Ok(i) => i,
        Err(e) => {
            println!("line with no first number: {}", line);
            panic!("no last number found: {}", e)
        }
    };

    let last_value = match first_number_in_string(&reversed) {
        Ok(i) => i,
        Err(e) => {
            println!("line with no last number: {}", line);
            panic!("no last number found: {}", e)
        }
    };

    let joined_number = format!("{}{}", first_value, last_value);
    let joined_number = joined_number.parse::<usize>().unwrap();

    return Ok(joined_number);
}

pub fn summarize(text: &str) -> usize {
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let mut sum = 0;

    for line in lines {
        if let Ok(line_value) = parse_value_from_line(line) {
            sum += line_value;
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_replacing_number_words() {
        let result = super::replace_number_words("two1nine");
        assert_eq!(result, "21nine");

        let result = super::replace_number_words("eightwothree");
        assert_eq!(result, "8wothree");

        let result = super::replace_number_words("abcone2threexyz");
        assert_eq!(result, "abc12threexyz");

        let result = super::replace_number_words("xtwone3four");
        assert_eq!(result, "x2ne3four");

        let result = super::replace_number_words("4nineeightseven2");
        assert_eq!(result, "49eightseven2");

        let result = super::replace_number_words("zoneight234");
        assert_eq!(result, "z1ight234");

        let result = super::replace_number_words("7pqrstsixteen");
        assert_eq!(result, "7pqrst6teen");

        let result = super::replace_number_words("sixeightfive3sdtwo");
        assert_eq!(result, "6eightfive3sdtwo");
    }

    #[test]
    fn test_value_from_line_parsing() {
        let result = super::parse_value_from_line("two1nine");
        assert_eq!(result, Ok(29));

        let result = super::parse_value_from_line("eightwothree");
        assert_eq!(result, Ok(83));

        let result = super::parse_value_from_line("abcone2threexyz");
        assert_eq!(result, Ok(13));

        let result = super::parse_value_from_line("xtwone3four");
        assert_eq!(result, Ok(24));

        let result = super::parse_value_from_line("4nineeightseven2");
        assert_eq!(result, Ok(42));

        let result = super::parse_value_from_line("zoneight234");
        assert_eq!(result, Ok(14));

        let result = super::parse_value_from_line("7pqrstsixteen");
        assert_eq!(result, Ok(76));
    }

    #[test]
    fn test_value_from_line_for_reversed_lines() {
        let result = super::replace_number_words_from_right_to_left("two1eightwo");
        assert_eq!(result, "two1eigh2");

        let result = super::parse_value_from_line("two1eightwo");
        assert_eq!(result, Ok(22));
    }

    #[test]
    fn test_summary() {
        let result = super::summarize(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        assert_eq!(result, 281);
    }

    #[test]
    fn test_blank_lines_are_ignored() {
        let result = super::summarize(
            "two1nine
eightwothree

abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
",
        );

        assert_eq!(result, 281);
    }
}
