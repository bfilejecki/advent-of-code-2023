use std::{
    char,
    collections::{HashMap, HashSet},
    fs::{read_to_string, File},
    io::BufReader,
    usize,
};

fn main() {
    let input = read_to_string("./input.txt").expect("File should exist");
    println!("Part one result is {}", part_one(&input));
    println!("Part two result is {}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    let input_bytes = input.as_bytes();
    let line_length = calculate_line_length(input);
    let mut raw_part_number = String::from("");
    let mut found_adjacent_symbol = false;
    let mut sum = 0;
    for (i, symbol) in input.char_indices() {
        if symbol.is_digit(10) {
            raw_part_number.push(symbol);
            if !found_adjacent_symbol && has_adjacent_symbol(i, line_length, input_bytes) {
                found_adjacent_symbol = true;
            }
        } else if found_adjacent_symbol {
            sum = sum + raw_part_number.parse::<u32>().unwrap();
            found_adjacent_symbol = false;
            raw_part_number.clear();
        } else {
            raw_part_number.clear();
        }
    }

    return sum;
}

fn part_two(input: &str) -> u32 {
    let input_bytes = input.as_bytes();
    let line_length = calculate_line_length(input);
    let mut gears_map: HashMap<usize, HashSet<u32>> = HashMap::new();
    let mut raw_part_number = String::from("");
    let mut adjacent_gears = HashSet::new();
    let mut found_adjacent_gear = false;
    for (i, symbol) in input.char_indices() {
        if symbol.is_digit(10) {
            raw_part_number.push(symbol);

            get_adjacent_asterisks(i, line_length, input_bytes)
                .iter()
                .for_each(|g| {
                    adjacent_gears.insert(g.clone());
                });

            if !adjacent_gears.is_empty() {
                found_adjacent_gear = true;
            }
        } else if found_adjacent_gear {
            let part_number = raw_part_number.parse::<u32>().unwrap();

            adjacent_gears.iter().for_each(|i| {
                if let Some(set) = gears_map.get_mut(i) {
                    set.insert(part_number);
                } else {
                    let mut new_set = HashSet::new();
                    new_set.insert(part_number);
                    gears_map.insert(*i, new_set);
                }
            });

            found_adjacent_gear = false;
            raw_part_number.clear();
            adjacent_gears.clear();
        } else {
            raw_part_number.clear();
        }
    }

    return gears_map.iter()
        .map(|(_,v)| {
            if v.len() == 2 {
                v.iter().product()
            } else {
                0
            }
        }).sum();
}

fn calculate_line_length(input: &str) -> usize {
    return input.find("\n").unwrap() + 1;
}

fn is_valid_symbol(symbol: u8) -> bool {
    let c = symbol as char;
    return !c.is_whitespace() && c != '.' && !c.is_digit(10);
}

fn has_adjacent_symbol(index: usize, line_length: usize, bytes: &[u8]) -> bool {
    let adjacent_indices = get_adjacent_indices(index, line_length, bytes);
    return adjacent_indices.iter().any(|i| is_valid_symbol(bytes[*i]));
}

fn get_adjacent_asterisks(index: usize, line_length: usize, bytes: &[u8]) -> Vec<usize> {
    let adjacent_indices = get_adjacent_indices(index, line_length, bytes);
    return adjacent_indices
        .iter()
        .map(|i| (*i, bytes[*i]))
        .filter(|(i, c)| *c as char == '*')
        .map(|(i, c)| i)
        .collect();
}

fn get_adjacent_indices(index: usize, line_length: usize, bytes: &[u8]) -> Vec<usize> {
    let bytes_size = bytes.len();
    let mut adjacent_indices: Vec<usize> = Vec::new();
    index
        .checked_sub(1)
        .filter(|&i| i < bytes_size)
        .map(|i| adjacent_indices.push(i));
    index
        .checked_sub(line_length + 1)
        .filter(|&i| i < bytes_size)
        .map(|i| adjacent_indices.push(i));
    index
        .checked_sub(line_length)
        .filter(|&i| i < bytes_size)
        .map(|i| adjacent_indices.push(i));
    index
        .checked_sub(line_length - 1)
        .filter(|&i| i < bytes_size)
        .map(|i| adjacent_indices.push(i));
    index
        .checked_add(1)
        .filter(|&i| i < bytes_size)
        .map(|i| adjacent_indices.push(i));
    index
        .checked_add(line_length - 1)
        .filter(|&i| i < bytes_size)
        .map(|i| adjacent_indices.push(i));
    index
        .checked_add(line_length)
        .filter(|&i| i < bytes_size)
        .map(|i| adjacent_indices.push(i));
    index
        .checked_add(line_length + 1)
        .filter(|&i| i < bytes_size)
        .map(|i| adjacent_indices.push(i));

    return adjacent_indices;
}

#[cfg(test)]
mod tests {
    use crate::{calculate_line_length, has_adjacent_symbol, is_valid_symbol};

    #[test]
    fn test_calculate_line_length() {
        // Given
        let input = "...
";
        // When
        let result = calculate_line_length(input);

        // Then
        assert_eq!(result, 4);
    }

    #[test]
    fn test_is_valid_symbol() {
        // Expect
        assert_eq!(is_valid_symbol('\n' as u8), false);
        assert_eq!(is_valid_symbol('1' as u8), false);
        assert_eq!(is_valid_symbol('2' as u8), false);
        assert_eq!(is_valid_symbol('.' as u8), false);
        assert_eq!(is_valid_symbol('*' as u8), true);
    }

    #[test]
    fn test_has_adjacent_symbols() {
        // Given
        let input = "467..114..
...*......
..35..633."
            .as_bytes();

        // Expect
        assert_eq!(has_adjacent_symbol(2, 11, input), true);
        assert_eq!(has_adjacent_symbol(0, 11, input), false);
        assert_eq!(has_adjacent_symbol(1, 11, input), false);
    }
}
