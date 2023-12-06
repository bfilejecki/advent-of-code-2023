use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("File should exist");
    // let input = read_to_string("./test-input.txt").expect("File should exist");
    println!("Part one result is {}", part_one(&input));
    println!("Part two result is {}", part_two(&input));
}

fn part_one(input: &str) -> u64 {
    let time_dist = parse_input(&input);
    return count_possible_solutions(&time_dist);
}

fn part_two(input: &str) -> u64 {
    let input = vec![(35937366u64, 212206012011044u64 )];
    return count_possible_solutions(&input);
}

fn count_possible_solutions(time_dist: &Vec<(u64, u64)>) -> u64 {
    return time_dist
        .iter()
        .map(|(time, dist)| {
            let mut count = 0;
            (0..*time)
                .into_iter()
                .for_each(|t| {
                    let speed = *time - t;
                    let distance = speed * t;
                    if distance > *dist {
                        count = count + 1;
                    }
                });
            return count;
        })
        .product();
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines();

    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let dist: Vec<u64> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    return times
        .into_iter()
        .zip(dist.into_iter())
        .map(|(t, d)| (t, d))
        .collect();
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::parse_input;

    #[test]
    fn test_parse_input() {
        // Given
        let input = read_to_string("./input.txt").expect("File should exist");

        // When
        let result = parse_input(&input);

        // Then
        assert_eq!(
            result,
            vec![
                (35u64, 212u64),
                (93u64, 2060u64),
                (73u64, 1201u64),
                (66u64, 1044u64)
            ]
        );
    }
}
